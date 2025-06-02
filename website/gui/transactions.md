# Transactions

Learn how to send, receive, and manage transactions using the NeoRust Desktop GUI.

## Transaction Overview

The Transactions section provides a comprehensive interface for managing all your Neo N3 transactions, including:

- **Send tokens** (NEO, GAS, and custom NEP-17 tokens)
- **Receive payments** with QR codes and address sharing
- **Transaction history** with detailed information
- **Multi-signature transactions** for enhanced security

## Sending Transactions

### Send NEO/GAS Tokens

1. **Navigate to Send**
   - Click on "Send" in the main navigation
   - Or use the quick send button from the dashboard

2. **Select Token Type**
   - Choose from available tokens: NEO, GAS, or custom tokens
   - View your current balance for the selected token

3. **Enter Transaction Details**
   - **Recipient Address**: Enter or paste the recipient's Neo address
   - **Amount**: Specify the amount to send
   - **Network Fee**: Review and adjust the network fee if needed

4. **Review and Confirm**
   - Double-check all transaction details
   - Click "Send Transaction"
   - Enter your wallet password to authorize

5. **Transaction Broadcast**
   - The transaction will be broadcast to the network
   - You'll receive a transaction hash for tracking
   - Monitor the status in the transaction history

### Send Custom NEP-17 Tokens

1. **Add Custom Token** (if not already added)
   - Go to "Tokens" section
   - Click "Add Token"
   - Enter the contract hash
   - Verify token details

2. **Send Custom Token**
   - Select the custom token from the dropdown
   - Follow the same steps as sending NEO/GAS
   - Ensure recipient supports the token

## Receiving Transactions

### Generate Receiving Address

1. **Navigate to Receive**
   - Click "Receive" in the main navigation
   - Your wallet address will be displayed

2. **QR Code Generation**
   - A QR code is automatically generated
   - Share this QR code for easy payments
   - Include amount and token type in QR code

3. **Address Sharing**
   - Copy your address to clipboard
   - Share via messaging or email
   - Verify address accuracy before sharing

### Payment Requests

1. **Create Payment Request**
   - Specify the amount and token type
   - Add an optional message or description
   - Generate a shareable payment link

2. **Monitor Incoming Payments**
   - Watch for incoming transactions
   - Receive notifications when payments arrive
   - View confirmation status

## Transaction History

### View Transaction History

1. **Access History**
   - Click "Transactions" in the navigation
   - View all incoming and outgoing transactions
   - Filter by date, type, or status

2. **Transaction Details**
   - Click on any transaction for details
   - View transaction hash, block height, timestamp
   - See gas fees and confirmation status

3. **Search and Filter**
   - Search by transaction hash or address
   - Filter by transaction type (sent/received)
   - Filter by date range or token type

### Transaction Status

- **⏳ Pending**: Transaction submitted but not confirmed
- **✅ Confirmed**: Transaction included in a block
- **❌ Failed**: Transaction failed to execute
- **🔄 Replaced**: Transaction replaced by another

## Advanced Transaction Features

### Multi-Signature Transactions

1. **Setup Multi-Sig Wallet**
   - Import or create multi-signature wallet
   - Configure required signature threshold
   - Add co-signer public keys

2. **Create Multi-Sig Transaction**
   - Initiate transaction as usual
   - Transaction enters "Pending Signatures" state
   - Share transaction details with co-signers

3. **Sign Multi-Sig Transaction**
   - Co-signers can view pending transactions
   - Provide signatures using their private keys
   - Transaction broadcasts when threshold is met

### Batch Transactions

1. **Create Batch Transaction**
   - Select multiple recipients
   - Specify amounts for each recipient
   - Review total amount and fees

2. **Execute Batch**
   - All transfers execute in a single transaction
   - Reduces overall network fees
   - Atomic execution (all or nothing)

## Transaction Settings

### Gas Fee Configuration

1. **Fee Estimation**
   - View estimated transaction fees
   - Understand fee calculation factors
   - Choose between slow/standard/fast processing

2. **Custom Fee Setting**
   - Set custom gas price and limit
   - Advanced users can optimize fees
   - Higher fees = faster confirmation

### Transaction Notifications

1. **Enable Notifications**
   - Turn on desktop notifications
   - Get alerts for incoming payments
   - Monitor transaction confirmations

2. **Notification Settings**
   - Customize notification types
   - Set minimum amount thresholds
   - Configure sound alerts

## Troubleshooting

### Common Issues

#### Transaction Not Confirming
- **Check network congestion**: High traffic can delay confirmations
- **Verify gas fee**: Low fees may cause delays
- **Wait for network**: Typically confirms within 15-30 seconds

#### Insufficient Balance
- **Check token balance**: Ensure sufficient funds
- **Account for fees**: Reserve GAS for transaction fees
- **Verify token type**: Confirm you're sending the right token

#### Invalid Address
- **Verify address format**: Neo addresses start with 'N'
- **Check address length**: Should be 34 characters
- **Use address validation**: GUI validates addresses automatically

### Error Messages

- **"Insufficient funds"**: Not enough balance to cover amount + fees
- **"Invalid address"**: Recipient address format is incorrect
- **"Network error"**: Connection issues with Neo network
- **"Transaction timeout"**: Transaction failed to broadcast

## Security Best Practices

### Transaction Security

1. **Verify Recipients**
   - Double-check recipient addresses
   - Use trusted address book entries
   - Verify amounts before sending

2. **Secure Your Wallet**
   - Use strong passwords
   - Enable two-factor authentication if available
   - Keep wallet software updated

3. **Network Safety**
   - Only use official network endpoints
   - Verify SSL certificates
   - Be cautious on public WiFi

### Recovery Procedures

1. **Transaction Recovery**
   - Keep transaction hashes for records
   - Use blockchain explorers to track status
   - Contact support for stuck transactions

2. **Wallet Recovery**
   - Maintain secure backups
   - Test recovery procedures
   - Store recovery phrases securely

## Integration with Other Features

### NFT Transactions

- Transfer NFTs using the same interface
- View NFT transaction history
- Batch NFT operations

### DeFi Integration

- Interact with DeFi protocols
- Stake tokens directly from wallet
- Provide liquidity to AMMs

### Cross-Chain Operations

- Bridge tokens to other networks
- Monitor cross-chain transaction status
- Handle wrapped token operations

## Getting Help

If you encounter issues with transactions:

1. **Check the FAQ** in the help section
2. **View transaction on explorer** for detailed status
3. **Contact support** through the help menu
4. **Join community discussions** for peer assistance

Remember: Always verify transaction details carefully before confirming. Blockchain transactions are irreversible! 