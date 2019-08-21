
use std::collections::{HashMap, HashSet};

type Triple = (String, String, String);
type QueryTriple = (Option<String>, Option<String>, Option<String>);
type QueryChain<'a>  = &'a[Option<String>];
type Double = (String, String);
type QueryDouble = (Option<String>, Option<String>);

/* TripleStore */
#[derive(Clone, Debug, PartialEq)]
pub struct TripleStore(pub HashMap<String, Box<HashMap<String, Box<HashSet<String>>>>>);
impl TripleStore {
  pub fn new() -> Self {
    TripleStore(HashMap::new())
  }
  pub fn add(&mut self, (h, m, t): Triple) {
    let heads = &mut self.0;
    if heads.contains_key(&h) {
      let mids = &mut heads[&h];
      if mids.contains_key(&m) {
        let tails = &mut mids[&m];
        if tails.contains(&t) {
          /* Triple already exists in TripleStore, don't add */
          return
        }
        else {
          /* Head and Mid exist in store, adding Tail */
          tails.insert(t);
        }
      }
      else {
        /* Head exists in store, adding Mid and Tail */
        mids.insert(m, Box::new([t].iter()
                                   .cloned()
                                   .collect()));
      }
    }
    else {
      /* Head, Mid and Tail do not exist in store, adding them all */
      heads.insert(h, Box::new([(m, Box::new([t].iter()
                                                .cloned()
                                                .collect()))
                               ].iter()
                                .cloned()
                                .collect()));
    }
  }
  pub fn erase(&mut self, (h, m, t): &Triple) {
    let heads = &mut self.0;
    if heads.contains_key(h) {
      let mids = &mut heads[h];
      if mids.contains_key(m) {
        let tails = &mut mids[m];
        if tails.contains(t) {
          /* If the triple is in the store, remove
               and shrink tail Vec if needed */
          tails.remove(t);
        }
        /* If the mid now contains no tails, remove
             and shrink mid Vec if needed */
        if tails.len() == 0 {
          mids.remove(m);
        }
      }
      /* If the head now contains no mids, remove
          and shrink head Vec if needed */
      if mids.len() == 0 {
        heads.remove(h);        
      }
    }
  }
  pub fn get(&self, qc: QueryChain) -> Vec<Vec<String>> {
    let mut ret_v: Vec<Vec<String>> = Vec::new();
    match qc.len() {
      1 => {
        for h in self.get_single(&qc[0]).iter() {
          ret_v.push(vec!(h.to_string()));
        }
      },
      2 => {
        for (h, m) in self.get_double(&(qc[0].clone(), qc[1].clone())).iter() {
          ret_v.push(vec!(h.to_string(), m.to_string()));
        }
      },
      3 => {
        for (h, m, t) in self.get_triple(&(qc[0].clone(), qc[1].clone(), qc[2].clone())).iter() {
          ret_v.push(vec!(h.to_string(), m.to_string(), t.to_string()));
        }
      },
      _ => {},
    };
    ret_v
  }
  pub fn get_triple(&self, qt: &QueryTriple) -> Vec<Triple> {
    let mut ret_v: Vec<Triple> = Vec::new();
    let heads = &self.0;
    match qt {
      (Some(h), Some(m), Some(t)) => {
        if heads.contains_key(h) {
          let mids = &heads[h];
          if mids.contains_key(m) {
            if mids[m].contains(t) {
              ret_v.push((h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (Some(h), Some(m), None) => {
        if heads.contains_key(h) {
          let mids = &heads[h];
          if mids.contains_key(m) {
            for t in mids[m].iter() {
              ret_v.push((h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (Some(h), None, None) => {
        if heads.contains_key(h) {
          let mids = &heads[h];
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              ret_v.push((h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (None, None, None) => {
        for (h, mids) in heads.iter() {
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              ret_v.push((h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      _ => {},
    };
    ret_v
  }
  pub fn get_double(&self, qd: &QueryDouble) -> Vec<Double> {
    let mut ret_v: Vec<Double> = Vec::new();
    let heads = &self.0;
    match qd {
      (Some(h), Some(t)) => {
        if heads.contains_key(h) {
          if heads[h].contains_key(t) {
            ret_v.push((h.to_string(), t.to_string()));
          }
        }
      },
      (Some(h), None) => {
        if heads.contains_key(h) {
          for (t, _) in heads[h].iter() {
            ret_v.push((h.to_string(), t.to_string()));
          }
        }
      },
      (None, Some(t)) => {
        for (h, tails) in heads.iter() {
          if tails.contains_key(t) {
            ret_v.push((h.to_string(), t.to_string()));
          }
        }
      },
      (None, None) => {
        for (h, tails) in heads.iter() {
          for (t, _) in tails.iter() {
            ret_v.push((h.to_string(), t.to_string()));
          }
        }
      },
    };
    ret_v
  }
  pub fn get_single(&self, qs: &Option<String>) -> Vec<String> {
    let mut ret_v: Vec<String> = Vec::new();
    let heads = &self.0;
    match qs {
      Some(h) => {
        if heads.contains_key(h) {
          ret_v.push(h.to_string());
        }
      },
      None => {
        for (h, _) in heads.iter() {
          ret_v.push(h.to_string());
        }
      },
    }
    ret_v
  }
  pub fn replace(&mut self, old_t: &Triple, new_t: Triple) {
    self.erase(old_t);
    self.add(new_t);
  }
}
impl<'a> IntoIterator for &'a TripleStore {
  type Item = (String, String, String);
  type IntoIter = TripleStoreRefIterator<'a>;
  fn into_iter(self) -> Self::IntoIter {
    TripleStoreRefIterator {
      store: &self,
      head_iter: self.0.iter(),
      mid_iter:  None,
      tail_iter: None,
      curr_head: None,
      curr_mid:  None,
      curr_tail: None,
      is_fresh: true,
    }
  }
}

/* Iterator */
pub struct TripleStoreRefIterator<'a> {
  store: &'a TripleStore,
  // head_iter: Box<dyn Iterator<Item = (String, Box<HashMap<String, Box<HashSet<String>>>>)>>,
  head_iter: std::collections::hash_map::Iter<'a, String, Box<HashMap<String, Box<HashSet<String>>>>>,
  mid_iter:  Option<std::collections::hash_map::Iter<'a, String, Box<HashSet<String>>>>,
  tail_iter: Option<std::collections::hash_set::Iter<'a, String>>,
  curr_head: Option<(&'a String, &'a Box<HashMap<String, Box<HashSet<String>>>>)>,
  curr_mid:  Option<(&'a String, &'a Box<HashSet<String>>)>,
  curr_tail: Option<&'a String>,
  is_fresh: bool, // Have we processed our first item yet?
}
impl<'a> Iterator for TripleStoreRefIterator<'a> {
  type Item = (String, String, String);
  fn next(&mut self) -> Option<Self::Item> {

    if self.is_fresh {
      self.is_fresh = false;
      self.curr_head = self.head_iter.next();
      if self.curr_head == None {
        return None
      }
      self.mid_iter = Some(self.curr_head.unwrap().1.iter());
      self.curr_mid = self.mid_iter.unwrap().next();
      self.tail_iter = Some(self.curr_mid.unwrap().1.iter());
      self.curr_tail = self.tail_iter.unwrap().next();
    }
    else if self.curr_head == None {
      return None
    }

    let head = self.curr_head.unwrap().0.clone();
    let mid  = self.curr_mid.unwrap().0.clone();
    let tail = self.curr_tail.unwrap().clone();

    self.curr_tail = self.tail_iter.unwrap().next();
    if self.curr_tail == None {
      self.curr_mid = self.mid_iter.unwrap().next();
      if self.curr_mid == None {
        self.curr_head = self.head_iter.next();
        if self.curr_head == None {
          return Some((head, mid, tail))
        }
        else {
          self.mid_iter = Some(self.curr_head.unwrap().1.iter());
          self.curr_mid = self.mid_iter.unwrap().next();
        }
      }
      self.tail_iter = Some(self.curr_mid.unwrap().1.iter());
      self.curr_tail = self.tail_iter.unwrap().next();
    }

    return Some((head, mid, tail))
  }
} 