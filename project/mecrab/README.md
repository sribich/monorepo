# MeCrab - High-Performance Morphological Analyzer

<p align="center">
  <img src="docs/mecrab.png" alt="MeCrab" width="800">
</p>

A pure Rust implementation of a morphological analyzer compatible with MeCab dictionaries (IPADIC format).

## Workspace Structure

```
mecrab/
├── mecrab/          # Core library - runtime morphological analyzer
├── kizame/          # CLI tool - KizaMe (刻め!)
├── fuzz/            # Fuzz testing
└── docs/            # Engineering directives and design docs
```

| Crate | Description | Dependencies |
|-------|-------------|--------------|
| `mecrab` | Core runtime library | Minimal (lightweight) |
| `kizame` | CLI with optional builder | Lightweight default |

## Features

- **High Performance**: SIMD-accelerated Viterbi (AVX2), parallel batch processing
- **Zero-copy Parsing**: Memory-mapped dictionary loading
- **Thread-safe**: Safe concurrent access using Rust's ownership model
- **Live Dictionary Updates**: Add/remove words at runtime without restart
- **Semantic Enrichment**: Wikidata URI linking, JSON-LD/RDF export (Turtle, N-Triples, N-Quads)
- **N-best Paths**: A* algorithm for multiple path analysis
- **Streaming Processing**: Sentence boundary detection for large text
- **Text Normalization**: NFKC, width conversion, case folding
- **Phonetic Transduction**: Kana ↔ Romaji, X-SAMPA, IPA conversion
- **Interactive TUI Debugger**: Lattice explorer with cost visualization ([screenshot](docs/tui.png))

> **Note:** When using `--with-ipa` with default text output, pipe through `cat` for proper terminal display of Unicode IPA characters. This is not required for JSON-LD output.

## Installation

### CLI (KizaMe)

```bash
# Default (lightweight)
cargo install kizame

# With Wikidata builder
cargo install kizame --features builder
```

### Rust Library

```toml
[dependencies]
mecrab = "0.1"

# Optional features
mecrab = { version = "0.1", features = ["json", "parallel"] }
```

## Quick Start

### CLI Usage

```bash
# Initialize dictionary (finds system IPADIC)
kizame dict init

# Interactive parsing
echo "すもももももももものうち" | kizame

# With dictionary path
kizame -d /var/lib/mecab/dic/ipadic-utf8 parse

# Dictionary info
kizame dict info
kizame dict dump -d /path/to/dic

# Interactive TUI debugger
kizame explore "東京に行く"
```

### Rust API

```rust
use mecrab::MeCrab;

let mecrab = MeCrab::new()?;
let result = mecrab.parse("すもももももももものうち")?;
println!("{}", result);

// Add custom words at runtime
mecrab.add_word("ChatGPT", "チャットジーピーティー", "チャットジーピーティー", 5000);

// N-best paths
use mecrab::viterbi::NbestSearch;
let nbest = NbestSearch::new(&mecrab);
for path in nbest.search("東京", 5)? {
    println!("Cost: {}, Path: {:?}", path.total_cost, path.nodes);
}

// Streaming processing
use mecrab::stream::SentenceReader;
let reader = SentenceReader::new(input);
for sentence in reader {
    let result = mecrab.parse(&sentence)?;
}
```

### Performance

- **83% parallel efficiency** on 6 cores (499.7% CPU usage)
- **~500K words/sec/core** training throughput
- **2.27x speedup** vs mutex-based baseline

## Development

```bash
# Run tests
cargo nextest run

# Clippy (no warnings policy)
cargo clippy --workspace

# Benchmarks
cargo bench -p mecrab

# Fuzz testing
cd fuzz && cargo +nightly fuzz run viterbi
```

## Statistics

- ~11,000 lines of Rust
- 174 tests (all passing)
- 4 fuzz targets
- 0 clippy warnings

## License

MIT OR Apache-2.0

## Copyright

Copyright 2026 COOLJAPAN OU (Team KitaSan)
