
#[derive(Clone, Debug)]
pub enum QueryUnit {
  Val(String),
  Var(String),
  Anon,
  Ignore,
}
impl QueryUnit {
  pub fn from(s: &str) -> Self {
    if s.len() == 0 {
      return QueryUnit::Ignore
    }
    let first_char = s.chars().next().unwrap();
    match first_char {
      '$' => { return QueryUnit::Var(s[1..].into()) },
      '?' => { return QueryUnit::Anon },
      _ => { return QueryUnit::Val(s.into()) },
    }
  }
}
#[derive(Clone, Debug)]
pub enum Query {
  Null,
  Single(QueryUnit),
  Double(QueryUnit, QueryUnit),
  Triple(QueryUnit, QueryUnit, QueryUnit),
  Chain(Vec<QueryUnit>),
}
impl Query {
  fn from(vals: &[QueryUnit]) -> Option<Self> {
    match vals.len() {
      0 => None,
      1 => Some(Query::Single(vals[0].clone())),
      2 => Some(Query::Double(vals[0].clone(), vals[1].clone())),
      3 => Some(Query::Triple(vals[0].clone(), vals[1].clone(), vals[2].clone())),
      _ => Some(Query::Chain(vals.to_vec())),
    }
  }
  pub fn from_str(vals: &[&str]) -> Option<Self> {
    match vals.len() {
      0 => None,
      1 => Some(Query::Single(QueryUnit::from(vals[0]))),
      2 => Some(Query::Double(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1])
                              )),
      3 => Some(Query::Triple(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1]),
                              QueryUnit::from(vals[2])
                              )),
      _ => {
        let mut chain: Vec<QueryUnit> = Vec::new();
        for v in vals {
          chain.push(QueryUnit::from(v));
        }
        Some(Query::Chain(chain))
      },
    }
  }
}