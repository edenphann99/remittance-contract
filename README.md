# Remittance Contract

## Problem
Many Vietnamese families receive money from relatives working abroad, but traditional remittance services are expensive, slow, and lack transparent tracking.

## Solution
Remittance Contract is a decentralized remittance application built on Stellar Soroban that allows overseas users to create transfer records and enables recipients in Vietnam to securely claim funds with transparent on-chain transaction tracking.

## Why Stellar
Stellar provides low-cost, fast, and transparent transactions, while Soroban smart contracts enable secure and programmable remittance workflows.

## Target User
- Vietnamese families receiving money from relatives overseas.
- Overseas workers sending money home.
- Small remittance businesses looking for transparent payment solutions.

## Live Demo

- **Network:** Stellar Testnet

- **Contract ID:**

```text
CDMISVENBJE5WSXPOXGYNREZY3UQUF44TTHJO7B3MB2ELOZ4JHVOCWOR
```

- **Deployment Transaction**

https://stellar.expert/explorer/testnet/tx/1fed9fdf7c79d9494a5c4a593692381634ae819bd76beffd702cc04484daaddf

- **Contract Explorer**

https://lab.stellar.org/r/testnet/contract/CDMISVENBJE5WSXPOXGYNREZY3UQUF44TTHJO7B3MB2ELOZ4JHVOCWOR

---

## Features

- ✅ Multi-wallet support
- ✅ Create remittance transfers
- ✅ Claim transfers by recipient
- ✅ Persistent storage
- ✅ Event emission (`create`, `claim`)
- ✅ Transfer status management
- ✅ Transaction history through IDs
- ✅ Built and deployed on Stellar Testnet

---

## Smart Contract Functions

### create_transfer()

Creates a new remittance transaction.

Parameters:

- `sender: Address`
- `receiver: Address`
- `amount: i128`

Returns:

```rust
u64 // transfer ID
```

---

### claim_transfer()

Allows the receiver to claim a pending transfer.

Parameters:

- `transfer_id: u64`
- `receiver: Address`

Changes status:

```text
Pending → Claimed
```

---

### get_transfer()

Retrieves transfer information.

Parameters:

```rust
transfer_id: u64
```

Returns:

```rust
Transfer
```

---

### get_counter()

Returns total number of transfers.

Returns:

```rust
u64
```

---

## Architecture

```text
Sender (Overseas)
        │
        ▼
create_transfer()
        │
        ▼
Soroban Smart Contract
        │
        ▼
Transfer Stored On-chain
        │
        ▼
Recipient (Vietnam)
        │
        ▼
claim_transfer()
        │
        ▼
Status = Claimed
```

---

## Data Structure

```rust
pub struct Transfer {
    pub id: u64,
    pub sender: Address,
    pub receiver: Address,
    pub amount: i128,
    pub status: TransferStatus,
}
```

Transfer status:

```rust
pub enum TransferStatus {
    Pending,
    Claimed,
}
```

---

## How to Run

### Clone repository

```bash
git clone https://github.com/yourname/remittance-contract.git
cd remittance-contract
```

### Build contract

```bash
cargo build --target wasm32v1-none --release
```

### Run tests

```bash
cargo test
```

### Deploy to Testnet

```bash
stellar contract deploy \
--wasm target/wasm32v1-none/release/remittance_contract.wasm \
--source alice \
--network testnet
```

---

## Example Usage

### Create transfer

```bash
stellar contract invoke \
--id CDMISVENBJE5WSXPOXGYNREZY3UQUF44TTHJO7B3MB2ELOZ4JHVOCWOR \
--source alice \
--network testnet \
-- create_transfer \
--sender GBDAYBIKL45EUDFEP5MMNSRXR4W6B7M4AKTS5WMQECXN4ST4QGOWNFGP \
--receiver GBDAYBIKL45EUDFEP5MMNSRXR4W6B7M4AKTS5WMQECXN4ST4QGOWNFGP \
--amount 1000
```

Output:

```text
2
```

---

### Claim transfer

```bash
stellar contract invoke \
--id CDMISVENBJE5WSXPOXGYNREZY3UQUF44TTHJO7B3MB2ELOZ4JHVOCWOR \
--source alice \
--network testnet \
-- claim_transfer \
--transfer_id 2 \
--receiver GBDAYBIKL45EUDFEP5MMNSRXR4W6B7M4AKTS5WMQECXN4ST4QGOWNFGP
```

---

## Tech Stack

- Smart Contract: Rust + Soroban SDK v22
- Blockchain: Stellar Testnet
- Contract Framework: Soroban
- Wallet: Stellar CLI identities
- Storage: Persistent Storage
- Event System: Soroban Events

---

## Future Improvements

- PIN-protected remittance claim
- Expiration time for transfers
- Cancel transfer function
- Admin fee mechanism
- Transfer history per wallet
- USDC integration
- React frontend + Freighter wallet
- Mobile-friendly UI

---

## Team

- Name : Eden
- Email: Edenphann@gmail.com

---

Built with ❤️ using Stellar Soroban.
