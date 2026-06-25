use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Terminal {
    buffer: String,
    history: Vec<String>,
    history_idx: usize,           
    vfs: HashMap<String, String>, 
    cwd: String,                  
}

#[wasm_bindgen]
impl Terminal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Terminal {
        let mut vfs = HashMap::new();
        
        vfs.insert(
            "/projects/cryptography_and_zk/decomm.md".to_string(),
            "# Decomm | Sovereign P2P Communication\n\
            Engineered a decentralized communication node in Rust using a libp2p Gossipsub mesh.\n\
            Implemented a two-phase ML-KEM-1024 (Kyber) key encapsulation handshake.\n\
            Integrated RISC Zero zkVM for ZK Merkle membership proving against a Solana-anchored root.".to_string(),
        );
        vfs.insert(
            "/projects/cryptography_and_zk/zk_identity_auth.sol".to_string(),
            "// ZK Identity Auth | Privacy-Preserving Group Authentication\n\
            // Circom 2.1.5 circuits: Poseidon-hashed commitments in 20-level Merkle tree.\n\
            // Groth16 zk-SNARK proof generation in-browser via snarkjs WASM.\n\
            // Solidity 0.8.20 contracts with nullifier replay protection.\n\
            // Hardhat deployment + Vite / React 19 frontend.".to_string(),
        );
        vfs.insert(
            "/projects/cryptography_and_zk/zk_identifier.txt".to_string(),
            "ZK Identifier | Privacy-Preserving Decentralized Identity\n\
            - Custom Circom arithmetic circuits (Groth16)\n\
            - Poseidon hashing & incremental Merkle membership proofs\n\
            - Rust backend with nullifier scoping for zero on-chain data exposure.".to_string(),
        );

        vfs.insert(
            "/projects/systems_and_infra/triad.rs".to_string(),
            "// TRIAD | Quantitative Trading Infrastructure\n\
            fn main() {\n    \
                let messaging = \"NATS JetStream\";\n    \
                let time_series = \"TimescaleDB\";\n    \
                let memory_layer = \"pgvector with HNSW indexing\";\n    \
                println!(\"Real-time microstructure signals via Binance WebSocket.\");\n\
            }".to_string(),
        );
        vfs.insert(
            "/projects/systems_and_infra/fraud_engine.py".to_string(),
            "# Fraud Engine | Real-Time Stream Processing Detection\n\
            class Architecture:\n    \
                def __init__(self):\n        \
                    self.broker = \"Redpanda (Kafka-compatible)\"\n        \
                    self.rules = [\"Velocity\", \"Amount > 5x avg\", \"Location mismatch\"]\n        \
                    self.ml = \"XGBoost classifier with SHAP explainability\"\n        \
                    self.observability = \"Prometheus metrics + Grafana dashboards\"\n    \
                def detect(self, tx):\n        \
                    return \"FastAPI + Redis state store, ~5.2 TPS, Streamlit FraudOps UI\"\n".to_string(),
        );
        vfs.insert(
            "/projects/systems_and_infra/crm_system.ts".to_string(),
            "// CRM System | Full-Stack Lead & Loan Management\n\
            const backend = {\n    \
                framework: \"Fastify + TypeScript\",\n    \
                database: \"PostgreSQL with Zod-validated schemas\",\n    \
                queue: \"BullMQ + Redis asynchronous workers\",\n    \
                auth: \"JWT + Bcrypt RBAC (Admin, Loan Officer)\"\n\
            };\n\
            const frontend = {\n    \
                framework: \"Next.js 16 + React 19\",\n        \
                styling: \"Tailwind CSS 4\",\n        \
                forms: \"React Hook Form + Zod validation\"\n\
            };\n\
            // Intelligent lead scoring, smart officer assignment, idempotency safeguards.".to_string(),
        );
        vfs.insert(
            "/projects/systems_and_infra/envision.tsx".to_string(),
            "// Envision | Provably Fair Binary Dice Terminal\n\
            const config = {\n    \
                fairness: \"Reverse hash chain HMAC-SHA256 pre-commitment\",\n    \
                transport: \"Binary Protocol Buffers over Socket.IO (60-70% smaller than JSON)\",\n    \
                storage: \"SQLite WAL mode for high-frequency transaction safety\",\n    \
                viz: \"Chart.js real-time balance trading terminal\"\n\
            };\n\
            // Client-side cryptographic verification, automated bots, sub-100ms WebSocket latency.".to_string(),
        );
        vfs.insert(
            "/projects/systems_and_infra/saferoute.py".to_string(),
            "# SafeRoute | AI-Powered Route Safety Analyzer\n\
            class SafetyEngine:\n    \
                def __init__(self):\n        \
                    self.coverage = \"12 Delhi metro locations\"\n        \
                    self.scale = \"6-level risk priority (0 Critical -> 5 Excellent)\"\n        \
                    self.ui = \"Multi-page Streamlit dashboard\"\n    \
                def analyze(self, src, dst):\n        \
                    return \"Alternative safer routes for women commuters\"\n".to_string(),
        );

