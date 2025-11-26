use crate::models::{FilterResult, Person};
use crate::analyzer::AnalysisReport;
use std::fs::File;
use std::io::Write;
use anyhow::Result;

pub struct Reporter;

impl Reporter {
    pub fn print_summary(report: &AnalysisReport) {
        println!("\n=== Analysis Summary ===");
        println!("Total: {}", report.total);
        println!("Unwanted: {} ({:.1}%)", report.unwanted, (report.unwanted as f64 / report.total as f64) * 100.0);
        println!("Relevant: {} ({:.1}%)", report.relevant, (report.relevant as f64 / report.total as f64) * 100.0);
    }

    pub fn export_to_csv(results: &[FilterResult], output_path: &str) -> Result<()> {
        let mut file = File::create(output_path)?;
        
        writeln!(file, "Name,URL,Position,Company,Should Remove,Reasons")?;
        
        for result in results {
            let Person::Connection(conn) = &result.person;
            writeln!(
                file,
                "{} {},{},{},{},{},{}",
                conn.first_name,
                conn.last_name,
                conn.url,
                conn.position,
                conn.company,
                result.should_remove,
                result.reasons.join("; ")
            )?;
        }
        
        Ok(())
    }
}