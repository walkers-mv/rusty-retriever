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

pub struct Index {
    pub documents: Vec<Document>,
    pub token2id: HashMap<String, usize>,
    pub id2token: HashMap<usize, String>,
    pub token2doc: HashMap<usize, Vec<usize>>,
    pub doc2token: HashMap<usize, Vec<usize>>,
    // How do you store "word -> list of documents"
    // Hint: HashMap<String, Vec<???>>
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
