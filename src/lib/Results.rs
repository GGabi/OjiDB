
use std::collections::HashMap;
use super::Queries::{Query, QueryUnit};

/*************************
*
* Result
*
*************************/
#[derive(Clone, Debug)]
pub enum ResultUnit {
  Value(String),
  Ignore,
}
#[derive(Clone, Debug)]
pub struct Result {
  pub values: Vec<ResultUnit>,
  var_map: HashMap<String, usize>,
}
impl IntoIterator for Result {
  type Item = ResultUnit;
  type IntoIter = ResultIterator;
  fn into_iter(self) -> Self::IntoIter {
    ResultIterator {
      store: self,
      curr_pos: 0,
    }
  }
}
pub struct ResultIterator {
  store: Result,
  curr_pos: usize,
}
impl Iterator for ResultIterator {
  type Item = ResultUnit;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_pos == self.store.values.len() {
      return None
    }
    else {
      let ret_val = Some(self.store.values[self.curr_pos].clone());
      self.curr_pos += 1;
      return ret_val
    }
  }
} 
impl<'a> IntoIterator for &'a Result {
  type Item = ResultUnit;
  type IntoIter = ResultRefIterator<'a>;
  fn into_iter(self) -> Self::IntoIter {
    ResultRefIterator {
      store: &self,
      curr_pos: 0,
    }
  }
}
pub struct ResultRefIterator<'a> {
  store: &'a Result,
  curr_pos: usize,
}
impl<'a> Iterator for ResultRefIterator<'a> {
  type Item = ResultUnit;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_pos == self.store.values.len() {
      return None
    }
    else {
      let ret_val = Some(self.store.values[self.curr_pos].clone());
      self.curr_pos += 1;
      return ret_val
    }
  }
} 
impl std::ops::Deref for Result {
  type Target = Vec<ResultUnit>;
  fn deref(&self) -> &Self::Target {
    &self.values
  }
}
impl std::ops::DerefMut for Result {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.values
    }
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
  pub fn iter(&self) -> ResultRefIterator {
    ResultRefIterator {
      store: &self,
      curr_pos: 0,
    }
  }
}

/*************************
*
* ResultCollection
*
*************************/
#[derive(Clone, Debug)]
pub struct ResultCollection {
  pub results: Vec<Result>,
  pub query: Query,
}
pub struct ResultCollectionIterator {
  results: ResultCollection,
  curr_pos: usize,
}
impl Iterator for ResultCollectionIterator {
  type Item = Result;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_pos == self.results.results.len() {
      return None
    }
    else {
      let ret_val = Some(self.results.results[self.curr_pos].clone());
      self.curr_pos += 1;
      return ret_val
    }
  }
} 
impl<'a> IntoIterator for &'a ResultCollection {
  type Item = Result;
  type IntoIter = ResultCollectionRefIterator<'a>;
  fn into_iter(self) -> Self::IntoIter {
    ResultCollectionRefIterator {
      results: &self,
      curr_pos: 0,
    }
  }
}
pub struct ResultCollectionRefIterator<'a> {
  results: &'a ResultCollection,
  curr_pos: usize,
}
impl<'a> Iterator for ResultCollectionRefIterator<'a> {
  type Item = Result;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_pos == self.results.results.len() {
      return None
    }
    else {
      let ret_val = Some(self.results.results[self.curr_pos].clone());
      self.curr_pos += 1;
      return ret_val
    }
  }
} 
impl std::ops::Deref for ResultCollection {
  type Target = Vec<Result>;
  fn deref(&self) -> &Self::Target {
    &self.results
  }
}
impl std::ops::DerefMut for ResultCollection {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.results
    }
}
impl ResultCollection {
  pub fn new() -> Self {
    ResultCollection {
      results: Vec::new(),
      query: Query::Null,
    }
  }
  pub fn from(q: Query, rs: Vec<Result>) -> Self {
    ResultCollection {
      results: rs,
      query: q,
    }
  }
  pub fn iter(&self) -> ResultCollectionRefIterator {
    ResultCollectionRefIterator {
      results: &self,
      curr_pos: 0,
    }
  }
}