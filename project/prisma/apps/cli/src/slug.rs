use std::borrow::Cow;

use deunicode::deunicode;

/// Slugify with default options.
pub fn slugify(input: &str) -> String {
    slugify_with_options(input, &Options::default())
}

/// Configurable options for slugification.
#[derive(Debug, Clone)]
pub struct Options {
    /// Word separator to use in the final slug.
    pub separator: String,
    /// Convert the result to lowercase (default: true)
    pub lowercase: bool,
    /// Trim leading/trailing separators (default: true)
    pub trim: bool,
    /// Maximum length of the slug in characters (None = unlimited)
    pub max_length: Option<usize>,
    /// Drop emoji and symbol characters before processing (default: true)
    pub drop_emoji: bool,
    /// Drop apostrophes before processing (default: true)
    pub drop_apostrophes: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            separator: "_".into(),
            lowercase: true,
            trim: true,
            max_length: None,
            drop_emoji: true,
            drop_apostrophes: true,
        }
    }
}

impl Options {
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    pub fn lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }

    pub fn trim(mut self, trim: bool) -> Self {
        self.trim = trim;
        self
    }

    pub fn max_length(mut self, max_length: Option<usize>) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn drop_emoji(mut self, drop_emoji: bool) -> Self {
        self.drop_emoji = drop_emoji;
        self
    }

    pub fn drop_apostrophes(mut self, drop_apostrophes: bool) -> Self {
        self.drop_apostrophes = drop_apostrophes;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Slugifier {
    options: Options,
}

impl Slugifier {
    pub fn new() -> Self {
        Self {
            options: Options::default(),
        }
    }

    pub fn with_options(options: Options) -> Self {
        Self { options }
    }

    pub fn options(&self) -> &Options {
        &self.options
    }

    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }

    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.options.separator = separator.into();
        self
    }

    pub fn lowercase(mut self, lowercase: bool) -> Self {
        self.options.lowercase = lowercase;
        self
    }

    pub fn trim(mut self, trim: bool) -> Self {
        self.options.trim = trim;
        self
    }

    pub fn max_length(mut self, max_length: Option<usize>) -> Self {
        self.options.max_length = max_length;
        self
    }

    pub fn slugify(&self, input: &str) -> String {
        slugify_impl(input, &self.options)
    }

    /// Slugify many inputs using the stored options.
    pub fn slugify_many<'a, I>(&self, inputs: I) -> Vec<String>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let iter = inputs.into_iter();
        let mut out = Vec::with_capacity(iter.size_hint().0);
        for s in iter {
            out.push(slugify_impl(s, &self.options));
        }
        out
    }

    /// Heuristic auto batch over a slice: picks parallel when worth it.
    pub fn slugify_many_auto(&self, inputs: &[&str]) -> Vec<String> {
        if should_use_parallel(inputs) {
            #[cfg(feature = "parallel")]
            {
                use rayon::prelude::*;
                return par_slugify_many(inputs.par_iter().copied(), &self.options);
            }
        }
        slugify_many(inputs.iter().copied(), &self.options)
    }

    /// Parallel slugify (requires `parallel` feature).
    #[cfg(feature = "parallel")]
    pub fn par_slugify_many<'a, I>(&self, inputs: I) -> Vec<String>
    where
        I: rayon::prelude::IntoParallelIterator<Item = &'a str>,
    {
        use rayon::prelude::*;
        inputs
            .into_par_iter()
            .map(|s| slugify_impl(s, &self.options))
            .collect()
    }
}

impl Default for Slugifier {
    fn default() -> Self {
        Self::new()
    }
}

pub fn slugify_with_options(input: &str, options: &Options) -> String {
    slugify_impl(input, options)
}

/// Slugify many inputs with the same options. Allocates one `String` per input.
pub fn slugify_many<'a, I>(inputs: I, options: &Options) -> Vec<String>
where
    I: IntoIterator<Item = &'a str>,
{
    let iter = inputs.into_iter();
    let mut out = Vec::with_capacity(iter.size_hint().0);
    for s in iter {
        out.push(slugify_impl(s, options));
    }
    out
}

/// Auto batch over a slice using a heuristic; uses parallel when beneficial.
pub fn slugify_many_auto(inputs: &[&str], options: &Options) -> Vec<String> {
    if should_use_parallel(inputs) {
        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            return par_slugify_many(inputs.par_iter().copied(), options);
        }
    }
    slugify_many(inputs.iter().copied(), options)
}