        vfs.insert(
            "/projects/web3_protocols/erc4626_vault.sol".to_string(),
            "// ERC4626 Vault | ZK-Enabled Yield Aggregator\n\
            // Solidity ERC4626 vault accepting USDC, minting vUSDC, routing capital to Aave V3.\n\
            // Groth16 zk-SNARK circuits (Circom / SnarkJS) for off-chain proof-of-reserves.\n\
            // On-chain ProofOfReserves oracle + Groth16Verifier deployed on Ethereum Sepolia.\n\
            // Next.js frontend with Wagmi, RainbowKit, and Viem Web3 integration.".to_string(),
        );
        vfs.insert(
            "/projects/web3_protocols/por_vault.rs".to_string(),
            "// PoR Vault | Time-Locked Bitcoin Inheritance\n\
            fn main() {\n    \
                let script = \"P2WSH with OP_CSV + OP_IF Dead Man's Switch\";\n    \
                let storage = \"AES-256-GCM keystore, BIP-174 PSBT air-gapped cold storage\";\n    \
                let watchtower = \"Autonomous multi-threaded daemon monitoring mempool.space\";\n    \
                let recovery = \"Miniscript descriptors compatible with Sparrow / Specter\";\n    \
                println!(\"Non-custodial inheritance vault with BIP-125 RBF mempool resilience.\");\n\
            }".to_string(),
        );
        vfs.insert(
            "/projects/web3_protocols/farmsetu.ts".to_string(),
            "// FarmSetu | Decentralized Agricultural Forward Contracts\n\
            const platform = {\n    \
                chain: \"Algorand TEAL smart contracts\",\n    \
                wallet: \"Pera Wallet connect with persistent sessions\",\n    \
                oracle: \"AlphaVantage state-specific commodity prices (INR/quintal)\",\n    \
                settlement: \"Automatic price-difference payout, zero intermediaries\"\n\
            };\n\
            // 29 Indian states, role-based dashboards for farmers and buyers.".to_string(),
        );
        vfs.insert(
            "/projects/web3_protocols/wallet_adapter.tsx".to_string(),
            "// Wallet Adapter | Solana dApp Scaffold\n\
            const stack = {\n    \
                framework: \"React 19 + Vite\",\n    \
                chain: \"Solana Devnet\",\n    \
                adapter: \"Solana Wallet Adapter + Web3.js\",\n    \
                rpc: \"Alchemy for fast, reliable transactions\"\n\
            };\n\
            // Lightweight wallet connection and SOL airdrop request interface.".to_string(),
        );

        vfs.insert(
            "/experience/lokachakra.md".to_string(),
            "## Lokachakra (UK-Based Technology Startup)\n\
            **Lead Blockchain and Zero-Knowledge Proof Intern**\n\
            Built a ZKP-based identity and KYC backend in Rust with automated privacy-by-design pipelines.\n\
            Engineered a cryptographically secure wallet system achieving 5,000+ TPS at sub-15ms latency.".to_string(),
        );
        vfs.insert(
            "/experience/digital_south_trust.md".to_string(),
            "## Digital South Trust\n\
            **Software Engineer Intern (Emerging Technologies)**\n\
            Engineered a government-facing permissioned blockchain (Hyperledger Fabric, 8 nodes, 50K+ daily tx).\n\
            Implemented ZKP-based cryptographic access control, reducing data exposure by 99%.".to_string(),
        );

        vfs.insert(
            "/research/pqc_migration.pdf".to_string(),
            "[ERR: Binary file cannot be read in standard output]\n\
            Metadata: Post-Quantum Cryptography: Preparing for the Quantum Threat.\n\
            G-CARED 2025 International Conference.\n\
            Proposed hybrid migration strategies achieving 35% lower performance overhead.".to_string(),
        );

