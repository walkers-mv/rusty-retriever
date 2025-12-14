use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rusty-retriever")]
#[command(about = "A local text search and retrieval system")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build an index from a directory of documents
    Index {
        /// Directory containing text files to index
        #[arg(long)]
        corpus: PathBuf,

        /// Output file for the serialized index
        #[arg(long)]
        out: PathBuf,

        /// Scoring model to use
        #[arg(long, default_value = "bm25")]
        model: ScoringModel,

        /// Minimum token length (filter out shorter tokens)
        #[arg(long, default_value = "2")]
        min_token_len: usize,

        /// Path to stopwords file (optional)
        #[arg(long)]
        stopwords: Option<PathBuf>,
    },
    /// Search an existing index
    Query {
        /// Path to the serialized index file
        #[arg(long)]
        index: PathBuf,

        /// Number of results to return
        #[arg(long, default_value = "10")]
        topk: usize,

        /// Show matching snippets in results
        #[arg(long)]
        show_snippet: bool,

        /// Output results as JSON
        #[arg(long)]
        json: bool,

        /// Query string
        query: String,
    },
    /// Evaluate retrieval quality on labeled data
    Eval {
        /// Path to the serialized index file
        #[arg(long)]
        index: PathBuf,

        /// Path to qrels JSONL file
        #[arg(long)]
        qrels: PathBuf,

        /// Number of results to evaluate (k for Recall@k, etc.)
        #[arg(long, default_value = "10")]
        topk: usize,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum ScoringModel {
    #[value(name = "tfidf")]
    TfIdf,
    #[value(name = "bm25")]
    Bm25,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Index {
            corpus,
            out,
            model,
            min_token_len,
            stopwords,
        } => {
            log::info!("Building index from corpus: {:?}", corpus);
            log::info!("Using model: {:?}", model);
            log::info!("Output file: {:?}", out);

            // TODO: Implement indexing logic
            // This will involve:
            // 1. Loading documents from corpus directory
            // 2. Tokenizing and preprocessing text
            // 3. Building term statistics based on chosen model
            // 4. Serializing index to output file

            println!(
                "Index command - corpus: {:?}, out: {:?}, model: {:?}",
                corpus, out, model
            );
            if let Some(stopwords_file) = stopwords {
                println!("Using stopwords from: {:?}", stopwords_file);
            }
            println!("Minimum token length: {}", min_token_len);
        }

        Commands::Query {
            index,
            topk,
            show_snippet,
            json,
            query,
        } => {
            log::info!("Querying index: {:?}", index);
            log::info!("Query: {}", query);
            log::info!("Top-k: {}", topk);

            // TODO: Implement query logic
            // This will involve:
            // 1. Loading serialized index
            // 2. Tokenizing query
            // 3. Computing scores for all documents
            // 4. Ranking and returning top-k results
            // 5. Formatting output (plain text or JSON)

            if json {
                println!("{{\"query\": \"{}\", \"results\": []}}", query);
            } else {
                println!("Query: {}", query);
                println!("Top {} results:", topk);
                if show_snippet {
                    println!("(Including snippets)");
                }
            }
        }

        Commands::Eval { index, qrels, topk } => {
            log::info!("Evaluating index: {:?}", index);
            log::info!("Using qrels: {:?}", qrels);
            log::info!("Evaluating at k={}", topk);

            // TODO: Implement evaluation logic
            // This will involve:
            // 1. Loading serialized index
            // 2. Loading qrels file
            // 3. Running queries from qrels
            // 4. Computing metrics (Recall@k, MRR@k, nDCG@k)
            // 5. Reporting results

            println!(
                "Evaluation - index: {:?}, qrels: {:?}, topk: {}",
                index, qrels, topk
            );
        }
    }

    Ok(())
}
