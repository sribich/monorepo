pub mod bench;
// pub mod debug;
pub mod dict;
pub mod error;
pub mod lattice;
pub mod viterbi;

use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use dict::Dictionary;
pub use error::Error;
pub use error::Result;
use lattice::Lattice;
use railgun_error::ResultExt;
use viterbi::ViterbiSolver;

/// Output format for morphological analysis results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Default MeCab format: surface\tfeatures
    #[default]
    Default,
    /// Dump all lattice information for debugging
    Dump,
}

/// A single morpheme (token) in the analysis result
#[derive(Debug, Clone)]
pub struct Morpheme {
    /// Surface form (the actual text)
    pub surface: String,
    /// Word ID (token index in dictionary, used for embeddings and training)
    pub word_id: u32,
    /// Part-of-speech ID
    pub pos_id: u16,
    /// Word cost
    pub wcost: i16,
    /// Feature string (comma-separated POS info, reading, etc.)
    pub feature: String,
}

impl fmt::Display for Morpheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Main line: MeCab-compatible format
        write!(f, "{}\t{}", self.surface, self.feature)?;

        Ok(())
    }
}

/// Analysis result containing a sequence of morphemes
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// The morphemes in the analysis result
    pub morphemes: Vec<Morpheme>,
    /// Output format
    format: OutputFormat,
}

impl fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.format {
            OutputFormat::Default => {
                for morpheme in &self.morphemes {
                    writeln!(f, "{morpheme}")?;
                }
                writeln!(f, "EOS")
            }
            OutputFormat::Dump => {
                for (i, morpheme) in self.morphemes.iter().enumerate() {
                    writeln!(
                        f,
                        "[{}] {} (pos_id={}, wcost={})\t{}",
                        i, morpheme.surface, morpheme.pos_id, morpheme.wcost, morpheme.feature
                    )?;
                }
                writeln!(f, "EOS")
            }
        }
    }
}

/// The main MeCrab morphological analyzer
#[derive(Clone)]
pub struct MeCrab {
    dictionary: Arc<Dictionary>,
    output_format: OutputFormat,
}

impl MeCrab {
    /// Create a builder for configuring MeCrab
    #[must_use]
    pub fn builder() -> MeCrabBuilder {
        MeCrabBuilder::new()
    }

