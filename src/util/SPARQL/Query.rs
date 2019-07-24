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
    Ordering, Triple, Double, QueryDouble, QueryTriple, QueryChain
  }
};
use std::collections::HashMap;

/* Query Unit */
#[derive(Clone, Debug, PartialEq)]
pub enum QueryUnit {
  Val(String),
  Var(String),
  Nil,
}
impl QueryUnit {
  pub fn from(s: &str) -> Self {
    if let Some(c) = s.chars().next() {
      if c == '$' {
        return QueryUnit::Var(s[1..].into())
      }
      else {
        return QueryUnit::Val(s.into())
      }
    }
    QueryUnit::Nil
  }
}

/* Result */
#[derive(Clone, Debug, PartialEq)]
pub enum ResultUnit {
  Value(String),
  Ignore,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Result {
  pub values: Vec<ResultUnit>,
  pub var_map: HashMap<String, usize>,
}
impl Result {
  pub fn new() -> Self {
    Result {
      values: Vec::new(),
      var_map: HashMap::new(),
    }
  }
  pub fn add_anon(&mut self, val: ResultUnit) {
    self.values.push(val);
  }
  pub fn add_var(&mut self, var: String, val: String) {
    self.values.push(ResultUnit::Value(val));
    self.var_map.insert(var, self.values.len()-1);
  }
  pub fn get_var(&self, var: &str) -> Option<String> {
    match self.var_map.get(var) {
      Some(&pos) => {
        match self.values[pos].clone() {
          ResultUnit::Value(val) => Some(val),
          _ => None,
        }
      },
      None => None,
    }
  }
}
#[derive(Clone, Debug)]
pub struct ResultCollection {
  pub results: Vec<Result>,
  // pub query: Query<'a>,
}
impl ResultCollection {
  pub fn new() -> Self {
    ResultCollection {
      results: Vec::new(),
      // query: Query {
      //   graph: &Graph::new(),
      //   vars: Vec::new(),
      //   conds: Vec::new(),
      // },
    }
  }
  // pub fn from(q: Query, rs: Vec<Result>) -> Self {
  //   ResultCollection {
  //     results: rs,
  //     query: q,
  //   }
  // }
}

/* Query */
#[derive(Clone, Debug)]
pub struct Query<'a> {
  graph: &'a Graph,
  vars: Vec<QueryUnit>,
  conds: Vec<(QueryUnit, QueryUnit, QueryUnit)>,
}
impl<'a> Query<'a> {
  pub fn new() -> QueryBase {
    QueryBase
  }
  pub fn fetch(self) -> ResultCollection {
    use QueryUnit::{Val, Var, Nil};
    let mut rc = ResultCollection::new();
    /* Actually start processing now */
    let mut q1: Option<String>;
    let mut q2: Option<String>;
    let mut q3: Option<String>;
    match &self.conds[0].0 {
      Val(a) => { q1 = Some(a.clone()); },
      Var(_) => { q1 = None; },
      Nil => { q1 = None; },
    };
    match &self.conds[0].1 {
      Val(b) => { q2 = Some(b.clone()); },
      Var(_) => { q2 = None; },
      Nil => { q2 = None; },
    };
    match &self.conds[0].2 {
      Val(b) => { q3 = Some(b.clone()); },
      Var(_) => { q3 = None; },
      Nil => { q3 = None; },
    };
    let query_res = self.graph.get_triple(&(q1, q2, q3));
    if query_res.len() > 0 {
      for i in 0..query_res.len() {
        let mut r = Result::new();
        match &self.conds[0].0 {
          Val(a) => { r.add_anon(ResultUnit::Value(a.to_string())); },
          Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
          Nil => { r.add_anon(ResultUnit::Ignore); },
        }
        match &self.conds[0].1 {
          Val(b) => { r.add_anon(ResultUnit::Value(b.to_string())); },
          Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
          Nil => { r.add_anon(ResultUnit::Ignore); },
        }
        match &self.conds[0].2 {
          Val(c) => { r.add_anon(ResultUnit::Value(c.to_string())); },
          Var(c) => { r.add_var(c.to_string(), query_res[i].2.clone()); },
          Nil    => { r.add_anon(ResultUnit::Ignore); },
        }
        rc.results.push(r);
      }
    }
    rc
  }
}

/* Query Builders */
pub struct QueryBase;
impl QueryBase {
  pub fn from(self, g: &Graph) -> QueryFrom {
    QueryFrom {
      graph: g,
    }
  }
}
pub struct QueryFrom<'a> {
  graph: &'a Graph,
}
impl<'a> QueryFrom<'a> {
  pub fn select(self, vars: &'a[&str]) -> QuerySelect<'a> {
    let qunits: Vec<QueryUnit> = vars.to_vec()
                     .into_iter()
                     .map(|x| QueryUnit::from(x))
                     .collect();
    QuerySelect {
      graph: self.graph,
      vars: qunits,
    }
  }
  pub fn compile(self) -> Query<'a> {
    Query {
      graph: self.graph,
      vars: Vec::new(),
      conds: Vec::new(),
    }
  }
  // pub fn fetch() -> Result {
  //   //todo
  // }
}
pub struct QuerySelect<'a> {
  graph: &'a Graph,
  vars: Vec<QueryUnit>,
}
impl<'a> QuerySelect<'a> {
  pub fn filter(self, conds: &[(&str, &str, &str)]) -> Query<'a> {
    let qconds: Vec<(QueryUnit, QueryUnit, QueryUnit)>
      = conds.to_vec()
             .into_iter()
             .map(|(x, y, z)| (QueryUnit::from(x), QueryUnit::from(y), QueryUnit::from(z)))
             .filter(|(x, y, z)| {
               for a in [x, y, z].iter() {
                 if let QueryUnit::Var(_) = a {
                   if !self.vars.contains(a) {
                     panic!("Undeclared variable in query!");
                   }
                 }
               }
               true
             })
             .collect();
    Query {
      graph: self.graph,
      vars: self.vars,
      conds: qconds,
    }
  }
  pub fn compile(self) -> Query<'a> {
    Query {
      graph: self.graph,
      vars: self.vars,
      conds: Vec::new(),
    }
  }
  // pub fn fetch() -> Result {
  //   //todo
  // }
}

