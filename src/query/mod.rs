pub mod parser;
pub mod matching;
pub mod ranking;

use term::Term;


#[derive(Debug, PartialEq)]
pub enum TermMatcher {
    Exact,
    Prefix,
}


#[derive(Debug, PartialEq)]
pub enum Query {
    MatchAll {
        boost: f64,
    },
    MatchNone,
    MatchTerm {
        field: String,
        term: Term,
        boost: f64,
        matcher: TermMatcher,
    },
    Bool {
        must: Vec<Query>,
        must_not: Vec<Query>,
        should: Vec<Query>,
        filter: Vec<Query>,
        minimum_should_match: i32,
        boost: f64,
    },
    DisjunctionMax {
        queries: Vec<Query>,
        boost: f64,
    },
    BoostScore {
        query: Box<Query>,
        boost: f64,
    }
}
