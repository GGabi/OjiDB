
use super::{Triple, QueryTriple, TOrdering, t_order};

/*
Trait to be implemented on Vec<T>.
Forces re-allocation if and only if the
length changes by a factor of 2, within
a specified range.
*/
trait BinaryResize {
  const MAX: usize;
  const MIN: usize;
  fn try_grow(&mut self);
  fn try_shrink(&mut self);
  fn grow(&mut self);
  fn shrink(&mut self);
}
impl<T> BinaryResize for Vec<T> {
  const MAX: usize = 64;
  const MIN: usize = 8;
  fn try_grow(&mut self) {
    if self.capacity() < Self::MAX
    && self.len() == self.capacity() {
      self.grow();
    }
  }
  fn try_shrink(&mut self) {
    if self.capacity() > Self::MIN
    && self.len() <= self.capacity()/2 {
      self.shrink();
    }
  }
  fn grow(&mut self) {
    self.reserve_exact(self.capacity());
  }
  fn shrink(&mut self) {
    self.shrink_to_fit();
  }
}

/*
A data-structure that stores triples in, I hesitate to say,
the most space-efficient way possible a-la Hexastore.
*/
#[derive(Clone, Debug)]
pub struct TripleStore(pub Vec<(String, Box<Vec<(String, Box<Vec<String>>)>>)>);
impl TripleStore {
  pub fn new() -> Self {
    TripleStore(Vec::with_capacity(8))
  }
  pub fn add(&mut self, (h, m, t): Triple) {
    let heads = &mut self.0;
    if let Some((_, mids)) = heads.iter_mut().find(|(val, _)| val == &h) {
      if let Some((_, tails)) = mids.iter_mut().find(|(val, _)| val == &m) {
        if let Some(_) = tails.iter().find(|val| val == &&t) {
          //Triple already exists in TripleStore, don't add
          return
        }
        else {
          //Head and Mid exist in store, adding Tail 
          tails.try_grow();
          tails.push(t);
        }
      }
      else {
        //Head exists in store, adding Mid and Tail
        mids.try_grow();
        let mut v = Vec::with_capacity(8);
        v.push(t);
        let tail = Box::new(v);
        mids.push((m, tail));
      }
    }
    else {
      //Head, Mid and Tail do not exist in store, adding them all
      heads.try_grow();
      let mut v = Vec::with_capacity(8);
      v.push(t);
      let tail = Box::new(v);
      let mut v = Vec::with_capacity(8);
      v.push((m, tail));
      let mid = Box::new(v);
      heads.push((h, mid));
    }
  }
  pub fn erase(&mut self, (h, m, t): &Triple) {
    let heads = &mut self.0;
    //Find the pos of the head in the store
    if let Some(head_pos) = heads.iter_mut()
                                 .position(|(val, _)| val == h) {
      let mids = &mut heads[head_pos].1;
      //Find the pos of the mid in the head
      if let Some(mid_pos) = mids.iter_mut()
                                  .position(|(val, _)| val == m) {
        let tails = &mut mids[mid_pos].1;
        //Find the pos of the tail in the mid
        if let Some(tail_pos) = tails.iter_mut()
                                     .position(|val| val == t) {
          //If the triple is in the store, remove
          //  and shrink tail Vec if needed
          tails.remove(tail_pos);
          tails.try_shrink();
        }
        //If the mid now contains no tails, remove
        //  and shrink mid Vec if needed
        if tails.len() == 0 {
          mids.remove(mid_pos);
          mids.try_shrink();
        }
      }
      //If the head now contains no mids, remove
      //  and shrink head Vec if needed
      if mids.len() == 0 {
        heads.remove(head_pos);
        heads.try_shrink();
      }
    }
  }
  pub fn get_triple(&self, qt: &QueryTriple) -> Vec<Triple> {
    let heads = &self.0;
    let mut ret_v: Vec<Triple> = Vec::new();
    match qt {
      (Some(h), Some(m), Some(t)) => {
        if let Some((_, mids)) = heads.iter().find(|(val, _)| val == h) {
          if let Some((_, tails)) = mids.iter().find(|(val, _)| val == m) {
            if let Some(_) = tails.iter().find(|val| val == &t) {
              ret_v.push((h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (Some(h), Some(m), None) => {
        if let Some((_, mids)) = heads.iter().find(|(val, _)| val == h) {
          if let Some((_, tails)) = mids.iter().find(|(val, _)| val == m) {
            for t in tails.iter() {
              ret_v.push((h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (Some(h), None, None) => {
        if let Some((_, mids)) = heads.iter().find(|(val, _)| val == h) {
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
  pub fn replace(&mut self, old_t: &Triple, new_t: Triple) {
    self.erase(old_t);
    self.add(new_t);
  }
}
impl TripleStore {
  pub fn get_ordered(&self, qt: &QueryTriple, ord: &TOrdering) -> Vec<Triple> {
    let heads = &self.0;
    let mut ret_v: Vec<Triple> = Vec::new();
    match qt {
      (Some(h), Some(m), Some(t)) => {
        if let Some((_, mids)) = heads.iter().find(|(val, _)| val == h) {
          if let Some((_, tails)) = mids.iter().find(|(val, _)| val == m) {
            if let Some(_) = tails.iter().find(|val| val == &t) {
              ret_v.push(t_order((h.to_string(), m.to_string(), t.to_string()), ord));
            }
          }
        }
      },
      (Some(h), Some(m), None) => {
        if let Some((_, mids)) = heads.iter().find(|(val, _)| val == h) {
          if let Some((_, tails)) = mids.iter().find(|(val, _)| val == m) {
            for t in tails.iter() {
              ret_v.push(t_order((h.to_string(), m.to_string(), t.to_string()), ord));
            }
          }
        }
      },
      (Some(h), None, None) => {
        if let Some((_, mids)) = heads.iter().find(|(val, _)| val == h) {
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              ret_v.push(t_order((h.to_string(), m.to_string(), t.to_string()), ord));
            }
          }
        }
      },
      (None, None, None) => {
        for (h, mids) in heads.iter() {
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              ret_v.push(t_order((h.to_string(), m.to_string(), t.to_string()), ord));
            }
          }
        }
      },
      _ => {},
    };
    ret_v
  }
}
impl IntoIterator for TripleStore {
  type Item = (String, String, String);
  type IntoIter = TripleStoreIterator;
  fn into_iter(self) -> Self::IntoIter {
    TripleStoreIterator {
      store: self,
      curr_head: 0,
      curr_mid: 0,
      curr_tail: 0,
    }
  }
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


pub struct TripleStoreIterator {
  store: TripleStore,
  curr_head: usize,
  curr_mid:  usize,
  curr_tail: usize,
}
impl Iterator for TripleStoreIterator {
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