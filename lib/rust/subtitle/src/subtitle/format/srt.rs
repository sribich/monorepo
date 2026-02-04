//! A parser for the SubRip (srt) file format.
//!
//! # Structure:
//!
//!   1. A numeric counter indicating the index of the subtitle
//!   2. Start and end time of the subtitle in the following format HH:MM:SS,MSS --> HH:MM:SS,MSS
//!   3. Subtitle text in one or more lines
//!   4. A blank line indicating the end of the subtitle
use std::{
    fs::{read, read_to_string},
    iter::once,
    path::{Path, PathBuf},
};

use chumsky::{
    error::Simple,
    primitive::just,
    text::{self, whitespace},
    Parser,
};
use color_eyre::eyre::Result;
use thiserror::Error;

use super::util::{number_i64, split_bom};
use crate::subtitle::{timing::*, SubtitleFileInterface};

#[derive(Error, Debug)]
pub enum SrtError {
    #[error("Expected SubRip index at {line_number:?}, found '{line:?}'")]
    ExpectedIndex { line_number: usize, line: String },
    #[error("Expected SubRip timestamp at {line_number:?}, found '{line:?}'")]
    ExpectedTimestamp { line_number: usize, line: String },
}

enum SrtParserState {
    Break,
    Index(i64),
    Timing(i64, TimeSpan),
    Dialog(i64, TimeSpan, Vec<String>),
}

///
#[derive(Clone, Debug)]
pub struct SrtFile {
    path: PathBuf,

    pub lines: Vec<SrtLine>,
}

#[derive(Clone, Debug)]
pub struct SrtLine {
    pub timespan: TimeSpan,
    pub index: i64,
    pub text: Vec<String>,
}

impl SubtitleFileInterface for SrtFile {}

impl SrtFile {
    pub fn new(path: &impl AsRef<Path>) -> SrtFile {
        SrtFile {
            path: path.as_ref().to_owned(),
            lines: vec![],
        }
    }

    ///
    ///
    /// # BNF
    ///
    /// // Tokens
    /// <MINUS> ::= "-"
    /// <PLUS>  ::= "+"
    /// <UTF-8> ::= UTF8
    ///
    /// <digit>   ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    ///
    /// // Lexicals
    /// <integer> ::= ['-'] <digit> <digit>*
    ///
    /// // Phrases
    /// <srt-file> ::= <block>+
    ///
    ///
    ///
    /// <block> ::= <index> <EOL> <timing> <EOL> <text>
    /// <index> ::= <integer>
    /// <timing> ::= (<integer> ':' <integer> ':' <integer> '.' <integer>)
    ///              | <integer> ':' <integer> ':' <integer> ',' <integer>
    /// <text> ::= {<UTF-8> <EOL>}
    ///
    pub fn parse(&mut self) -> Result<()> {
        // , raw_data: &str

        let raw_data = read_to_string(&self.path).map_err(|e| {
            // println!("{:#?} {:?}", e, &self.path);
            e
        })?;

        let (_, raw_data) = split_bom(&raw_data[..]);

        let mut result: Vec<SrtLine> = Vec::new();
        let mut state: SrtParserState = SrtParserState::Break;

        // We add `once("")` so the last entry is not ignored
        for (line_num, line) in raw_data.lines().chain(once("")).enumerate() {
            state = match state {
                SrtParserState::Break => {
                    if line.trim().is_empty() {
                        SrtParserState::Break
                    } else {
                        SrtParserState::Index(number_i64().parse(line).unwrap())
                        // SrtParserState::Index(Self::parse_index_line(line_num, line)?)
                    }
                }
                SrtParserState::Index(index) => SrtParserState::Timing(
                    index,
                    Self::parse_timestamp_line()
                        .parse(line)
                        .map_err(|e| {
                            println!("{:?} {:?}", e, self.path);
                            0
                        })
                        .unwrap(),
                ),
                SrtParserState::Timing(index, timespan) => {
                    Self::expect_dialog_line(
                        line_num,
                        line,
                        &mut result,
                        index,
                        timespan,
                        Vec::new(),
                    )
                    // text.push();
                    // SrtParserState::Dialog(index, timespan, vec![ line.trim().to_string() ])
                }
                SrtParserState::Dialog(index, timespan, text) => {
                    Self::expect_dialog_line(line_num, line, &mut result, index, timespan, text)
                }
            };
        }

        self.lines = result;

        Ok(())
        // Ok(SrtFile { lines: result })
    }

    /*
    fn parse_index_line(line_num: usize, line: &str) -> Result<i64> {
        line
            .trim()
            .parse::<i64>()
            .with_context(|| SrtError::ExpectedIndex {
                line_number: line_num,
                line: line.to_string(),
            })
    }
    */

    fn parse_timestamp() -> impl Parser<char, TimePoint, Error = Simple<char>> {
        number_i64()
            .then(just(":"))
            .then(number_i64())
            .then(just(":"))
            .then(number_i64())
            .then(just(","))
            .then(number_i64())
            .map(
                |((((((hours, _), minutes), _), seconds), _), milliseconds)| {
                    TimePoint::from_components(hours, minutes, seconds, milliseconds)
                },
            )
    }

    fn parse_timestamp_line() -> impl Parser<char, TimeSpan, Error = Simple<char>> {
        whitespace()
            .ignored()
            .ignore_then(SrtFile::parse_timestamp())
            .then_ignore(whitespace().ignored())
            .then_ignore(just("-->"))
            .then_ignore(whitespace().ignored())
            .then(SrtFile::parse_timestamp())
            .then_ignore(whitespace().ignored())
            .map(|(start, end)| TimeSpan::new(start, end))
    }

    fn expect_dialog_line(
        _line_num: usize,
        line: &str,
        result: &mut Vec<SrtLine>,
        index: i64,
        timespan: TimeSpan,
        mut text: Vec<String>,
    ) -> SrtParserState {
        if line.trim().is_empty() {
            result.push(SrtLine {
                index,
                timespan,
                text,
            });
            SrtParserState::Break
        } else {
            text.push(line.trim().to_string());
            SrtParserState::Dialog(index, timespan, text)
        }
    }
}

/// impl SrtFile {
/// Creates a .srt file from raw SrtLine data.
// pub fn create(lines: Vec<SrtLine>)
/// }

#[cfg(test)]
mod tests {
    #[test]
    fn create_srt_test() {
        let _a: i32 = 1;
    }
}
