# NetPrune 
 
[![Crates.io](https://img.shields.io/crates/v/netprune.svg)](https://crates.io/crates/netprune) 
[![Documentation](https://docs.rs/netprune/badge.svg)](https://docs.rs/netprune) 
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 
 
Smart LinkedIn Connection Management Tool 
 
A Rust-based tool to analyze and manage your LinkedIn network using intelligent keyword filtering. 
 
## Installation 
 
```bash 
cargo install netprune 
``` 
 
Or build from source: 
 
```bash 
git clone https://github.com/ChronoCoders/netprune.git 
cd netprune 
cargo build --release 
``` 
 
## Quick Start 
 
### 1. Export Your LinkedIn Connections 
 
1. Go to LinkedIn Settings and Privacy 
2. Data Privacy - Get a copy of your data 
3. Select Connections and download CSV 
 
### 2. Configure Filters 
 
Copy config.example.toml to config.toml and customize keywords. 
 
### 3. Analyze Your Network 
 
```bash 
netprune analyze --input Connections.csv 
``` 
 
### 4. Export Filtered Connections 
 
```bash 
netprune export --input Connections.csv --output unwanted.csv 
``` 
 
## Important Notice 
 
Browser automation is experimental and not recommended. 
 
LinkedIn Terms of Service prohibit automated access. Using automation may result in: 
 
- Account suspension or ban 
- CAPTCHA challenges 
- Rate limiting 
 
Recommended Workflow: 
 
1. Use NetPrune for analysis and filtering 
2. Export filtered list to CSV 
3. Remove connections manually via LinkedIn 
4. Spread removals over days or weeks 
 
## Links 
 
- ?? [Landing Page](https://chronocoders.github.io/netprune/) 
- ?? [Crates.io](https://crates.io/crates/netprune) 
- ?? [Documentation](https://docs.rs/netprune) 
- ?? [Report Issues](https://github.com/ChronoCoders/netprune/issues) 
 
## License 
 
MIT 
