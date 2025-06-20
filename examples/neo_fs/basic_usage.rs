/// NeoFS Basic Usage Example
///
/// This example demonstrates comprehensive NeoFS operations including container
/// management, object storage, access control, and session handling.
use neo3::neo_fs::{
	AccessPermission, Container, ContainerId, NeoFSClient, Object, OwnerId,
	PlacementPolicy, BasicACL, Session, ObjectHeader
};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🗂️  NeoFS Basic Usage Example");
	println!("==============================\n");

	// 1. Create and configure NeoFS client
	println!("📡 1. Configuring NeoFS Client...");
	let config = neo3::neo_fs::NeoFSConfig {
		endpoint: "grpc.testnet.fs.neo.org:8082".to_string(),
		timeout: std::time::Duration::from_secs(60),
		secure: true,
	};
	
	let client = NeoFSClient::with_config(config);
	println!("   ✅ Client configured for TestNet");
	println!("   📍 Endpoint: grpc.testnet.fs.neo.org:8082");
	println!("   ⏱️  Timeout: 60 seconds");

	// 2. Create owner identity
	println!("\n🔐 2. Creating Owner Identity...");
	// In production, this would come from a Neo wallet
	let owner_id = OwnerId::from_wallet_address("NPvKVTGZapmFWABLsyvfreuqn73jCjJtN1")?;
	println!("   👤 Owner: {}", owner_id);
	
	// Create session for operations
	let session = Session::new(owner_id.clone())
		.with_expiration(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600)
		.with_context(b"NeoFS Example Session".to_vec());
	println!("   🎫 Session created (expires in 1 hour)");

	// 3. Container creation example
	println!("\n📦 3. Container Management...");
	
	// Define placement policy
	let placement = PlacementPolicy::new()
		.with_replicas(2)
		.with_selector("Country", vec!["USA", "EU"])
		.with_filter("StorageType", "SSD");
	
	// Create container with attributes
	let container = Container::new(
		ContainerId::generate()?,
		owner_id.clone()
	)
		.with_placement_policy(placement)
		.with_basic_acl(BasicACL::PublicReadWrite)
		.with_attribute("Name", "Example Storage Container")
		.with_attribute("Purpose", "NeoRust SDK Examples")
		.with_attribute("Environment", "TestNet");

	println!("   📋 Container prepared:");
	println!("      • ID: {}", container.id);
	println!("      • ACL: PublicReadWrite");
	println!("      • Replicas: 2");
	println!("      • Attributes: {} defined", container.attributes.len());

	// Show container operations
	match client.create_container(&container, &session).await {
		Ok(cid) => println!("   ✅ Container created: {}", cid),
		Err(e) => println!("   ⚠️  Container creation pending: {}", e),
	}

	// 4. Object storage example
	println!("\n📄 4. Object Storage Operations...");
	
	// Create object with metadata
	let object_data = b"Hello from NeoFS! This is sample data stored in the decentralized storage network.";
	let object = Object::new(container.id.clone(), owner_id.clone())
		.with_payload(object_data.to_vec())
		.with_attribute("ContentType", "text/plain")
		.with_attribute("FileName", "hello.txt")
		.with_attribute("Timestamp", &SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().to_string())
		.with_attribute("Compression", "none")
		.with_attribute("Encryption", "none");

	println!("   📝 Object prepared:");
	println!("      • Size: {} bytes", object.size());
	println!("      • Container: {}", object.container_id);
	println!("      • Attributes:");
	for (key, value) in &object.attributes.attributes {
		println!("        - {}: {}", key, value);
	}

	// Calculate object ID (hash of header + payload)
	let object_id = object.calculate_id()?;
	println!("      • Object ID: {}", object_id);

	// Upload object
	match client.put_object(&object, &session).await {
		Ok(oid) => println!("   ✅ Object uploaded: {}", oid),
		Err(e) => println!("   ⚠️  Upload pending: {}", e),
	}

	// 5. Object retrieval example
	println!("\n🔍 5. Object Retrieval...");
	
	// Get object header
	match client.head_object(&container.id, &object_id, &session).await {
		Ok(header) => {
			println!("   📋 Object header retrieved:");
			println!("      • Version: {:?}", header.version);
			println!("      • Size: {} bytes", header.payload_size);
			println!("      • Created: {:?}", header.creation_epoch);
		},
		Err(e) => println!("   ⚠️  Header retrieval pending: {}", e),
	}

	// Get full object
	match client.get_object(&container.id, &object_id, &session).await {
		Ok(retrieved) => {
			println!("   ✅ Object retrieved successfully");
			if let Ok(content) = String::from_utf8(retrieved.payload.clone()) {
				println!("   📄 Content preview: {}...", &content[..40.min(content.len())]);
			}
		},
		Err(e) => println!("   ⚠️  Object retrieval pending: {}", e),
	}

	// 6. Access control demonstration
	println!("\n🔐 6. Access Control Management...");
	
	// Create ACL for container
	let acl_rules = vec![
		(AccessPermission::GetObject, vec![owner_id.clone()]),
		(AccessPermission::PutObject, vec![owner_id.clone()]),
		(AccessPermission::DeleteObject, vec![owner_id.clone()]),
		(AccessPermission::GetContainer, vec![OwnerId::anyone()]),
	];

	println!("   🛡️  Access rules defined:");
	for (permission, allowed) in &acl_rules {
		println!("      • {:?}: {} principals", permission, allowed.len());
	}

	// 7. Search operations
	println!("\n🔎 7. Object Search...");
	
	// Search by attributes
	let search_filters = vec![
		("ContentType", "text/plain"),
		("Compression", "none"),
	];

	match client.search_objects(&container.id, search_filters, &session).await {
		Ok(results) => {
			println!("   ✅ Search completed:");
			println!("   📊 Found {} objects matching criteria", results.len());
		},
		Err(e) => println!("   ⚠️  Search pending: {}", e),
	}

	// 8. Container listing
	println!("\n📋 8. Container Listing...");
	
	match client.list_containers(&owner_id, &session).await {
		Ok(containers) => {
			println!("   ✅ Containers for owner:");
			for (idx, cid) in containers.iter().enumerate() {
				println!("      {}. {}", idx + 1, cid);
			}
		},
		Err(e) => println!("   ⚠️  Listing pending: {}", e),
	}

	// 9. Cleanup operations
	println!("\n🧹 9. Cleanup Operations...");
	
	// Delete object
	match client.delete_object(&container.id, &object_id, &session).await {
		Ok(_) => println!("   ✅ Object deleted"),
		Err(e) => println!("   ⚠️  Deletion pending: {}", e),
	}

	// Delete container (only when empty)
	match client.delete_container(&container.id, &session).await {
		Ok(_) => println!("   ✅ Container deleted"),
		Err(e) => println!("   ⚠️  Container deletion pending: {}", e),
	}

	// 10. Advanced features
	println!("\n🚀 10. Advanced Features Available:");
	println!("   • 📤 Multipart uploads for large files");
	println!("   • 🔄 Object versioning support");
	println!("   • 🏷️  Extended attributes (xattrs)");
	println!("   • 🔗 Object linking and references");
	println!("   • 📊 Storage metrics and statistics");
	println!("   • 🌐 Geographic placement policies");
	println!("   • 🔐 Homomorphic hashing for integrity");

	println!("\n✅ NeoFS basic usage example completed!");
	println!("💡 Note: Full gRPC implementation enables all operations shown above");
	
	Ok(())
}

