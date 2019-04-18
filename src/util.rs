
// OjiD
// A web that wards off, rather than causing, data related nightmares

/*
Trait to be implemented on Vec<T>.
Forces re-allocation if and only if the
length changes by a factor of 2, within
a specified range.
*/
trait BinaryResize {
  const max: usize;
  const min: usize;
  fn try_grow(&mut self);
  fn try_shrink(&mut self);
  fn grow(&mut self);
  fn shrink(&mut self);
}
impl<T> BinaryResize for Vec<T> {
  const max: usize = 64;
  const min: usize = 8;
  fn try_grow(&mut self) {
    if self.capacity() < Self::max
    && self.len() == self.capacity() {
      self.grow();
    }
  }
  fn try_shrink(&mut self) {
    if self.capacity() > Self::min
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
Definitions of the 3 possible orderings of Triples returned from
search queries in the graph, where:
S = Subject
P = Predicate
O = Object
*/
enum TOrdering {
  SPO,
  POS,
  OSP,
}

/*
Triple to be used exclusively for querying the database.
All instances of None indicate the values to be searched for.
*/
pub type QueryTriple = (Option<String>, Option<String>, Option<String>);

/*
A tuple of 3 Strings
Type aliad reduces verbosity of almost all later code
*/
#[derive(Clone, Debug)]
pub struct Triple(String, String, String);
impl std::convert::From<(&str, &str, &str)> for Triple {
  fn from(x: (&str, &str, &str)) -> Self {
    Triple(x.0.to_string(), x.1.to_string(), x.2.to_string())
  }
}
impl Triple {
  fn order(&self, curr_ordering: &TOrdering) -> Self {
    match &curr_ordering {
      POS => {
        Triple(self.2.to_string(),
             self.0.to_string(),
             self.1.to_string()
             )  
      },
      OSP => {
        Triple(self.1.to_string(),
             self.2.to_string(),
             self.0.to_string()
             )
      },
      _ => {
        self.clone()
      },
    }
  }
}

/*
A data-structure that stores triples in, I hesitate to say,
the most space-efficient way possible a-la Hexastore.
*/
#[derive(Clone, Debug)]
struct TripleStore(Vec<(String, Box<Vec<(String, Box<Vec<String>>)>>)>);
impl TripleStore {
  fn new() -> Self {
    TripleStore(Vec::with_capacity(8))
  }
  fn add(&mut self, t: &Triple) {
    if let Some((_, mids)) = self.0.iter_mut().find(|(val, _)| val == &t.0) {
      if let Some((_, tails)) = mids.iter_mut().find(|(val, _)| val == &t.1) {
        if let Some(_) = tails.iter().find(|val| val == &&t.2) {
          return
        }
        else {
          tails.try_grow();
          tails.push(t.2.to_string());
        }
      }
      else {
        mids.try_grow();
        let mut v = Vec::with_capacity(8);
        v.push(t.2.to_string());
        let mut tail = Box::new(v);
        mids.push((t.1.to_string(), tail));
      }
    }
    else {
      self.0.try_grow();
      let mut v = Vec::with_capacity(8);
      v.push(t.2.to_string());
      let mut tail = Box::new(v);
      let mut v = Vec::with_capacity(8);
      v.push((t.1.to_string(), tail));
      let mut mid = Box::new(v);
      self.0.push((t.0.to_string(), mid));
    }
  }
  fn erase(&mut self, t: &Triple) {
    if let Some(head_pos) = self.0.iter_mut().position(|(val, _)| val == &t.0 ) {
      if let Some(mid_pos) = self.0[head_pos]
                  .1.iter_mut()
                  .position(|(val, _)| val == &t.1 ) {
        if let Some(tail_pos) = self.0[head_pos]
                    .1[mid_pos]
                    .1.iter_mut()
                    .position(|val| val == &t.2 ) {
          self.0[head_pos]
            .1[mid_pos]
            .1.remove(tail_pos);
          self.0[head_pos].1[mid_pos].1.try_shrink();
        }
        if self.0[head_pos].1[mid_pos].1.len() == 0 {
          self.0[head_pos]
            .1.remove(mid_pos);
          self.0[head_pos].1.try_shrink();
        }
      }
      if self.0[head_pos].1.len() == 0 {
        self.0.remove(head_pos);
        self.0.try_shrink();
      }
    }
  }
  fn get(&self, qt: &QueryTriple) -> Vec<Triple> {
    let mut v: Vec<Triple> = Vec::new();
    match qt {
      (Some(h), Some(m), Some(t)) => {
        if let Some((_, mids)) 
          = self.0.iter()
              .find(|(val, _)| val == h) {
          if let Some((_, tails))
            = mids.iter()
                .find(|(val, _)| val == m) {
            if let Some(_)
              = tails.iter()
                   .find(|val| val == &t) {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (Some(h), Some(m), None) => {
        if let Some((_, mids)) 
          = self.0.iter()
              .find(|(val, _)| val == h) {
          if let Some((_, tails))
            = mids.iter()
                .find(|(val, _)| val == m) {
            for t in tails.iter() {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (Some(h), None, None) => {
        if let Some((_, mids)) 
          = self.0.iter()
              .find(|(val, _)| val == h) {
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      (None, None, None) => {
        for (h, mids) in self.0.iter() {
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()));
            }
          }
        }
      },
      _ => {},
    };
    v
  }
  fn replace(&mut self, old_t: Triple, new_t: Triple) {
    self.erase(&old_t);
    self.add(&new_t);
  }
}
impl TripleStore {
  fn get_ordered(&self, qt: &QueryTriple, ord: TOrdering) -> Vec<Triple> {
    let mut v: Vec<Triple> = Vec::new();
    match qt {
      (Some(h), Some(m), Some(t)) => {
        if let Some((_, mids)) 
          = self.0.iter()
              .find(|(val, _)| val == h) {
          if let Some((_, tails))
            = mids.iter()
                .find(|(val, _)| val == m) {
            if let Some(_)
              = tails.iter()
                   .find(|val| val == &t) {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()).order(&ord));
            }
          }
        }
      },
      (Some(h), Some(m), None) => {
        if let Some((_, mids)) 
          = self.0.iter()
              .find(|(val, _)| val == h) {
          if let Some((_, tails))
            = mids.iter()
                .find(|(val, _)| val == m) {
            for t in tails.iter() {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()).order(&ord));
            }
          }
        }
      },
      (Some(h), None, None) => {
        if let Some((_, mids)) 
          = self.0.iter()
              .find(|(val, _)| val == h) {
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()).order(&ord));
            }
          }
        }
      },
      (None, None, None) => {
        for (h, mids) in self.0.iter() {
          for (m, tails) in mids.iter() {
            for t in tails.iter() {
              v.push(Triple(h.to_string(), m.to_string(), t.to_string()).order(&ord));
            }
          }
        }
      },
      _ => {},
    };
    v
  }
}

