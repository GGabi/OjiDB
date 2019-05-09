pub mod DataStores;
pub mod Queries;
pub mod Results;

pub use DataStores::Graph::Graph as Graph;
pub use DataStores::TripleStore::TripleStore as TripleStore;
pub use Queries::Query as DBQuery;
pub use Queries::QueryUnit as DBQueryUnit;
pub use Results::Result as DBResult;
pub use Results::ResultUnit as DBResultUnit;
pub use Results::ResultCollection as DBResultCollection;

//Delcare common resources for nested modules
pub enum TOrdering {
  SPO,
  POS,
  OSP,
}

type Triple = (String, String, String);
type QueryTriple = (Option<String>, Option<String>, Option<String>);
type QueryChain<'a>  = &'a[Option<String>];
type Double = (String, String);
type QueryDouble = (Option<String>, Option<String>);
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