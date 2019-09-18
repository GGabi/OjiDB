use super::super::DataStores::Graph::Graph;
use super::Result::{ResultUnit, ResultCollection};

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
  graph: &'a Graph,
  vars:  Vec<QueryUnit>,
  conds: Vec<[QueryUnit; 3]>,
}
impl<'a> Query<'a> {
  pub fn new() -> QueryBuilder<'a> {
    QueryBuilder {
      graph: None,
      vars:  None,
      conds: None,
    }
  }
  pub fn fetch(self) -> ResultCollection<'a> {
    use QueryUnit::{Val, Var, Nil};
    let mut rc = ResultCollection::new();
    /* Start processing now */
    let q1 = match &self.conds[0][0] {
      Val(a) => Some(a.clone()),
      _      => None,
    };
    let q2 = match &self.conds[0][1] {
      Val(b) => Some(b.clone()),
      _      => None,
    };
    let q3 = match &self.conds[0][2] {
      Val(b) => Some(b.clone()),
      _      => None,
    };
    let query_res = self.graph.get_triple(&(q1, q2, q3));
    if query_res.len() > 0 {
      for i in 0..query_res.len() {
        let mut r = super::Result::Result::new();
        match &self.conds[0][0] {
          Val(a) => { r.add_val(ResultUnit::Val(a.to_string())); },
          Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
          Nil => { r.add_val(ResultUnit::Nil); },
        }
        match &self.conds[0][1] {
          Val(b) => { r.add_val(ResultUnit::Val(b.to_string())); },
          Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
          Nil => { r.add_val(ResultUnit::Nil); },
        }
        match &self.conds[0][2] {
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
impl<'a> Query<'a> {
  fn fetch_2(self) -> ResultCollection<'a> {
    use QueryUnit::{Val, Var, Nil};
    let mut rc = ResultCollection::new();
    /* Start processing now */
    /* Grab all the triples */
    for query in &self.conds {
      let q1 = match &query[0] {
        Val(a) => Some(a.clone()),
        _      => None,
      };
      let q2 = match &query[1] {
        Val(b) => Some(b.clone()),
        _      => None,
      };
      let q3 = match &query[2] {
        Val(c) => Some(c.clone()),
        _      => None,
      };
      let query_res = self.graph.get_triple(&(q1, q2, q3));
      if query_res.len() > 0 {
        for i in 0..query_res.len() {
          let mut r = super::Result::Result::new();
          match &query[0] {
            Val(a) => { r.add_val(ResultUnit::Val(a.to_string())); },
            Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
            Nil    => { r.add_val(ResultUnit::Nil); },
          }
          match &query[1] {
            Val(b) => { r.add_val(ResultUnit::Val(b.to_string())); },
            Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
            Nil    => { r.add_val(ResultUnit::Nil); },
          }
          match &query[2] {
            Val(c) => { r.add_val(ResultUnit::Val(c.to_string())); },
            Var(c) => { r.add_var(c.to_string(), query_res[i].2.clone()); },
            Nil    => { r.add_val(ResultUnit::Nil); },
          }
          rc.results.push(r);
        }
      }
    }
    /* Filter the triples based on conditions for variables */
    /* TODO: Provide support for the OR operator
             when a user has multiple conditions.*/
    /*
      Gabe likes James
      Gabe hates Kieran
      James likes John

      Query: $name1 likes $name2

      valid_triples: [0, 0, 0]
      current_var_vals: (x, y, z)
      previous_var_vals: vec<[x, y, z]>

      for each enumerated result:
        if still considered valid:
          
      for each condition:
        for each enumerated result:
          if still considered valid:
            try get_var with current condition's var names and insert into curr_var_vals
            if all of these get_vars result in None then skip this triple (the get_triple didn't come from these variables)

    */
    let mut _valid_triples = vec![true; rc.results.len()];
    let mut _curr_var_vals = [String::new(), String::new(), String::new()];
    let mut _prev_var_vals: Vec<[String; 3]> = Vec::new();
    print!("Yeet");
    /* Return the final collection of results */
    rc
  }
}

/* Query Builders */
pub struct QueryBuilder<'a> {
  graph: Option<&'a Graph>,
  vars:  Option<Vec<QueryUnit>>,
  conds: Option<Vec<[QueryUnit; 3]>>,
}
impl<'a> QueryBuilder<'a> {
  pub fn new() -> Self {
    QueryBuilder {
      graph: None,
      vars: None,
      conds: None,
    }
  }
  pub fn from(self, g: &'a Graph) -> Self {
    if let None = self.graph {
      QueryBuilder {
        graph: Some(g),
        vars: None,
        conds: None,
      }
    }
    else {
      panic!("Query already assosciated with a Graph.");
    }
  }
  pub fn select(self, vars: &'a [&str]) -> Self {
    if let None = self.vars {
      let qunits: Vec<QueryUnit> = vars.to_vec()
                                       .into_iter()
                                       .map(|x| QueryUnit::from(x))
                                       .collect();
      QueryBuilder {
        graph: self.graph,
        vars: Some(qunits),
        conds: None,
      }
    }
    else {
      panic!("Query already has vars.");
    }
  }
  pub fn filter(self, conds: &[[&str; 3]]) -> QueryBuilder<'a> {
    if let None = self.conds {
      let qconds: Vec<[QueryUnit; 3]>
        = conds.into_iter()
              .map(|[s, p, o]| [QueryUnit::from(*s), QueryUnit::from(*p), QueryUnit::from(*o)])
              .filter(|triple| {
                for a in triple.iter() {
                  match (a, &self.vars) {
                    (QueryUnit::Var(_), Some(vars_vec)) => {
                      if !vars_vec.contains(a) {
                        panic!("Undeclared variable in query!");
                      }
                    },
                    _ => {},
                  }
                }
                true
              })
              .collect();
      QueryBuilder {
        graph: self.graph,
        vars: self.vars,
        conds: Some(qconds),
      }
    }
    else {
      panic!("Query already has filter conditions.");
    }
  }
  pub fn compile(self) -> Query<'a> {
    match (self.graph, self.vars, self.conds) {
      (Some(g), Some(vs), Some(cs)) => {
        Query {
          graph: g,
          vars: vs,
          conds: cs,
        }
      },
      (None, _, _) => panic!("Cannot compile incomplete query, expected .from()"),
      (_, None, _) => panic!("Cannot compile incomplete query, expected .select()"),
      (_, _, None) => panic!("Cannot compile incomplete query, expected .filter()"),
    }
  }
  pub fn fetch(self) -> ResultCollection<'a> {
    self.compile().fetch()
  }
}