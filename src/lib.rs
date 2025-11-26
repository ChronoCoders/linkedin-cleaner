//! NetPrune - LinkedIn Connection Management Tool 
//! 
//! A Rust library for analyzing and managing LinkedIn connections 
//! using intelligent keyword-based filtering. 
//! 
//! # Features 
//! 
//! - Parse LinkedIn CSV exports 
//! - Filter connections by keywords 
//! - Export filtered results 
//! - Statistical analysis 
//! - Browser automation (experimental) 
//! 
//! # Example 
//! 
//! ```no_run 
//! use netprune::parser::csv_parser::CsvParser; 
//! use netprune::models::config::Config; 
//! 
//! let config = Config::load("config.toml").unwrap(); 
//! let connections = CsvParser::parse_connections("connections.csv").unwrap(); 
//! ``` 
 
pub mod parser; 
pub mod filters; 
pub mod models; 
pub mod analyzer; 
pub mod automation; 