/// Parallel slugify (requires `parallel` feature).
#[cfg(feature = "parallel")]
pub fn par_slugify_many<'a, I>(inputs: I, options: &Options) -> Vec<String>
where
    I: rayon::prelude::IntoParallelIterator<Item = &'a str>,
{
    use rayon::prelude::*;
    inputs
        .into_par_iter()
        .map(|s| slugify_impl(s, options))
        .collect()
}

fn should_use_parallel(inputs: &[&str]) -> bool {
    let total_bytes: usize = inputs.iter().map(|s| s.len()).sum();
    let count = inputs.len();
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let byte_threshold = 100_000usize.saturating_mul(cores);
    if total_bytes >= byte_threshold {
        return true;
    }
    count >= 1_000
}

fn slugify_impl(input: &str, options: &Options) -> String {
    let pre = Cow::Borrowed(input);

    // Stage 2: optionally filter before transliteration to drop emoji/apostrophes with Cow
    let filtered = if options.drop_emoji || options.drop_apostrophes {
        filter_pre_transliteration_cow(&pre, options)
    } else {
        pre
    };

    // Stage 3: ASCII short-circuit: if already ASCII, skip transliteration
    let ascii: Cow<str> = if filtered.is_ascii() {
        filtered
    } else {
        Cow::Owned(deunicode(&filtered))
    };

    // Stage 4: fast ASCII pass to build slug with optimized capacity estimation
    let estimated_capacity = estimate_slug_capacity(&ascii, options);
    let mut builder = String::with_capacity(estimated_capacity);
    let mut prev_was_sep = false;
    let mut bytes_so_far = 0usize;
    let sep = options.separator.as_str();
    let sep_len = sep.len();
    let enforce_max = options.max_length.is_some();

    for byte in ascii.as_bytes() {
        let c = *byte as char; // ASCII only
        if c.is_ascii_alphanumeric() {
            if enforce_max && would_exceed(bytes_so_far, 1, options) {
                break;
            }
            builder.push(c);
            prev_was_sep = false;
            bytes_so_far += 1;
        } else if !prev_was_sep {
            if enforce_max && would_exceed(bytes_so_far, sep_len, options) {
                break;
            }
            builder.push_str(sep);
            prev_was_sep = true;
            bytes_so_far += sep_len;
        }
    }

    // Trim edges and lowercase if requested
    let mut text = builder;
    if options.trim {
        trim_separators(&mut text, sep);
    }
    if options.lowercase {
        text = text.to_lowercase();
    }
    text
}

// Cow-based pre-filter, avoids allocation when not needed
fn filter_pre_transliteration_cow<'a>(input: &'a Cow<'a, str>, options: &Options) -> Cow<'a, str> {
    if !options.drop_emoji && !options.drop_apostrophes {
        return input.clone();
    }
    // Check necessity first
    let needs_filtering = input.chars().any(|ch| {
        (options.drop_apostrophes && (ch == '\'' || ch == '\u{2019}'))
            || (options.drop_emoji
                && !(ch.is_alphabetic()
                    || ch.is_numeric()
                    || ch.is_whitespace()
                    || ch == '-'
                    || ch == '_'))
    });
    if !needs_filtering {
        return input.clone();
    }
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        if options.drop_apostrophes && (ch == '\'' || ch == '\u{2019}') {
            continue;
        }
        if options.drop_emoji
            && !(ch.is_alphabetic()
                || ch.is_numeric()
                || ch.is_whitespace()
                || ch == '-'
                || ch == '_')
        {
            out.push(' ');
            continue;
        }
        out.push(ch);
    }
    Cow::Owned(out)
}

/// Estimates the optimal capacity for the slug string builder.
fn estimate_slug_capacity(ascii: &str, options: &Options) -> usize {
    let input_len = ascii.len();
    let sep_len = options.separator.len();
    if let Some(max_len) = options.max_length {
        return std::cmp::min(
            input_len + (input_len / 4) * (sep_len.saturating_sub(1)),
            max_len,
        );
    }
    let estimated_separators = input_len / 4;
    let separator_overhead = estimated_separators.saturating_mul(sep_len.saturating_sub(1));
    let base_estimate = input_len + separator_overhead;
    base_estimate + (base_estimate / 8)
}

fn would_exceed(current: usize, add_len: usize, options: &Options) -> bool {
    options
        .max_length
        .map(|max_len| current + add_len > max_len)
        .unwrap_or(false)
}

fn trim_separators(text: &mut String, sep: &str) {
    while text.starts_with(sep) {
        text.drain(..sep.len());
    }
    while text.ends_with(sep) {
        let new_len = text.len() - sep.len();
        text.truncate(new_len);
    }
}
