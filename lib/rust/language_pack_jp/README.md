# 



# The goal of this repository is to do the following:

1. Parse Japanese into a format to gain understanding and split text in a manner to learn via creating flashcards, seeing grammar points, etc.
2. Join audio books and ebooks.

# 2.

In a way we must first start with the second option because it deals with the most granular 

# 1.





```
[
    {
        original_text: "",
        original_parse: [
            MECAB_TOKENS_AS_ORIGINALLY_DEFINED
        ],
        timestamp: ...,
        preprocessed_text: "",
        preprocess_ranges: [
            (start, end) = (orig_start, orig_end) -> any preprocessing has the output ranges mapped onto the original ranges.
                                                     formatting should not be destructive, just normalization.
        ],
        preprocess_parse_tree: [
            MECAB_TOKENS... -> mecab tokens should point to ranges
                               // PREPROCESS_PARSE_TREE IS WHAT WILL BE USED TO DIFF AUDIO BOOKS AND EBOOKS.
        ],
        transformed: [
            ...transforms should only be run on the ebook. They contain the groupings of text.
        ],
    }
]
```



```AUDIOBOOK_TRANSCRIPTION
[
    
]
```

TODO(sr): We need to fuzz/miri this crate since it has unsafe
