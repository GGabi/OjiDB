
use std::collections::{HashMap, HashSet};

type Triple = (String, String, String);
type QueryTriple = (Option<String>, Option<String>, Option<String>);
type QueryChain<'a>  = &'a[Option<String>];
type Double = (String, String);
type QueryDouble = (Option<String>, Option<String>);

/*
Trait to be implemented on Vec<T>.
Forces re-allocation if and only if the
length changes by a factor of 2, within
a specified range.
*/
// trait BinaryResize {
//   const MAX: usize;
//   const MIN: usize;
//   fn try_grow(&mut self);
//   fn try_shrink(&mut self);
//   fn grow(&mut self);
//   fn shrink(&mut self);
// }
// impl<T> BinaryResize for Vec<T> {
//   const MAX: usize = 64;
//   const MIN: usize = 8;
//   fn try_grow(&mut self) {
//     if self.capacity() < Self::MAX
//     && self.len() == self.capacity() {
//       self.grow();
//     }
//   }
//   fn try_shrink(&mut self) {
//     if self.capacity() > Self::MIN
//     && self.len() <= self.capacity()/2 {
//       self.shrink();
//     }
//   }
//   fn grow(&mut self) {
//     self.reserve_exact(self.capacity());
//   }
//   fn shrink(&mut self) {
//     self.shrink_to_fit();
//   }
// }

/* TripleStore */
#[derive(Clone, Debug, PartialEq)]
pub struct TripleStore(pub HashMap<String, Box<HashMap<String, Box<HashSet<String>>>>>);
impl TripleStore {
  pub fn new() -> Self {
    TripleStore(Vec::with_capacity(8))
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
  // pub fn get(&self, qc: QueryChain) -> Vec<Vec<String>> {
  //   let mut ret_v: Vec<Vec<String>> = Vec::new();
  //   match qc.len() {
  //     1 => {
  //       for h in self.get_single(&qc[0]).iter() {
  //         ret_v.push(vec!(h.to_string()));
  //       }
  //     },
  //     2 => {
  //       for (h, m) in self.get_double(&(qc[0].clone(), qc[1].clone())).iter() {
  //         ret_v.push(vec!(h.to_string(), m.to_string()));
  //       }
  //     },
  //     3 => {
  //       for (h, m, t) in self.get_triple(&(qc[0].clone(), qc[1].clone(), qc[2].clone())).iter() {
  //         ret_v.push(vec!(h.to_string(), m.to_string(), t.to_string()));
  //       }
  //     },
  //     _ => {},
  //   };
  //   ret_v
  // }
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
  // pub fn get_double(&self, qd: &QueryDouble) -> Vec<Double> {
  //   let heads = &self.0;
  //   let mut ret_v: Vec<Double> = Vec::new();
  //   match qd {
  //     (Some(h), Some(t)) => {
  //       if let Some((_, tails)) = heads.iter().find(|(val, _)| val == h) {
  //         if let Some(_) = tails.iter().find(|(val, _)| val == t) {
  //           ret_v.push((h.to_string(), t.to_string()));
  //         }
  //       }
  //     },
  //     (Some(h), None) => {
  //       if let Some((_, tails)) = heads.iter().find(|(val, _)| val == h) {
  //         for (t, _) in tails.iter() {
  //           ret_v.push((h.to_string(), t.to_string()));
  //         }
  //       }
  //     },
  //     (None, Some(t)) => {
  //       for (h, tails) in heads.iter() {
  //         if let Some((t, _)) = tails.iter().find(|(val, _)| val == t) {
  //           ret_v.push((h.to_string(), t.to_string()));
  //         }
  //       }
  //     },
  //     (None, None) => {
  //       for (h, tails) in heads.iter() {
  //         for (t, _) in tails.iter() {
  //           ret_v.push((h.to_string(), t.to_string()));
  //         }
  //       }
  //     },
  //   };
  //   ret_v
  // }
  // pub fn get_single(&self, qs: &Option<String>) -> Vec<String> {
  //   let heads = &self.0;
  //   let mut ret_v: Vec<String> = Vec::new();
  //   match qs {
  //     Some(h) => {
  //       if let Some((_, _)) = heads.iter().find(|(val, _)| val == h) {
  //         ret_v.push(h.to_string());
  //       }
  //     },
  //     None => {
  //       for (h, _) in heads.iter() {
  //         ret_v.push(h.to_string());
  //       }
  //     },
  //   }
  //   ret_v
  // }
  // pub fn replace(&mut self, old_t: &Triple, new_t: Triple) {
  //   self.erase(old_t);
  //   self.add(new_t);
  // }
}
impl<'a> IntoIterator for &'a TripleStore {
  type Item = (String, String, String);
  type IntoIter = TripleStoreRefIterator<'a>;
  fn into_iter(self) -> Self::IntoIter {
    TripleStoreRefIterator {
      store: &self,
      curr_head: 0,
      curr_mid: 0,
      curr_tail: 0,
    }
  }
}

/* Iterator */
#[derive(Clone, Debug, PartialEq)]
pub struct TripleStoreRefIterator<'a> {
  pub store: &'a TripleStore,
  pub curr_head: usize,
  pub curr_mid:  usize,
  pub curr_tail: usize,
}
impl<'a> Iterator for TripleStoreRefIterator<'a> {
  type Item = (String, String, String);
  fn next(&mut self) -> Option<Self::Item> {

    if self.curr_head == self.store.0.len() {
      return None
    }

    let head = self.store.0[self.curr_head].0.to_string();
    let mid = self.store.0[self.curr_head].1[self.curr_mid].0.to_string();
    let tail = self.store.0[self.curr_head].1[self.curr_mid].1[self.curr_tail].to_string();

    if self.curr_tail == self.store.0[self.curr_head].1[self.curr_mid].1.len()-1 {
      if self.curr_mid == self.store.0[self.curr_head].1.len()-1 {
          self.curr_head += 1;
          self.curr_mid = 0;
          self.curr_tail = 0;
      }
      else {
        self.curr_mid += 1;
        self.curr_tail = 0;
      }
    }
    else {
      self.curr_tail += 1;
    }
    return Some((head, mid, tail))
  }
} 