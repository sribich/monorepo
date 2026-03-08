# KizaMe (刻め!)

CLI for MeCrab morphological analyzer.

**MeCab → KizaMe (刻め = "Chop up!")**

## Installation

```bash
# Default (lightweight)
cargo install kizame

# With Wikidata builder
cargo install kizame --features builder
```

## Commands

### Parse (Default)
```bash
# Basic parsing
echo "すもももももももものうち" | kizame
kizame -d /var/lib/mecab/dic/ipadic-utf8 parse
```

### Dict
```bash
kizame dict init          # Find IPADIC
kizame dict info          # Show stats
kizame dict dump -d /path # Inspect
kizame dict dump -d /path --vocab > vocab.txt  # Extract vocabulary
```

### Explore (Interactive TUI Debugger)

```bash
# Launch interactive lattice debugger
kizame explore "東京に行く"

# With custom dictionary
kizame explore -d /path/to/dict "テキスト"
```

**Screenshot:**

![TUI Debugger](../docs/tui.png)

**Features:**
- Interactive lattice visualization with cost breakdown
- Vim-style navigation (h/j/k/l, arrows)
- Best path highlighting
- Node-by-node cost inspection
- Connection cost visualization
- Press `?` for help, `q` to quit

## Options

| Flag | Description |
|------|-------------|
| `-d, --dicdir` | Dictionary path |
| `-O, --output-format` | default/dump |
| `-n, --nbest` | N-best output count |
| `--with-ipa` | Include IPA pronunciation (requires \| cat) |

## Output Formats

| Format | Description |
|--------|-------------|
| `default` | MeCab-compatible output |
| `dump` | Full token details |
| `json` | JSON array output |
| `turtle` | Turtle (TTL) RDF format |
| `ntriples` | N-Triples RDF format |
| `nquads` | N-Quads RDF format |

## Examples

```bash
# Basic parsing
echo "東京都庁で会議" | kizame

# Turtle (TTL) RDF output
echo "東京に行く" | kizame -O turtle

# N-Triples RDF output
echo "東京に行く" | kizame -O ntriples

# N-Quads RDF output
echo "東京に行く" | kizame -O nquads

# N-best paths
echo "すもも" | kizame -n 5

# Streaming large files
cat large_corpus.txt | kizame -w > tokenized.txt
```

## License

MIT OR Apache-2.0
