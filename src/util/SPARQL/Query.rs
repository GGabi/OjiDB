use super::super::DataStores::Graph::Graph;
use super::Result::*;

/* Query Unit */
#[derive(Clone, Debug, PartialEq)]
pub enum QueryUnit {
  Val(String),
  Var(String),
  Nil,
}
impl<'a> From<&'a str> for QueryUnit {
  fn from(s: &str) -> Self {
    match s.chars().next() {
      Some('$') => QueryUnit::Var(s[1..].into()),
      Some(_)   => QueryUnit::Val(s.into()),
      None      => QueryUnit::Nil,
    }
  }
}

/* Query */
#[derive(Clone, Debug)]
pub struct Query<'a> {
  graph: Option<&'a Graph>,
  vars: Vec<QueryUnit>,
  conds: Vec<(QueryUnit, QueryUnit, QueryUnit)>,
}
impl<'a> Query<'a> {
  pub fn new() -> QueryBase {
    QueryBase
  }
  pub fn fetch(self) -> ResultCollection<'a> {
    use QueryUnit::{Val, Var, Nil};
    let mut rc = ResultCollection::new();
    if let None = self.graph {
      return rc
    }
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
    let query_res = self.graph.unwrap().get_triple(&(q1, q2, q3));
    if query_res.len() > 0 {
      for i in 0..query_res.len() {
        let mut r = Result::new();
        match &self.conds[0].0 {
          Val(a) => { r.add_val(ResultUnit::Val(a.to_string())); },
          Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
          Nil => { r.add_val(ResultUnit::Nil); },
        }
        match &self.conds[0].1 {
          Val(b) => { r.add_val(ResultUnit::Val(b.to_string())); },
          Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
          Nil => { r.add_val(ResultUnit::Nil); },
        }
        match &self.conds[0].2 {
          Val(c) => { r.add_val(ResultUnit::Val(c.to_string())); },
          Var(c) => { r.add_var(c.to_string(), query_res[i].2.clone()); },
          Nil    => { r.add_val(ResultUnit::Nil); },
        }
        rc.results.push(r);
      }
    }
    rc
  }
}

/* Query Builders */
pub struct QueryBase;
impl<'a> QueryBase {
  pub fn from(self, g: &Graph) -> QueryFrom {
    QueryFrom {
      graph: g,
    }
  }
  pub fn compile(self) -> Query<'a> {
    Query {
      graph: None,
      vars: Vec::new(),
      conds: Vec::new(),
    }
  }
  pub fn fetch(self) -> ResultCollection<'a> {
    self.compile().fetch()
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
      graph: Some(self.graph),
      vars: Vec::new(),
      conds: Vec::new(),
    }
  }
  pub fn fetch(self) -> ResultCollection<'a> {
    self.compile().fetch()
  }
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
      graph: Some(self.graph),
      vars: self.vars,
      conds: qconds,
    }
  }
  pub fn compile(self) -> Query<'a> {
    Query {
      graph: Some(self.graph),
      vars: self.vars,
      conds: Vec::new(),
    }
  }
  pub fn fetch(self) -> ResultCollection<'a> {
    self.compile().fetch()
  }
}
