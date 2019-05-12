mod DataStores;
mod Queries;
mod Results;

pub use DataStores::Graph::Graph as Graph;
pub use DataStores::TripleStore::TripleStore as TripleStore;
pub use Queries::Query as DBQuery;
pub use Queries::QueryUnit as DBQueryUnit;
pub use Results::Result as DBResult;
pub use Results::ResultUnit as DBResultUnit;
pub use Results::ResultCollection as DBResultCollection;

//Delcare common resources for nested modules
pub enum Ordering {
  SPO,
  POS,
  OSP,
  SP,
  PO,
  OS,
  S,
  P,
  O,
}

type Triple = (String, String, String);
type QueryTriple = (Option<String>, Option<String>, Option<String>);
type QueryChain<'a>  = &'a[Option<String>];
type Double = (String, String);
type QueryDouble = (Option<String>, Option<String>);