    /// Parse the input text and return analysis result
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails.
    pub fn parse<'b>(&self, text: &'b str) -> Result<AnalysisResult> {
        // Build the lattice
        let lattice = Lattice::build(text, &self.dictionary)?;

        // Solve using Viterbi algorithm
        let solver = ViterbiSolver::new(&self.dictionary);
        let (_, path) = solver.solve(lattice)?;

        let morphemes = path
            .into_iter()
            .map(|node| Morpheme {
                surface: node.surface,
                word_id: node.word_id,
                pos_id: node.pos_id,
                wcost: node.wcost,
                feature: node.feature,
            })
            .collect();

        Ok(AnalysisResult {
            morphemes,
            format: self.output_format,
        })
    }

    /// Parse multiple texts in parallel using Rayon
    ///
    /// This method leverages all available CPU cores for batch processing,
    /// providing significant speedup for large workloads.
    ///
    /// # Errors
    ///
    /// Returns a vector of results, where each result may be an error.
    #[cfg(feature = "parallel")]
    pub fn parse_batch(&self, texts: &[&str]) -> Vec<Result<AnalysisResult>> {
        use rayon::prelude::*;
        texts.par_iter().map(|text| self.parse(text)).collect()
    }

    /// Parse multiple texts sequentially (fallback when parallel feature is disabled)
    #[cfg(not(feature = "parallel"))]
    pub fn parse_batch(&self, texts: &[&str]) -> Vec<Result<AnalysisResult>> {
        texts.iter().map(|text| self.parse(text)).collect()
    }

    /// Add a word to the dictionary at runtime
    ///
    /// This is a key feature for production systems that need to handle
    /// new vocabulary (product names, trending terms, etc.) without restart.
    ///
    /// # Arguments
    ///
    /// * `surface` - The surface form (the actual text)
    /// * `reading` - The katakana reading
    /// * `pronunciation` - The pronunciation (often same as reading)
    /// * `wcost` - Word cost (lower = more preferred, typical: 5000-8000)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mecrab = MeCrab::new()?;
    ///
    /// // Add a new word
    /// mecrab.add_word("ChatGPT", "チャットジーピーティー", "チャットジーピーティー", 5000);
    ///
    /// // Now it will be recognized
    /// let result = mecrab.parse("ChatGPTを使う")?;
    /// ```
    pub fn add_word(&self, surface: &str, reading: &str, pronunciation: &str, wcost: i16) {
        self.dictionary
            .add_simple_word(surface, reading, pronunciation, wcost);
    }

    /// Remove a word from the overlay dictionary
    ///
    /// Returns true if the word was found and removed.
    /// Note: Only overlay words can be removed; system dictionary entries persist.
    pub fn remove_word(&self, surface: &str) -> bool {
        self.dictionary.remove_word(surface)
    }

    /// Get the number of words in the overlay dictionary
    pub fn overlay_size(&self) -> usize {
        self.dictionary.overlay_size()
    }

    /*
    /// Parse the input text and return N-best analysis results
    ///
    /// Returns multiple alternative analyses ranked by cost, useful for
    /// disambiguation and exploring alternative segmentations.
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to analyze
    /// * `n` - Number of best paths to return
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails.
    pub fn parse_nbest(&self, text: &str, n: usize) -> Result<Vec<(AnalysisResult, i64)>> {
        // Build the lattice
        let mut lattice = Lattice::build(text, &self.dictionary)?;

        // Solve using Viterbi algorithm with N-best
        let solver = ViterbiSolver::new(&self.dictionary);
        let paths = solver.solve_nbest(&mut lattice, n)?;

        // Convert paths to analysis results
        let results = paths
            .into_iter()
            .map(|(path, cost)| {
                let morphemes = path
                    .into_iter()
                    .map(|node| Morpheme {
                        surface: node.surface,
                        word_id: node.word_id,
                        pos_id: node.pos_id,
                        wcost: node.wcost,
                        feature: node.feature,
                    })
                    .collect();

                (
                    AnalysisResult {
                        morphemes,
                        format: self.output_format,
                    },
                    cost,
                )
            })
            .collect();

        Ok(results)
    }
    */
}

/// Builder for configuring a MeCrab instance.
#[derive(Debug, Default)]
pub struct MeCrabBuilder {
    dicdir: Option<PathBuf>,
    userdic: Option<PathBuf>,
    output_format: OutputFormat,
}

impl MeCrabBuilder {
    /// Create a new builder with default settings
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the dictionary directory.
    #[must_use]
    pub fn dicdir(mut self, path: Option<PathBuf>) -> Self {
        self.dicdir = path;
        self
    }

    /// Sets the user dictionary path.
    #[must_use]
    pub fn userdic(mut self, path: Option<PathBuf>) -> Self {
        self.userdic = path;
        self
    }

    /// Sets the output format.
    #[must_use]
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    }

    /// Builds the MeCrab instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the dictionary cannot be loaded.
    pub fn build(self) -> Result<MeCrab> {
        let dictionary = match self.dicdir {
            Some(dicdir) => Dictionary::load(&dicdir).boxed_local()?,
            None => return Err(Error::DictionaryNotSet),
        };

        Ok(MeCrab {
            dictionary: Arc::new(dictionary),
            output_format: self.output_format,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let builder = MeCrab::builder();
        assert!(builder.dicdir.is_none());
        assert!(builder.userdic.is_none());
        assert_eq!(builder.output_format, OutputFormat::Default);
    }

    #[test]
    fn test_expected() {
        let home = std::env::var("HOME").unwrap();

        let tagger = crate::MeCrabBuilder::new()
            .dicdir(Some(PathBuf::from(format!(
                "{home}/Projects/sribich/_/unidic-cwj-202302"
            ))))
            .output_format(OutputFormat::Default)
            .build()
            .unwrap();

        assert_eq!(tagger.parse("こんにちは").unwrap().morphemes.len(), 3);
    }
}
