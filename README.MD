# Transaction Analyzer

**The Transaction Analyzer** is an open-source tool for analyzing blockchain transactions in Web3.  
Integrated into the **Cometix Security Web Extension**, it leverages artificial intelligence (AI)  
to examine transactions in real time on Ethereum and Solana blockchains. The tool  
detects anomalies, such as unauthorized transfers or exploit-linked addresses,  
providing instant warnings to help users and developers maintain secure  
decentralized interactions.

## Key Features

- **Real-Time Transaction Scanning** *(Open Source)*  
  Examines blockchain transactions as they occur, identifying risks like suspicious  
  transfers, with immediate notifications.

- **Anomaly Detection** *(Open Source)*  
  Uses AI to detect irregular patterns, such as hidden fees or scam-related addresses,  
  alerting users to potential threats.

- **Exploit Address Tracking** *(Open Source)*  
  Identifies wallet addresses tied to known exploits or fraud, generating warnings  
  for safe transaction decisions.

- **Transaction Pattern Analysis** *(Open Source)*  
  Analyzes transaction behaviors, like gas usage or function calls, delivering detailed  
  reports for secure operations.

- **Risk Warning System**  
  Provides proprietary AI-driven alerts for high-risk transactions, ensuring  
  proactive protection against threats.

- **Real-Time Diagnostics**  
  Offers instant AI-based insights into transaction details, flagging issues  
  like excessive gas for quick response.

- **Security Report Dashboard**  
  Compiles analysis results into a dashboard, presenting risk summaries and  
  mitigation steps to support Web3 safety.

---

## Architecture

```
                  ┌────────────────────────────┐
                  │    Cometix Web Extension   │
                  └────────────┬───────────────┘
                               │
        ┌──────────────────────▼──────────────────────┐
        │    Transaction Analyzer (Rust + WASM)       │
        │  - Scanner Engine                           │
        │  - Exploit Address Module                   │
        │  - Pattern Analyzer                         │
        │  - Risk Alert System                        │
        │  - AI Engine                                │
        └──────────────────────┬──────────────────────┘
                               │
              ┌────────────────▼─────────────────┐
              │      Ethereum / Solana RPC       │
              └──────────────────────────────────┘
```

---

### Requirements

| Tool                  | Version      |
|-----------------------|--------------|
| `cometix-sdk`         | `1.0.16`     |
| `solana-instruction`  | `>=2.2.1`    |
| `eth-instruction`     | `>=3.143.1`  |
| `cometix-security`    | `1.0.52`     |
| `cometix-utils`       | `1.1.12`     |
| `wasm-pack`           | `>=0.10.2`   |
| `wasm-pack`           | `>=0.10.2`   |
| `Rust`                | `>=1.70.0`   |
| `Node.js`             | `>=18.0.0`   |
| `Vite`                | `>=4.0.0`    |

---

## Local Development

Install Rust toolchain:

```bash
curl https://sh.rustup.rs -sSf | sh
cargo install wasm-pack
```

Install frontend deps:

```bash
cd frontend/
npm install
```

## Building the WASM Module

```bash
cd transaction-analyzer/
wasm-pack build --target web --out-dir ../output/out
```

Flags:
- `--target web`: for browser-based WASM  
- `--out-dir`: exports to frontend for integration
