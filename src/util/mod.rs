#![allow(non_snake_case)]

mod DataStores;
mod SPARQL;

pub use SPARQL::Query::Query as OjiQuery;
pub use SPARQL::Query::QueryUnit as OjiQueryUnit;
pub use SPARQL::Result::ResultUnit as OjiResultUnit;
pub use SPARQL::Result::Result as OjiResult;
pub use SPARQL::Result::ResultCollection as OjiResultCollection;
pub use DataStores::Graph::Graph as Graph;
pub use DataStores::TripleStore::TripleStoreRefIterator as TripleRefIter;
pub use DataStores::TripleStore::TripleStore as TripleStore;

//Delcare common resources for nested modules
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
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
