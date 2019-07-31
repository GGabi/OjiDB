
use super::Ordering;

/*************************
*
* QueryUnit
*
*************************/
#[derive(Clone, Debug)]
pub enum QueryUnit {
  Val(String), //Literal, Lit
  Var(String), //Named, Nam
  Anon,        //Unnamed
  Ignore,      //Void
}
impl QueryUnit {
  pub fn from(s: &str) -> Self {
    if s.len() == 0 {
      return QueryUnit::Ignore
    }
    let first_char = s.chars().next().unwrap();
    match first_char {
      '_' => { return QueryUnit::Ignore },
      '$' => { return QueryUnit::Var(s[1..].into()) },
      '?' => { return QueryUnit::Anon },
      _ => { return QueryUnit::Val(s.into()) },
    }
  }
}

/* Std Traits */
impl PartialEq for QueryUnit {
  fn eq(&self, other: &Self) -> bool {
    use QueryUnit::*;
    match self {
      Val(a) => {
        if let Val(b) = other {
          return a == b
        }
        else {
          return false
        }
      },
      Var(a) => {
        if let Var(b) = other {
          return a == b
        }
        else {
          return false
        }
      },
      Anon => {
        if let Anon = other {
          return true
        }
        else {
          return false
        }
      },
      Ignore => {
        if let Ignore = other {
          return true
        }
        else {
          return false
        }
      },
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
  Single(QueryUnit, Ordering),
  Double(QueryUnit, QueryUnit, [Ordering; 2]),
  Triple(QueryUnit, QueryUnit, QueryUnit, [Ordering; 3]),
  // Chain(Vec<QueryUnit>, Vec<Ordering>),
}
//Builders
impl Query {
  fn make_single(val: QueryUnit, ord: Ordering) -> Self {
    if let QueryUnit::Ignore = val {
      return Query::Null
    }
    Query::Single(val, ord)
  }
  fn make_double(head: QueryUnit, tail: QueryUnit, ord: [Ordering; 2]) -> Self {
    match (&head, &tail) {
      (QueryUnit::Ignore, _) => Self::make_single(tail, ord[1].clone()),
      (_, QueryUnit::Ignore) => Self::make_single(head, ord[0].clone()),
      _ => Query::Double(head, tail, ord),
    }
  }
  fn make_triple(head: QueryUnit, mid: QueryUnit, tail: QueryUnit) -> Self {
    use Ordering::{S, P, O};
    match (&head, &mid, &tail) {
      (QueryUnit::Ignore, _, _) => Self::make_double(mid, tail, [P, O]),
      (_, QueryUnit::Ignore, _) => Self::make_double(head, tail, [S, O]),
      (_, _, QueryUnit::Ignore) => Self::make_double(head, mid, [S, P]),
      _ => Query::Triple(head, mid, tail, [S, P, O]),
    }
  }
  // fn make_chain(chain: &[QueryUnit]) -> Self {
  //   let filtered_chain: Vec<&QueryUnit> = chain.into_iter()
  //                                             .filter(|x| if let QueryUnit::Ignore = x {return true} else {return false})
  //                                             .collect();
  //   match filtered_chain.len() {
  //     0 => Query::Null,
  //     1 => Self::make_single(filtered_chain[0].clone()),
  //     2 => Self::make_double(filtered_chain[0].clone(), filtered_chain[1].clone()),
  //     3 => Self::make_triple(filtered_chain[0].clone(), filtered_chain[1].clone(), filtered_chain[2].clone()),
  //     _ => Query::Chain(filtered_chain.into_iter().map(|x| x.clone()).collect()),
  //   }
  // }
}
impl Query {
  pub fn from(vals: &[QueryUnit]) -> Self {
    use Ordering::{S, P};
    match vals.len() {
      0 => Query::Null,
      1 => Self::make_single(vals[0].clone(), S),
      2 => Self::make_double(vals[0].clone(), vals[1].clone(), [S, P]),
      3 => Self::make_triple(vals[0].clone(), vals[1].clone(), vals[2].clone()),
      _ => Query::Null,
      // _ => Self::make_chain(vals),
    }
  }
  pub fn from_str(vals: &[&str]) -> Self {
    use Ordering::{S, P};
    match vals.len() {
      0 => Query::Null,
      1 => Self::make_single(QueryUnit::from(vals[0]), S),
      2 => Self::make_double(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1]),
                              [S, P]),
      3 => Self::make_triple(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1]),
                              QueryUnit::from(vals[2])
                              ),
      _ => Query::Null,
      // _ => {
      //   let mut chain: Vec<QueryUnit> = Vec::new();
      //   for v in vals {
      //     chain.push(QueryUnit::from(v));
      //   }
      //   Self::make_chain(&chain)
      // },
    }
  }
}

/* Std Traits */
impl PartialEq for Query {
  fn eq(&self, other: &Self) -> bool {
    use Query::*;
    match self {
      Null => {
        if let Null = other {
          return true
        }
        false
      },
      Single(a, ord_a) => {
        if let Single(x, ord_x) = other {
          return a == x && ord_a == ord_x
        }
        false
      },
      Double(a, b, ord_a) => {
        if let Double(x, y, ord_x) = other {
          return a == x && b == y && ord_a == ord_x
        }
        false
      },
      Triple(a, b, c, ord_a) => {
        if let Triple(x, y, z, ord_x) = other {
          return a == x && b == y && c == z && ord_a == ord_x
        }
        false
      },
    }
  }
}