use crate::models::Connection;
use anyhow::Result;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CsvParser;

impl CsvParser {
    pub fn parse_connections(path: &str) -> Result<Vec<Connection>> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);

        for _ in 0..3 {
            let mut line = String::new();
            buf_reader.read_line(&mut line)?;
        }

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(buf_reader);

        let mut connections = Vec::new();

        for result in reader.deserialize() {
            let connection: Connection = result?;
            connections.push(connection);
        }

        Ok(connections)
    }
}
