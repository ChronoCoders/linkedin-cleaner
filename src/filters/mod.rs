use crate::models::{Config, Connection, FilterResult, Person};

pub struct FilterEngine {
    config: Config,
}

impl FilterEngine {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn filter_connection(&self, connection: &Connection) -> FilterResult {
        let mut reasons = Vec::new();
        
        if connection.position.trim().is_empty() {
            reasons.push("Empty position".to_string());
        }
        
        let text = format!(
            "{} {}",
            connection.position.to_lowercase(),
            connection.company.to_lowercase()
        );
        
        if self.has_relevant_keywords(&text) {
            return FilterResult {
                person: Person::Connection(connection.clone()),
                should_remove: false,
                reasons: vec![],
            };
        }
        
        if self.has_unwanted_keywords(&text) {
            reasons.push("Unwanted industry/position".to_string());
        }
        
        if !connection.position.trim().is_empty() {
            reasons.push("Not relevant to blockchain/tech".to_string());
        }
        
        let should_remove = !reasons.is_empty();
        
        FilterResult {
            person: Person::Connection(connection.clone()),
            should_remove,
            reasons,
        }
    }

    fn has_unwanted_keywords(&self, text: &str) -> bool {
        self.config.filters.unwanted_keywords
            .iter()
            .any(|kw| text.contains(kw))
    }

    fn has_relevant_keywords(&self, text: &str) -> bool {
        self.config.filters.relevant_keywords
            .iter()
            .any(|kw| text.contains(kw))
    }
}