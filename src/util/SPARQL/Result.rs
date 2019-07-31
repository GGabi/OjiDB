use std::collections::HashMap;
use super::Query::*;

/* ResultUnit */
#[derive(Clone, Debug, PartialEq)]
pub enum ResultUnit {
  Val(String),
  Nil,
}
impl<'a> From<&'a str> for ResultUnit {
  fn from(s: &str) -> Self {
    match s.chars().next() {
      Some(_) => ResultUnit::Val(s.into()),
      None    => ResultUnit::Nil
    }
  }
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
  pub fn add_val(&mut self, val: ResultUnit) {
    self.values.push(val);
  }
  pub fn add_var(&mut self, var: String, val: String) {
    self.values.push(ResultUnit::Val(val));
    self.var_map.insert(var, self.values.len()-1);
  }
  pub fn get_val(&self, pos: usize) -> Option<String> {
    if self.values.len() <= pos {
      match &self.values[pos] {
        ResultUnit::Val(a) => return Some(a.clone()),
        _ => return None
      };
    }
    None
  }
  pub fn get_var(&self, var: &str) -> Option<String> {
    match self.var_map.get(var) {
      Some(&pos) => {
        match self.values[pos].clone() {
          ResultUnit::Val(val) => Some(val),
          _ => None,
        }
      },
      None => None,
    }
  }
}
#[derive(Clone, Debug)]
pub struct ResultCollection<'a> {
  pub results: Vec<Result>,
  pub query: Query<'a>,
}
impl<'a> ResultCollection<'a> {
  pub fn new() -> Self {
    ResultCollection {
      results: Vec::new(),
      query: Query::new().compile(),
    }
  }
  // pub fn from(q: Query, rs: Vec<Result>) -> Self {
  //   TODO
  // }
}
