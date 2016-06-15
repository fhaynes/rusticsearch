use query::Query;
use query::parser::QueryParseError;


pub fn build_and_query(queries: Vec<Query>) -> Result<Query, QueryParseError> {
    if queries.len() == 0 {
        // TODO: raise error
        Ok(Query::MatchNone)
    } else if queries.len() == 1 {
        // Single query, unpack it from queries array and return it
        for query in queries {
            return Ok(query);
        }

        unreachable!();
    } else {
        Ok(Query::And {
            queries: queries,
        })
    }
}


pub fn build_or_query(queries: Vec<Query>) -> Result<Query, QueryParseError> {
    if queries.len() == 0 {
        // TODO: raise error
        Ok(Query::MatchNone)
    } else if queries.len() == 1 {
        // Single query, unpack it from queries array and return it
        for query in queries {
            return Ok(query);
        }

        unreachable!();
    } else {
        Ok(Query::Or {
            queries: queries,
        })
    }
}


pub fn build_disjunction_max_query(queries: Vec<Query>) -> Result<Query, QueryParseError> {
    if queries.len() == 0 {
        // TODO: raise error
        Ok(Query::MatchNone)
    } else if queries.len() == 1 {
        // Single query, unpack it from queries array and return it
        for query in queries {
            return Ok(query);
        }

        unreachable!();
    } else {
        Ok(Query::DisjunctionMax {
            queries: queries,
        })
    }
}


pub fn build_score_query(query: Query, mul: f64, add: f64) -> Query {
    if mul == 1.0f64 && add == 0.0f64 {
        // This score query won't have any effect
        return query;
    }

    Query::Score {
        query: Box::new(query),
        mul: mul,
        add: add,
    }
}