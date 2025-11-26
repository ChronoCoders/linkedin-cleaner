# NetPrune 
 
Smart LinkedIn Connection Management Tool 
 
A Rust-based tool to analyze and manage your LinkedIn network using intelligent keyword filtering. 
 
## Features 
 
- CSV parsing and analysis of LinkedIn connections 
- Keyword-based filtering for connection management 
- Export filtered results to CSV 
- Browser automation (experimental - use at your own risk) 
- Configurable filters and safety features 
 
## Installation 
 
```bash 
cargo build --release 
``` 
 
## Quick Start 
 
### 1. Export Your LinkedIn Connections 
 
1. Go to LinkedIn Settings and Privacy 
2. Data Privacy - Get a copy of your data 
3. Select Connections and download CSV 
 
### 2. Configure Filters 
 
Copy config.example.toml to config.toml and customize keywords: 
 
```toml 
[filters] 
keywords = ["blockchain", "crypto", "web3", "rust", "developer"] 
``` 
 
### 3. Analyze Your Network 
 
```bash 
cargo run --release -- analyze --input Connections.csv 
``` 
 
### 4. Export Filtered Connections 
 
```bash 
cargo run --release -- export --input Connections.csv --output unwanted.csv 
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
 
## Configuration 
 
```toml 
[automation] 
delay_min_ms = 2000 
delay_max_ms = 5000 
batch_size = 10 
daily_limit = 20 
dry_run = true 
 
[filters] 
keywords = ["blockchain", "crypto", "web3"] 
``` 
 
## License 
 
MIT 
 
## Links 
 
- ?? [Landing Page](https://chronocoders.github.io/netprune/) 
- ?? [Wiki & Documentation](https://github.com/ChronoCoders/netprune/wiki) 
- ?? [Report Issues](https://github.com/ChronoCoders/netprune/issues) 
