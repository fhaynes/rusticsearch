pub mod standard;
pub mod ngram;

use serde_json;
use serde_json::value::ToJson;
use kite::token::Token;

use analysis::ngram_generator::Edge;
use analysis::filters::lowercase::LowercaseFilter;
use analysis::tokenizers::standard::StandardTokenizer;
use analysis::tokenizers::ngram::NGramTokenizer;


/// Defines a tokenizer
///
/// You can use this to define a tokenizer before having to bind it to any data
///
/// # Examples
///
/// ```
/// use kite::{Term, Token};
/// use kite::analysis::tokenizers::TokenizerSpec;
///
/// let standard_tokenizer = TokenizerSpec::Standard;
/// let token_stream = standard_tokenizer.initialise("Hello, world!");
///
/// let tokens = token_stream.collect::<Vec<Token>>();
///
/// assert_eq!(tokens, vec![
///     Token { term: Term::from_string("Hello"), position: 1 },
///     Token { term: Term::from_string("world"), position: 2 },
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TokenizerSpec {
    Standard,
    Lowercase,
    NGram {
        min_size: usize,
        max_size: usize,
        edge: Edge,
    }
}


impl TokenizerSpec {
    pub fn initialise<'a>(&self, input: &'a str) -> Box<Iterator<Item=Token> + 'a> {
        match *self {
            TokenizerSpec::Standard => {
                Box::new(StandardTokenizer::new(input))
            }
            TokenizerSpec::Lowercase => {
                Box::new(LowercaseFilter::new(Box::new(StandardTokenizer::new(input))))
            }
            TokenizerSpec::NGram{min_size, max_size, edge} => {
                Box::new(NGramTokenizer::new(input, min_size, max_size, edge))
            }
        }
    }
}


impl ToJson for TokenizerSpec {
    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        match *self {
            TokenizerSpec::Standard => {
                Ok(json!({
                    "type": "standard",
                }))
            }
            TokenizerSpec::Lowercase => {
                Ok(json!({
                    "type": "lowercase",
                }))
            }
            TokenizerSpec::NGram{min_size, max_size, edge} => {
                match edge {
                    Edge::Left => {
                        Ok(json!({
                            "type": "edgeNGram",
                            "side": "front",
                            "min_gram": min_size,
                            "max_gram": max_size,
                        }))
                    }
                    Edge::Right => {
                        Ok(json!({
                            "type": "edgeNGram",
                            "side": "back",
                            "min_gram": min_size,
                            "max_gram": max_size,
                        }))
                    }
                    Edge::Neither => {
                        Ok(json!({
                            "type": "ngram",
                            "min_gram": min_size,
                            "max_gram": max_size,
                        }))
                    }
                }
            }
        }
    }
}
