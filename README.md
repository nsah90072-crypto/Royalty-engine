# 💸 Royalty Engine – Soroban Smart Contract

## 📖 Project Description

**Royalty Engine** is a smart contract built using Soroban on the Stellar network that enables automatic royalty management for digital assets.

It allows creators to register their assets and define a royalty percentage that is enforced on every transaction. This ensures fair and transparent compensation for creators across secondary sales.

---

## ⚙️ What It Does

The contract provides a simple and efficient royalty system:

* Register digital assets with a creator address
* Assign a royalty percentage to each asset
* Store royalty data securely on-chain
* Calculate royalty amounts based on sale price

---

## ✨ Features

* 🔐 **Decentralized Royalty Storage**
  All royalty data is stored securely on-chain using Soroban

* 📊 **Automated Royalty Calculation**
  Instantly calculates creator earnings from any sale

* 👤 **Creator Ownership Mapping**
  Each asset is permanently linked to its creator

* ⚡ **Gas Efficient Design**
  Lightweight implementation for low-cost execution

* 🔁 **Composable Architecture**
  Easily integrates with NFT marketplaces and dApps

---

## 🔗 Deployed Smart Contract Link

**Deployer Address:**
CAMTXA2GPU2G3BQPAA4P57343YCR2566HPP7ZDYD2XI6NX7MXFKSCRHF
https://stellar.expert/explorer/testnet/contract/CAMTXA2GPU2G3BQPAA4P57343YCR2566HPP7ZDYD2XI6NX7MXFKSCRHF

> ⚠️ Note: This is the deployer (wallet) address.
> After deployment, replace this section with your actual **contract ID**.

You can explore deployments here:

* https://stellar.expert
* https://soroban.stellar.org

---

## 🚀 Getting Started

### Prerequisites

* Rust
* Soroban CLI
* Stellar Account

---

### 🔨 Build the Contract

```bash
soroban contract build
```

---

### 🚀 Deploy the Contract

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/royalty_engine.wasm \
  --source CAMTXA2GPU2G3BQPAA4P57343YCR2566HPP7ZDYD2XI6NX7MXFKSCRHF
```

---

## 🧪 Example Usage

### Register an Asset

```bash
soroban contract invoke \
  --id <contract-id> \
  --fn register_asset \
  --arg asset_id=ART1 \
  --arg creator=CAMTXA2GPU2G3BQPAA4P57343YCR2566HPP7ZDYD2XI6NX7MXFKSCRHF \
  --arg percentage=10
```

---

### Calculate Royalty

```bash
soroban contract invoke \
  --id <contract-id> \
  --fn calculate_royalty \
  --arg asset_id=ART1 \
  --arg sale_price=1000
```

---

## 🛠 Future Improvements

* Multi-recipient royalty distribution
* Automatic payout transfers
* NFT standard compatibility (ERC-2981-like for Soroban)
* Frontend dashboard for creators

---

## 📄 License

MIT License
