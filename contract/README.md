
# Domain Marketplace on Stellar Soroban

## 🚀 Project Description

This project is a decentralized Domain Marketplace built using Soroban smart contracts on the Stellar blockchain. It allows users to register, buy, and sell domain names in a trustless and transparent environment.

The system eliminates intermediaries and ensures ownership is verifiable on-chain.

---

## ⚙️ What It Does

- Users can register unique domain names
- Domain owners can list domains for sale
- Buyers can purchase domains directly
- Ownership is securely stored on-chain

---

## ✨ Features

### 1. Domain Registration
- Register unique domain names
- Prevent duplicate registrations

### 2. Buy Domains
- Transfer ownership securely
- Ensures domain is available for sale

### 3. Sell Domains
- Owners can list domains with price
- Update price anytime

### 4. Ownership Tracking
- Blockchain-based ownership
- Transparent and immutable records

---

## 🧱 Tech Stack

- Soroban SDK (Rust)
- Stellar Blockchain

---

## 📦 Project Structure

```
├── src/
│   └── lib.rs   # Smart contract
├── Cargo.toml
└── README.md
```

---

## 🛠️ Setup Instructions

### Prerequisites
- Rust installed
- Soroban CLI installed

### Install Soroban CLI
```
cargo install soroban-cli
```

### Build Contract
```
cargo build --target wasm32-unknown-unknown --release
```

### Deploy Contract
```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/domain_marketplace.wasm
```

---

## 🔐 Future Improvements

- Add payment handling using Stellar tokens
- Auction-based domain selling
- Domain expiration system
- ENS-like name resolution

---

## 📜 License

MIT License
*/
