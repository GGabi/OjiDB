/*
SPARQL Query Builder Pattern:
let results = Query.from().select().where(){.fetch() | .compile()};

from(): Takes a single graph
select(): Takes either the Enum “All” or a slice of variable names
where(): Only valid for queries with variable names, not All
        Takes a slice of triples that take either a variable name from select() or a string literal
compile(): Converts a Query builder to the appropriate finalised Query struct and returns it
fetch(): Compile the Query, sends it off and returns the result from the database

Struct QueryBase {
  fn from() -> QueryFrom;
}
Struct QueryFrom {
  fn select() -> QuerySelect;
  fn compile() -> Query;
  fn fetch() -> DBResult;
  datasource: Graph;
}
Struct QuerySelect {
  fn where() -> Query;
  fn compile() -> Query;
  fn fetch() -> DBResult;
  datasource: Graph;
  variables: Vec<variables>
}
Struct Query {
  fn fetch() -> DBResult;
  datasource: Graph;
  variables: Vec<String>;
  conditions: Vec<(String, String, String)>;
}
*/

use super::{
  super::{
    DataStores::Graph::Graph,
    Ordering, Triple, Double, QueryDouble, QueryTriple, QueryChain,
    Results::{Result, ResultUnit, ResultCollection}
  }
};

pub struct QueryBase;
impl QueryBase {
  pub fn from(g: &Graph) -> QueryFrom {
    QueryFrom {
      graph: g,
    }
  }
}
pub struct QueryFrom<'a> {
  graph: &'a Graph,
}
impl<'a> QueryFrom<'a> {
  pub fn select(self, vars: &'a[String]) -> QuerySelect<'a> {
    QuerySelect {
      graph: self.graph,
      vars: vars.to_vec(),
    }
  }
  // pub fn compile() -> Query<'a> {
  //   //todo
  // }
  // pub fn fetch() -> Result {
  //   //todo
  // }
}
pub struct QuerySelect<'a> {
  graph: &'a Graph,
  vars: Vec<String>,
}
impl<'a> QuerySelect<'a> {
  pub fn r#where(self, conds: &[(String, String, String)]) -> Query<'a> {
    Query {
      graph: self.graph,
      vars: self.vars,
      conds: conds.to_vec(),
    }
  }
  // pub fn compile() -> Query<'a> {
  //   //todo
  // }
  // pub fn fetch() -> Result {
  //   //todo
  // }
}
pub struct Query<'a> {
  graph: &'a Graph,
  vars: Vec<String>,
  conds: Vec<(String, String, String)>,
}
impl<'a> Query<'a> {
  pub fn new() -> QueryBase {
    QueryBase
  }
  // pub fn fetch() -> Result {
  //   //todo
  // }
}