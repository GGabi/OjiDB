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

/* Oji Query */

/*************************
*
* QueryUnit
*
*************************/
#[derive(Clone, Debug)]
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

/*************************
*
* Query
*
*************************/
// #[derive(Clone, Debug)]
// pub enum Query {
//   Null,
//   Single(QueryUnit, Ordering),
//   Double(QueryUnit, QueryUnit, [Ordering; 2]),
//   Triple(QueryUnit, QueryUnit, QueryUnit),
//   // Chain(Vec<QueryUnit>, Vec<Ordering>),
// }
// //Builders
// impl Query {
//   fn make_single(val: QueryUnit, ord: Ordering) -> Self {
//     if let QueryUnit::Nil = val {
//       return Query::Null
//     }
//     Query::Single(val, ord)
//   }
//   fn make_double(head: QueryUnit, tail: QueryUnit, ord: [Ordering; 2]) -> Self {
//     match (&head, &tail) {
//       (QueryUnit::Nil, _) => Self::make_single(tail, ord[1].clone()),
//       (_, QueryUnit::Nil) => Self::make_single(head, ord[0].clone()),
//       _ => Query::Double(head, tail, ord),
//     }
//   }
//   fn make_triple(head: QueryUnit, mid: QueryUnit, tail: QueryUnit) -> Self {
//     use Ordering::{S, P, O};
//     match (&head, &mid, &tail) {
//       (QueryUnit::Nil, _, _) => Self::make_double(mid, tail, [P, O]),
//       (_, QueryUnit::Nil, _) => Self::make_double(head, tail, [S, O]),
//       (_, _, QueryUnit::Nil) => Self::make_double(head, mid, [S, P]),
//       _ => Query::Triple(head, mid, tail),
//     }
//   }
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
// }
// impl Query {
//   pub fn from(vals: &[QueryUnit]) -> Self {
//     use Ordering::{S, P, O};
//     match vals.len() {
//       0 => Query::Null,
//       1 => Self::make_single(vals[0].clone(), S),
//       2 => Self::make_double(vals[0].clone(), vals[1].clone(), [S, P]),
//       3 => Self::make_triple(vals[0].clone(), vals[1].clone(), vals[2].clone()),
//       _ => Query::Null,
//       // _ => Self::make_chain(vals),
//     }
//   }
//   pub fn from_str(vals: &[&str]) -> Self {
//     use Ordering::{S, P, O};
//     match vals.len() {
//       0 => Query::Null,
//       1 => Self::make_single(QueryUnit::from(vals[0]), S),
//       2 => Self::make_double(QueryUnit::from(vals[0]),
//                               QueryUnit::from(vals[1]),
//                               [S, P]),
//       3 => Self::make_triple(QueryUnit::from(vals[0]),
//                               QueryUnit::from(vals[1]),
//                               QueryUnit::from(vals[2])
//                               ),
//       _ => Query::Null,
//       // _ => {
//       //   let mut chain: Vec<QueryUnit> = Vec::new();
//       //   for v in vals {
//       //     chain.push(QueryUnit::from(v));
//       //   }
//       //   Self::make_chain(&chain)
//       // },
//     }
//   }
// }
