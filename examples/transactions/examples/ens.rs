/// Neo N3 Name Service (NNS) Example
///
/// This example demonstrates how Neo Name Service works on Neo N3,
/// similar to ENS on Ethereum but for the Neo ecosystem.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🌐 Neo Name Service (NNS) Example");
	println!("=================================");

	println!("\n📚 Understanding Neo Name Service:");
	println!("   • Human-readable names for Neo addresses");
	println!("   • Domains end with .neo (e.g., alice.neo)");
	println!("   • Supports subdomains (e.g., wallet.alice.neo)");
	println!("   • Can store multiple record types");
	println!("   • Decentralized naming system");

	println!("\n🔧 NNS Contract Features:");
	println!("   • Register domains");
	println!("   • Set resolver records");
	println!("   • Transfer ownership");
	println!("   • Renew domains");
	println!("   • Query domain information");

	println!("\n📋 Record Types:");
	println!("   • A - Neo N3 address");
	println!("   • CNAME - Canonical name");
	println!("   • TXT - Text records");
	println!("   • AAAA - IPv6 address");
	println!("   • Custom types for dApps");

	println!("\n💡 Domain Resolution Process:");
	println!("   1. Query NNS contract with domain name");
	println!("   2. Get resolver contract for domain");
	println!("   3. Query resolver for record type");
	println!("   4. Receive resolved address or data");

	println!("\n📦 Example Domain Resolution:");
	println!("   Domain: payments.neo");
	println!("   Resolves to: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc");
	println!("   Owner: NikhQp1aAD1YFCiwknhM5LQQebj4464bCJ");
	println!("   Expiry: Block 2,500,000");

	println!("\n⚙️ Registration Process:");
	println!("   1. Check domain availability");
	println!("   2. Calculate registration fee");
	println!("   3. Submit registration transaction");
	println!("   4. Set resolver records");
	println!("   5. Configure subdomain settings");

	println!("\n💰 Pricing Structure:");
	println!("   • Shorter names cost more GAS");
	println!("   • 1-2 characters: Premium pricing");
	println!("   • 3-4 characters: Higher fees");
	println!("   • 5+ characters: Standard fees");
	println!("   • Annual renewal required");

	println!("\n🔐 Domain Management:");
	println!("   • Transfer ownership to another address");
	println!("   • Update resolver records");
	println!("   • Add/remove subdomains");
	println!("   • Set admin permissions");
	println!("   • Renew before expiration");

	println!("\n🎯 Use Cases:");
	println!("   • Simplified payment addresses");
	println!("   • dApp identity management");
	println!("   • Decentralized websites");
	println!("   • Smart contract aliases");
	println!("   • Cross-chain identity");

	println!("\n💡 Integration Example:");
	println!("   // Instead of:");
	println!("   send_to(\"NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc\")");
	println!("   ");
	println!("   // You can use:");
	println!("   send_to(resolve_nns(\"alice.neo\"))");

	println!("\n⚠️ Important Considerations:");
	println!("   • Always verify resolved addresses");
	println!("   • Check domain expiration status");
	println!("   • Handle resolution failures gracefully");
	println!("   • Cache results for performance");
	println!("   • Be aware of domain squatting");

	println!("\n🚀 For working examples, see:");
	println!("   • examples/neo_nns/");
	println!("   • NNS contract documentation");
	println!("   • Neo N3 naming standards");

	Ok(())
}
