use crate::models::FilterResult;

pub struct Scorer;

impl Scorer {
    pub fn analyze(results: &[FilterResult]) -> AnalysisReport {
        let total = results.len();
        let unwanted = results.iter().filter(|r| r.should_remove).count();
        let relevant = total - unwanted;

        AnalysisReport {
            total,
            unwanted,
            relevant,
        }
    }
}

#[derive(Debug)]
pub struct AnalysisReport {
    pub total: usize,
    pub unwanted: usize,
    pub relevant: usize,
}
