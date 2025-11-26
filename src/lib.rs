/// NetPrune - LinkedIn Connection Management Tool 
/// 
/// A Rust library for analyzing and managing LinkedIn connections 
/// using intelligent keyword-based filtering. 
/// 
/// # Features 
/// 
/// - Parse LinkedIn CSV exports 
/// - Filter connections by keywords 
/// - Export filtered results 
/// - Statistical analysis 
/// 
/// # Example 
/// 
/// ```rust 
/// use netprune::csv_parser::parse_csv; 
/// use netprune::filter::filter_connections; 
/// use netprune::config::load_config; 
/// 
/// let config = load_config().unwrap(); 
/// let connections = parse_csv("connections.csv").unwrap(); 
/// let results = filter_connections(&connections, &config.filters); 
/// ``` 
 
pub mod csv_parser; 
pub mod filter; 
pub mod config; 
pub mod models; 
pub mod automation; 
