# rusty-retriever

A small, practical Rust project: build a local text search index (TF-IDF and/or BM25), query it with cosine similarity, and evaluate retrieval quality (Recall@k, MRR, nDCG).

This is intentionally “ML-adjacent”: it teaches you real Rust (I/O, error handling, serialization, iterators, testing, CLI ergonomics) while producing a demo-able retriever you can actually use.

## Features

- Index a folder of `.txt` / `.md` files into a compact on-disk index
- Retrieval:
  - TF-IDF + cosine similarity (baseline)
  - BM25 (recommended)
- Fast CLI:
  - `index` to build
  - `query` to search
  - `eval` to score on a labeled set
- Save/load index (serde + bincode)
- Deterministic results (stable tokenization & ranking)

## Why this repo exists

Most “learn Rust by doing ML” projects either:
1) become a math library rewrite, or
2) get stuck fighting GPU bindings.

This one stays grounded: ship a useful retriever, measure it, and keep the code clean.

## Quickstart

### 1) Build
```bash
cargo build --release
````

### 2) Index a folder

```bash
./target/release/rusty-retriever index \
  --corpus ./data/corpus \
  --out ./data/index.bin \
  --model bm25
```

### 3) Query it

```bash
./target/release/rusty-retriever query \
  --index ./data/index.bin \
  --topk 5 \
  "how do I change a tire"
```

Example output:

* ranked results with score
* file path
* small snippet + token offsets (optional)

## CLI

### `index`

Build an index from a directory of documents.

```bash
rusty-retriever index --corpus <DIR> --out <FILE> --model <bm25|tfidf>
```

Options (typical):

* `--corpus`: directory containing `.txt`/`.md`
* `--out`: output file for serialized index
* `--model`: `bm25` (default) or `tfidf`
* `--min-token-len`: filter tiny tokens (default: 2)
* `--stopwords`: path to stopword list (optional)

### `query`

Search an existing index.

```bash
rusty-retriever query --index <FILE> --topk 10 "your query here"
```

Options (typical):

* `--topk`: number of hits
* `--show-snippet`: print matching snippet (optional)
* `--json`: machine-readable output for piping into other tools (optional)

### `eval`

Evaluate retrieval on labeled queries.

```bash
rusty-retriever eval --index <FILE> --qrels ./data/qrels.jsonl --topk 10
```

Metrics:

* Recall@k
* MRR@k
* nDCG@k

## Data formats

### Corpus

A directory with text files. Example:

```
data/corpus/
  doc1.md
  doc2.txt
  notes/
    doc3.md
```

### Qrels (evaluation set)

`JSONL` with one query per line:

```json
{"query":"reset my password","relevant":["help/password_reset.md","kb/auth.txt"]}
{"query":"expense policy","relevant":["hr/expenses.md"]}
```

Notes:

* `relevant` values should match the relative paths stored in the index.
* You can include multiple relevant docs per query.

## Project layout (suggested)

```
rusty-retriever/
  crates/
    indexer/      # tokenization, term stats, corpus loading
    retrieval/    # tf-idf, bm25, vector scoring
    eval/         # metrics + qrels loader
  apps/
    cli/          # clap CLI wired to crates
  data/           # tiny sample corpus + example qrels
```

## Roadmap (nice-to-haves)

* [ ] Better tokenization (unicode aware, optional stemming)
* [ ] Phrase queries / AND/OR filtering
* [ ] Reranker (logistic regression on sparse features)
* [ ] Benchmark harness (criterion)
* [ ] Memory-mapped index for large corpora

## Development

### Format / lint / test

```bash
cargo fmt
cargo clippy --all-targets --all-features -D warnings
cargo test
```

### Recommended: CI

Add a GitHub Actions workflow that runs the three commands above on push/PR.

## License

MIT (or Apache-2.0).
