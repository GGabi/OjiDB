pub mod Graph;
pub mod TripleStore;
pub mod Queries;
pub mod Results;

// use Graph::Graph;
// use TripleStore::TripleStore;
use Queries::{Query, QueryUnit};
use Results::{Result, ResultUnit, ResultCollection};

/*
Definitions of the 3 possible orderings of Triples returned from
search queries in the graph, where:
S = Subject
P = Predicate
O = Object
*/
pub enum TOrdering {
  SPO,
  POS,
  OSP,
}
fn t_order(t: Triple, curr_ordering: &TOrdering) -> Triple {
  match &curr_ordering {
    POS => {
      (t.2.to_string(),
       t.0.to_string(),
       t.1.to_string())  
    },
    OSP => {
      (t.1.to_string(),
       t.2.to_string(),
       t.0.to_string())
    },
    _ => {
      t.clone()
    },
  }
}

/*
Data types to reduce verbosity.
All instances of None in Querys indicate required values.
*/
pub type Triple = (String, String, String);
pub type QueryTriple = (Option<String>, Option<String>, Option<String>);
pub type QueryChain<'a>  = &'a[Option<String>];
type Double = (String, String);
type QueryDouble = (Option<String>, Option<String>);