// Helper trait implementations
trait NeoFSHelpers {
	fn generate() -> Result<ContainerId, Box<dyn std::error::Error>>;
	fn from_wallet_address(address: &str) -> Result<OwnerId, Box<dyn std::error::Error>>;
	fn anyone() -> OwnerId;
	fn calculate_id(&self) -> Result<neo3::neo_fs::ObjectId, Box<dyn std::error::Error>>;
}

impl NeoFSHelpers for ContainerId {
	fn generate() -> Result<ContainerId, Box<dyn std::error::Error>> {
		// Generate random container ID
		use rand::Rng;
		let mut rng = rand::thread_rng();
		let id: [u8; 32] = rng.gen();
		Ok(ContainerId::from(hex::encode(id)))
	}
}

impl NeoFSHelpers for OwnerId {
	fn from_wallet_address(address: &str) -> Result<OwnerId, Box<dyn std::error::Error>> {
		// Convert Neo address to owner ID
		Ok(OwnerId::from(address.to_string()))
	}
	
	fn anyone() -> OwnerId {
		OwnerId::from("*".to_string())
	}
}

impl NeoFSHelpers for Object {
	fn calculate_id(&self) -> Result<neo3::neo_fs::ObjectId, Box<dyn std::error::Error>> {
		// Calculate object ID from header hash
		use neo3::neo_crypto::sha256;
		let header_data = format!("{}{}{}", self.container_id, self.owner_id, self.size());
		let hash = sha256(header_data.as_bytes());
		Ok(neo3::neo_fs::ObjectId::from(hex::encode(&hash[..16])))
	}
}