        Terminal {
            buffer: String::new(),
            history: Vec::new(),
            history_idx: 0,       
            vfs,
            cwd: "/".to_string(),
        }
    }

    pub fn process_key(&mut self, key: &str) -> String {
        match key {
            "Enter" => {
                let cmd = self.buffer.trim().to_string();
                self.buffer.clear();
                if cmd.is_empty() { return String::new(); }
                self.history.push(cmd.clone());
                self.history_idx = self.history.len(); 
                self.execute_command(&cmd)
            }
            "Backspace" => { self.buffer.pop(); self.buffer.clone() }
            _ if key.chars().count() == 1 => { self.buffer.push_str(key); self.buffer.clone() }
            _ => self.buffer.clone(),
        }
    }

    pub fn get_buffer(&self) -> String { self.buffer.clone() }
    
    pub fn get_cwd(&self) -> String { self.cwd.clone() }
    
    pub fn handle_up(&mut self) -> String {
        if self.history.is_empty() || self.history_idx == 0 {
            return self.buffer.clone();
        }
        self.history_idx -= 1;
        self.buffer = self.history[self.history_idx].clone();
        self.buffer.clone()
    }

    pub fn handle_down(&mut self) -> String {
        if self.history.is_empty() || self.history_idx >= self.history.len() {
            return self.buffer.clone();
        }
        self.history_idx += 1;
        if self.history_idx == self.history.len() {
            self.buffer.clear(); // Reached the bottom, clear buffer
        } else {
            self.buffer = self.history[self.history_idx].clone();
        }
        self.buffer.clone()
    }

    pub fn handle_tab(&mut self) -> String {
        let parts: Vec<&str> = self.buffer.split_whitespace().collect();
        if parts.is_empty() { return self.buffer.clone(); }

        let cmd = parts[0];
        
        if (cmd == "cd" || cmd == "cat") && parts.len() == 2 {
            let prefix = parts[1];
            let search_dir = if self.cwd == "/" { "/" } else { &format!("{}/", self.cwd) };
            let mut matches = std::collections::HashSet::new();

            for (path, _) in &self.vfs {
                if path.starts_with(search_dir) {
                    let remainder = path.replace(search_dir, "");
                    let name = if let Some(slash_idx) = remainder.find('/') {
                        &remainder[..slash_idx]
                    } else {
                        &remainder
                    };
                    
                    if name.starts_with(prefix) {
                        matches.insert(name.to_string());
                    }
                }
            }

            if matches.len() == 1 {
                let matched_name = matches.into_iter().next().unwrap();
                self.buffer = format!("{} {} ", cmd, matched_name);
            }
        }
        self.buffer.clone()
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    fn execute_command(&mut self, cmd_line: &str) -> String {
        let parts: Vec<&str> = cmd_line.split_whitespace().collect();
        if parts.is_empty() { return String::new(); }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "help" => "COMMANDS: help, whoami, clear, pwd, ls, cd <dir>, cat <file>".to_string(),
            "whoami" => "Bhavya Jain\nSystems & Cryptography Engineer\nCurrently building: AlterBlock".to_string(),
            "pwd" => self.cwd.clone(),
            "clear" => {
                self.history.clear();
                "__CLEAR_SCREEN__".to_string()
            }
            "ls" => {
                let mut target_dir = self.cwd.clone();
                if !args.is_empty() {
                    let target = args[0];
                    if target_dir == "/" {
                        target_dir = format!("/{}", target);
                    } else {
                        target_dir = format!("{}/{}", target_dir, target);
                    }
                }

                let search_dir = if target_dir == "/" { "/".to_string() } else { format!("{}/", target_dir) };
                let mut entries = std::collections::HashSet::new();

                for (path, _) in &self.vfs {
                    if path.starts_with(&search_dir) {
                        let remainder = path.replace(&search_dir, "");
                        if remainder.is_empty() { continue; } // Skip if it matches the dir exactly
                        
                        if let Some(slash_idx) = remainder.find('/') {
                            // It's a nested directory
                            entries.insert(format!("<span class='c-dir'>{}</span>", &remainder[..slash_idx]));
                        } else {
                            // It's a file
                            entries.insert(format!("<span class='c-file'>{}</span>", remainder));
                        }
                    }
                }

                if entries.is_empty() { 
                    if !args.is_empty() && self.vfs.contains_key(&target_dir) {
                        // They tried to `ls` a specific file
                        format!("<span class='c-file'>{}</span>", args[0])
                    } else if !args.is_empty() {
                        format!("<span class='c-err'>ls: cannot access '{}': No such file or directory</span>", args[0])
                    } else {
                        " ".to_string()
                    }
                } else { 
                    let mut output: Vec<String> = entries.into_iter().collect();
                    output.sort(); 
                    output.join("  ") 
                }
            }
            "cd" => {
                if args.is_empty() {
                    self.cwd = "/".to_string();
                    return String::new();
                }
                let target = args[0];
                if target == ".." {
                    if self.cwd != "/" {
                        let mut parts: Vec<&str> = self.cwd.split('/').collect();
                        parts.pop(); 
                        let new_cwd = parts.join("/");
                        self.cwd = if new_cwd.is_empty() { "/".to_string() } else { new_cwd };
                    }
                    return String::new();
                }
                
                let new_path = if self.cwd == "/" { format!("/{}", target) } else { format!("{}/{}", self.cwd, target) };
                
                if self.vfs.contains_key(&new_path) {
                    return format!("<span class='c-err'>cd: {}: Not a directory</span>", target);
                }
                
                let dir_exists = self.vfs.keys().any(|k| k.starts_with(&format!("{}/", new_path)));
                
                if dir_exists {
                    self.cwd = new_path;
                    String::new()
                } else {
                    format!("<span class='c-err'>cd: {}: No such file or directory</span>", target) 
                }
            }
            "cat" => {
                if args.is_empty() { return "<span class='c-err'>cat: missing file operand</span>".to_string(); }
                let target = args[0];
                let full_path = if self.cwd == "/" { format!("/{}", target) } else { format!("{}/{}", self.cwd, target) };
                
                match self.vfs.get(&full_path) {
                    Some(content) => content.clone(),
                    None => format!("<span class='c-err'>cat: {}: No such file</span>", target),
                }
            }
            _ => format!("<span class='c-err'>bhavya-os: command not found: {}</span>", command),
        }
    }
}
