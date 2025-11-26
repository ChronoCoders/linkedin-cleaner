# LinkedIn Connection Cleaner 
 
A Rust-based tool to analyze and manage LinkedIn connections using keyword filtering and browser automation. 
 
## Features 
 
- CSV parsing and analysis of LinkedIn connections 
- Keyword-based filtering for connection management 
- Export filtered results to CSV 
- Browser automation for bulk connection removal (in progress) 
- Configurable rate limiting and safety features 
 
## Installation 
 
```bash 
cargo build --release 
``` 
 
## Configuration 
 
Copy `config.example.toml` to `config.toml` and adjust settings: 
 
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
 
## Usage 
 
### 1. Export your LinkedIn connections 
 
Go to LinkedIn Settings & Privacy > Data Privacy > Get a copy of your data > Select "Connections" and download the CSV. 
 
### 2. Analyze connections 
```bash 
cargo run --release -- analyze --input Connections.csv 
``` 
 
### 3. Export filtered connections 
```bash 
cargo run --release -- export --input Connections.csv --output unwanted.csv 
``` 
 
### 4. Remove connections (experimental) 
```bash 
cargo run --release -- remove --input Connections.csv --email "your@email.com" --password "yourpassword" 
``` 
 
## Safety Notes 
 
**Rate Limiting Recommendations:** 
- New accounts: 10 removals/day max 
- Established accounts: 20-30 removals/day max 
- Always test with dry_run = true first 
- Use delays between actions to avoid detection 
 
## License 
 
MIT 
 
## ?? Important Notice 
 
**Browser automation is experimental and not recommended for production use.** 
 
LinkedIn's Terms of Service prohibit automated access and scraping. Using the automation features may result in: 
 
- Account suspension or permanent ban 
- CAPTCHA challenges 
- Rate limiting 
- Legal action 
 
**Recommended Usage:** 
 
1. Use this tool for **analysis and filtering only** 
2. Export the filtered connection list to CSV 
3. Use LinkedIn's official interface to remove connections manually 
4. Spread removals over days/weeks to avoid detection 
 
The automation code is provided for **educational purposes only**. Use at your own risk. 
