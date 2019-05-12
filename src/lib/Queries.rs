
/*************************
*
* QueryUnit
*
*************************/
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

/*************************
*
* Query
*
*************************/
#[derive(Clone, Debug)]
pub enum Query {
  Null,
  Single(QueryUnit),
  Double(QueryUnit, QueryUnit),
  Triple(QueryUnit, QueryUnit, QueryUnit),
  Chain(Vec<QueryUnit>),
}
//Makers
impl Query {
  fn make_single(val: QueryUnit) -> Self {
    if let QueryUnit::Ignore = val {
      return Query::Null
    }
    Query::Single(val)
  }
  fn make_double(head: QueryUnit, tail: QueryUnit) -> Self {
    match (&head, &tail) {
      (QueryUnit::Ignore, _) => Self::make_single(tail),
      (_, QueryUnit::Ignore) => Self::make_single(head),
      _ => Query::Double(head, tail),
    }
  }
  fn make_triple(head: QueryUnit, mid: QueryUnit, tail: QueryUnit) -> Self {
    match (&head, &mid, &tail) {
      (QueryUnit::Ignore, _, _) => Self::make_double(mid, tail),
      (_, QueryUnit::Ignore, _) => Self::make_double(head, tail),
      (_, _, QueryUnit::Ignore) => Self::make_double(head, mid),
      _ => Query::Triple(head, mid, tail),
    }
  }
  fn make_chain(chain: &[QueryUnit]) -> Self {
    let filtered_chain: Vec<&QueryUnit> = chain.into_iter()
                                              .filter(|x| if let QueryUnit::Ignore = x {return true} else {return false})
                                              .collect();
    match filtered_chain.len() {
      0 => Query::Null,
      1 => Self::make_single(filtered_chain[0].clone()),
      2 => Self::make_double(filtered_chain[0].clone(), filtered_chain[1].clone()),
      3 => Self::make_triple(filtered_chain[0].clone(), filtered_chain[1].clone(), filtered_chain[2].clone()),
      _ => Query::Chain(filtered_chain.into_iter().map(|x| x.clone()).collect()),
    }
  }
}
impl Query {
  pub fn from(vals: &[QueryUnit]) -> Self {
    match vals.len() {
      0 => Query::Null,
      1 => Self::make_single(vals[0].clone()),
      2 => Self::make_double(vals[0].clone(), vals[1].clone()),
      3 => Self::make_triple(vals[0].clone(), vals[1].clone(), vals[2].clone()),
      _ => Self::make_chain(vals),
    }
  }
  pub fn from_str(vals: &[&str]) -> Self {
    match vals.len() {
      0 => Query::Null,
      1 => Self::make_single(QueryUnit::from(vals[0])),
      2 => Self::make_double(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1])
                              ),
      3 => Self::make_triple(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1]),
                              QueryUnit::from(vals[2])
                              ),
      _ => {
        let mut chain: Vec<QueryUnit> = Vec::new();
        for v in vals {
          chain.push(QueryUnit::from(v));
        }
        Self::make_chain(&chain)
      },
    }
  }
}