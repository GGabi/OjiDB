mod DataStores;
mod Queries;
mod Results;
mod SPARQL;

pub use SPARQL::Query::Query as SparQuery;
pub use DataStores::Graph::Graph as Graph;
pub use DataStores::TripleStore::TripleStoreRefIterator as TripleRefIter;
pub use DataStores::TripleStore::TripleStore as TripleStore;
pub use Queries::Query as OjiQuery;
pub use Queries::QueryUnit as OjiQueryUnit;
pub use Results::Result as OjiResult;
pub use Results::ResultUnit as OjiResultUnit;
pub use Results::ResultCollection as OjiResultCollection;

//Delcare common resources for nested modules
#[derive(Clone, Debug)]
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
