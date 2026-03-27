# 🏢 Real Estate Tokenization Smart Contract (Soroban)

## 📌 Project Description

This project is a **smart contract built using Soroban on the Stellar blockchain** that enables **real estate tokenization**. It allows property ownership to be divided into smaller digital shares (tokens), making real estate investment more accessible and flexible.

By leveraging blockchain technology, this system ensures **transparency, security, and decentralization** in property ownership and transfer.

---

## ⚙️ What It Does

* Creates tokenized representations of real estate properties
* Assigns ownership tokens to property creators
* Enables secure transfer of property shares between users
* Maintains user balances for each property on-chain

---

## ✨ Features

### 🏗️ Property Creation

* Create a new property using a unique property ID
* Define total tokens representing ownership shares
* Automatically assigns all tokens to the property owner

### 🔁 Token Transfer

* Transfer fractional ownership (tokens) between users
* Requires authentication for secure transactions
* Prevents transfers when balance is insufficient

### 📊 Balance Tracking

* Check token balance for any user and property
* Stores all balances on-chain for transparency

### 🔐 Security

* Uses Soroban’s built-in authentication (`require_auth`)
* Ensures only authorized users can perform actions

---

## 🧱 Smart Contract Structure

* `Property` → Stores property details (ID, owner, total tokens)
* `DataKey` → Handles storage keys for properties and balances
* `create_property()` → Creates and initializes a property
* `transfer()` → Transfers tokens between users
* `balance()` → Returns user token balance

---

## 🚀 How to Build & Run

### 1. Install Dependencies

* Rust
* Soroban CLI

### 2. Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Deploy (Example)

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm \
  --source YOUR_ACCOUNT
```

---
### Contract Address : CDGBAFRS5U2FXZSMDVZ7ZBCUVDDKYIT4EJHVQC5PYNOUC3HECSH25MU4
![WhatsApp Image 2026-03-27 at 13 36 44](https://github.com/user-attachments/assets/c968552b-ebd1-4d6d-9065-f18ecdedd584)

## 📌 Use Cases

* Fractional real estate investment
* Blockchain-based property ownership
* Real estate crowdfunding platforms

---

## 🔮 Future Improvements

* Add property metadata (location, valuation, documents)
* NFT-based ownership certificates
* Marketplace for buying/selling tokens
* Rental income distribution system
* KYC/AML integration

---

## 🧰 Tech Stack

* **Stellar Soroban**
* **Rust**
* **Blockchain Smart Contracts**

---

## 📄 License

This project is licensed under the MIT License.
