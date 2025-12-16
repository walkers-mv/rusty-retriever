use indexer::InvertedIndex;
use std::collections::HashMap;

pub struct BM25Scorer {
    pub k1: f64,
    pub b: f64,
}

pub struct QueryResult {
    pub doc_id: usize,
    pub score: f64,
}

impl BM25Scorer {
    pub fn score_query(&self, query_terms: &[String], index: &InvertedIndex) -> Vec<QueryResult> {
        // ... compute IDF, score each doc, return ranked list
        let query_idfs: HashMap<String, f64> = query_terms
            .iter()
            .map(|term| {
                let df = index
                    .term_to_docs
                    .get(term)
                    .map(|docs| docs.len())
                    .unwrap_or(0);
                let idf = if df > 0 {
                    ((index.total_docs as f64 - df as f64 + 0.5) / (df as f64 + 0.5)).ln()
                } else {
                    0.0
                };
                (term.clone(), idf)
            })
            .collect();

        let mut results: Vec<QueryResult> = (0..index.total_docs)
            .filter_map(|doc_id| {
                let mut total_score = 0.0;
                let doc_len = index.doc_lengths[doc_id] as f64;

                for term in query_terms {
                    if let Some(doc_list) = index.term_to_docs.get(term) {
                        if let Some(&(_, term_freq)) =
                            doc_list.iter().find(|&&(id, _)| id == doc_id)
                        {
                            let idf = query_idfs[term];
                            let tf_component = (term_freq as f64 * (self.k1 + 1.0))
                                / (term_freq as f64
                                    + self.k1
                                        * (1.0 - self.b
                                            + self.b * (doc_len / index.avg_doc_length)));
                            total_score += idf * tf_component;
                        }
                    }
                }
                if total_score > 0.0 {
                    Some(QueryResult {
                        doc_id,
                        score: total_score,
                    })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }
}