//I am Gabe, Gabe likes Rust
//I --> am --> Gabe --> likes --> Rust
//[a, b, ..., y, z]
//get(a, b, c)
//c, d, e

/*
A data-structure that sacrifices space for fast data access
via storing 3 versions of the same "Triple data" in
unique orderings inspired by Hexastore.
*/
#[derive(Clone, Debug)]
pub struct Web {
  spo: TripleStore,
  pos: TripleStore,
  osp: TripleStore,
}
impl Web {
  //Odd length queries greater than 3
  pub fn get_chain_naive(&self, query: &[Option<String>]) -> Vec<Vec<String>> {
    let mut ret_v: Vec<Vec<String>> = Vec::new();
    ret_v.push(Vec::new());
    let mut i: usize = 2;
    while i < query.len() {
      let Triple(s, p, o) =
        self.get(
          &(query[i-2].clone(),
            query[i-1].clone(),
            query[i].clone())
        )[0].clone();
      if i == 2 { ret_v[0].push(s); }
      ret_v[0].push(p);
      ret_v[0].push(o);
      println!("{:?}", ret_v);
      i += 2
    }
    if ret_v[0].len() == query.len() {
      return ret_v
    }
    return Vec::new()
  }
  //All length queries greater than 3
  pub fn get_chain_2(&self, query: &[Option<String>]) -> Vec<Vec<String>> {
    //Initialise all data
    let mut ret_v: Vec<Vec<String>> = Vec::new();
    ret_v.push(Vec::new());
    let mut i: usize = 0;
    let end: usize = query.len()-2;
    //If the query length is even, then the final item must be a Predicate
    if query.len() % 2 == 0 {
      
    }
    else {
      while i < end {
        let Triple(s, p, o) =
          self.get(
            &(query[i].clone(),
              query[i+1].clone(),
              query[i+2].clone())
          )[0].clone();
        if i == 0 { ret_v[0].push(s); }
        ret_v[0].push(p);
        ret_v[0].push(o);
        i += 2
      }
    }

    println!("{:?}", ret_v);
    //Return if got it all
    if ret_v[0].len() == query.len() {
      return ret_v
    }
    return Vec::new()
  }
}
impl Web {
  pub fn new() -> Self {
    Web {
      spo: TripleStore::new(),
      pos: TripleStore::new(),
      osp: TripleStore::new()
    }
  }
  pub fn add(&mut self, t: &Triple) {
    let Triple(s, p, o) = t;
    self.spo.add(&Triple(s.to_string(), p.to_string(), o.to_string()));
    self.pos.add(&Triple(p.to_string(), o.to_string(), s.to_string()));
    self.osp.add(&Triple(o.to_string(), s.to_string(), p.to_string()));
  }
  pub fn erase(&mut self, t: &Triple) {
    let Triple(s, p, o) = t;
    self.spo.erase(&Triple(s.to_string(), p.to_string(), o.to_string()));
    self.pos.erase(&Triple(p.to_string(), o.to_string(), s.to_string()));
    self.osp.erase(&Triple(o.to_string(), s.to_string(), p.to_string()));
  }
  pub fn get(&self, qt: &QueryTriple) -> Vec<Triple> {
    match qt {
      (Some(s), Some(p), Some(o)) => {
        self.spo.get(&(
          Some(s.to_string()),
          Some(p.to_string()),
          Some(o.to_string())
          ))
      },
      (Some(s), Some(p), None) => {
        self.spo.get(&(
          Some(s.to_string()),
          Some(p.to_string()),
          None
          ))
      },
      (Some(s), None, Some(o)) => {
        self.osp.get_ordered(&(
          Some(o.to_string()),
          Some(s.to_string()),
          None
          ),
          TOrdering::OSP)
      },
      (None, Some(p), Some(o)) => {
        self.pos.get_ordered(&(
          Some(p.to_string()),
          Some(o.to_string()),
          None
          ),
          TOrdering::POS)
      },
      (Some(s), None, None) => {
        self.spo.get(&(
          Some(s.to_string()),
          None,
          None
          ))
      },
      (None, Some(p), None) => {
        self.pos.get_ordered(&(
          Some(p.to_string()),
          None,
          None
          ),
          TOrdering::POS)
      },
      (None, None, Some(o)) => {
        self.osp.get_ordered(&(
          Some(o.to_string()),
          None,
          None
          ),
          TOrdering::OSP)
      },
      (None, None, None) => {
        self.spo.get(&(
          None,
          None,
          None
          ))
      },
    }
  }
  pub fn replace(&mut self, old_t: Triple, new_t: Triple) {
    self.erase(&old_t);
    self.add(&new_t);
  }
}














