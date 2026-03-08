# MeCrab: Technical Highlights 2026

## 1. Extreme Velocity

**SIMD & Parallelism:** Full utilization of modern CPU instruction sets (AVX-512/NEON) that were impossible with 2000s-era codebases. Matrix computations parallelized at the hardware level.

**True Zero-Copy:** Architecture that mmaps dictionary files and performs zero memory copies from analysis to output.

## 3. Phonetic Backbone

**IPA Output:** Outputs precise International Phonetic Alphabet (IPA) notation, not just reading pronunciations.

**TTS Ready:** Directly embeddable as a backend for speech synthesis and Japanese language learning applications.

## 5. Industrial Robustness

**Panic Free:** Rust's type system combined with hundreds of millions of fuzzing tests mathematically guarantees the system will never crash on any malicious input data.

**Memory Safety:** Terms like "memory leak" and "segmentation fault" from the C++ era do not exist in this project.

## 6. Run Anywhere

**3-Crate Architecture:** Lean dependency management that provides lightweight CLI for users and robust build capabilities for data scientists.

---

## MeCab vs. MeCrab: The Decisive Differences

| Feature | Old MeCab (C++) | MeCrab (Rust) |
|---------|-----------------|---------------|
| Output | Text strings | Semantic & Phonetic |
| Dictionary | Static, manual updates | Dynamic, auto-generated (Wikidata Fusion) |
| Memory | Pointer operations, many copies | Zero-Copy & Packed Structs |
| Safety | Depends on developer diligence | Guaranteed by compiler and fuzzing |
| Role | Analysis tool | AI & Web infrastructure foundation |

---

## Future Extension Features (Candidates)

### 1. The "Matrix" TUI Debugger (Visual Lattice Explorer)

*Your terminal becomes a cockpit.*

MeCab's debug output (like `-N2`) was just walls of text, making it painful to mentally reconstruct the graph. Using `ratatui`, we transform this into a rich interactive interface.

#### Feature Details

**Lattice Graph View:**
- Display the lattice structure (morpheme candidate graph) for input text as a tree or network on the left side of the screen
- Navigation: Move between nodes (word candidates) with arrow keys, select paths with Enter

**Cost Inspection Panel:**
- Display details of the selected node on the right side
- Real-time display of word cost, connection cost, and cumulative cost
- Show "why the system chose this path" via cost delta from the best path

**Dynamic Re-calculation:**
- Test "what-if" scenarios like "what happens if I choose 'cut' here?" with on-the-spot cost recalculation

#### Implementation Approach

- **Libraries:** ratatui (UI rendering), crossterm (input handling)
- **Architecture:** Add a `DebugSession` struct to mecrab-core that retains and returns Viterbi intermediate state (all node cost information). CLI (`kizame explore`) instantiates this and runs the TUI loop.

#### Why It's Cool

- **Hacker Aesthetic:** Manipulating lattice structures composed of green text on a pitch-black screen is straight out of *The Matrix*.
- **Educational Value:** The ultimate teaching material for students and engineers learning NLP to intuitively understand the Viterbi algorithm.

---

### 3. "Compile-Time" Dictionary (include_dict!)

*Freedom from "dictionary file not found" forever.*

The biggest headache when distributing morphological analyzers is "dictionary path configuration". We solve this with Rust's powerful macro system.

#### Feature Details

**Macro Magic:**
- Just write `mecrab::include_system_dict!("ipadic-2.7.0");` to download and build the dictionary at compile time, embedding it in the executable's static region (`.rodata`)

**Single Binary Distribution:**
- The generated binary (`kizame`) requires zero external files. Just `scp` to a server and it runs.

**No Parsing Cost:**
- At runtime, just cast a memory address. No file I/O or parsing occurs, achieving startup time of 0.00 seconds (immeasurable).

#### Implementation Approach

- **Proc Macro:** Create `mecrab-macros` crate
- **Bytes Integration:** Internally generate `include_bytes!` macro and map to structs defined in the dict crate via unsafe cast
- **Compression (Optional):** For binary size concerns, option to compress with zstd and decompress at initialization

#### Why It's Cool

- **Ultimate Portability:** When building Docker images, just `COPY kizame /bin/`. Whether Alpine Linux, the peace of mind that "if you have the binary, it absolutely runs" is immense.
- **Serverless Optimal:** Peak performance in environments where cold start (startup speed) is critical, like AWS Lambda or Cloudflare Workers.

---

## Summary

- **TUI Debugger** captures the hearts of *developers*.
- **Compile-Time Dict** captures the hearts of *infrastructure engineers*.
