use anyhow::Result;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct Document {
    pub id: usize,
    pub title: String,
    pub text: String,
    pub metadata: HashMap<String, Value>,
    // What else ??
}

pub struct InvertedIndex {
    pub term_to_docs: HashMap<String, Vec<(usize, usize)>>,
    pub documents: Vec<Document>,
    pub doc_lengths: Vec<usize>,
    pub avg_doc_length: f64,
    pub total_docs: usize,
}

pub fn load_documents(_corpus_dir: &Path) -> Result<Vec<Document>> {
    let mut documents = Vec::new();

    let corpus_file = _corpus_dir.join("corpus.jsonl");

    //Look for corpus.jsonl file in directory
    let content = std::fs::read_to_string(&corpus_file)?;

    for (index, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let json_doc: serde_json::Value = serde_json::from_str(line)?;

        let document = Document {
            id: index,
            title: json_doc["title"].as_str().unwrap_or("").to_string(),
            text: json_doc["text"].as_str().unwrap_or("").to_string(),
            metadata: HashMap::new(),
        };

        documents.push(document);

        if index % 1000 == 0 && index != 0 {
            log::info!("Processed {} documents", index + 1);
        }
    }

    log::info!("Finished processing {} documents", documents.len());

    Ok(documents)
}

pub fn tokenize(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b\w+\b").unwrap();

    let mut tokens = Vec::new();

    for mat in re.find_iter(text) {
        let word = mat.as_str().to_lowercase();

        tokens.push(word)
    }

    tokens
}

pub fn token_count(tokens: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for token in tokens {
        let count = counts.entry(token.clone()).or_insert(0);
        *count += 1;
    }

    counts
}

impl InvertedIndex {
    pub fn new(documents: Vec<Document>) -> Self {
        let mut term_to_docs = HashMap::new();
        let mut doc_lengths = Vec::new();
        let mut avg_doc_length = 0.0;
        let total_docs = documents.len();

        for doc in &documents {
            
            let tokens = tokenize(&doc.text);
            let term_frequencies = token_count(&tokens);

            doc_lengths.push(tokens.len());
            avg_doc_length += tokens.len() as f64;

            for (term, frequency) in term_frequencies {
                term_to_docs
                    .entry(term)
                    .or_insert_with(Vec::new)
                    .push((doc.id, frequency));
            }
        }
        avg_doc_length /= total_docs as f64;

        Self {
            term_to_docs,
            documents,
            doc_lengths,
            avg_doc_length,
            total_docs,
        }
    }
}
