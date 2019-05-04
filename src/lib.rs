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
fn t_order(t: Triple, curr_ordering: &TOrdering) -> Triple {
  match &curr_ordering {
    POS => {
      (t.2.to_string(),
       t.0.to_string(),
       t.1.to_string())  
    },
    OSP => {
      (t.1.to_string(),
       t.2.to_string(),
       t.0.to_string())
    },
    _ => {
      t.clone()
    },
  }
}

/*
Data types to reduce verbosity.
All instances of None in Querys indicate required values.
*/
pub type Triple = (String, String, String);
pub type QueryTriple = (Option<String>, Option<String>, Option<String>);
pub type QueryChain<'a>  = &'a[Option<String>];
type Double = (String, String);
type QueryDouble = (Option<String>, Option<String>);

/*
A data-structure that stores triples in, I hesitate to say,
the most space-efficient way possible a-la Hexastore.
*/
#[derive(Clone, Debug)]
struct TripleStore(Vec<(String, Box<Vec<(String, Box<Vec<String>>)>>)>);
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
  fn get_ordered(&self, qt: &QueryTriple, ord: &TOrdering) -> Vec<Triple> {
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
  pub fn new() -> Self {
    Web {
      spo: TripleStore::new(),
      pos: TripleStore::new(),
      osp: TripleStore::new()
    }
  }
  pub fn add(&mut self, (s, p, o): Triple) {
    //Add should eventually consume the input
    self.spo.add((s.to_string(), p.to_string(), o.to_string()));
    self.pos.add((p.to_string(), o.to_string(), s.to_string()));
    self.osp.add((o, s, p));
  }
  pub fn erase(&mut self, (s, p, o): &Triple) {
    self.spo.erase(&(s.to_string(), p.to_string(), o.to_string()));
    self.pos.erase(&(p.to_string(), o.to_string(), s.to_string()));
    self.osp.erase(&(o.to_string(), s.to_string(), p.to_string()));
  }
  pub fn get_triple(&self, qt: &QueryTriple) -> Vec<Triple> {
    match qt {
      (Some(s), Some(p), Some(o)) => {
        self.spo.get_triple(&(
          Some(s.to_string()),
          Some(p.to_string()),
          Some(o.to_string())
          ))
      },
      (Some(s), Some(p), None) => {
        self.spo.get_triple(&(
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
          &TOrdering::OSP)
      },
      (None, Some(p), Some(o)) => {
        self.pos.get_ordered(&(
          Some(p.to_string()),
          Some(o.to_string()),
          None
          ),
          &TOrdering::POS)
      },
      (Some(s), None, None) => {
        self.spo.get_triple(&(
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
          &TOrdering::POS)
      },
      (None, None, Some(o)) => {
        self.osp.get_ordered(&(
          Some(o.to_string()),
          None,
          None
          ),
          &TOrdering::OSP)
      },
      (None, None, None) => {
        self.spo.get_triple(&(
          None,
          None,
          None
          ))
      },
    }
  }
  pub fn replace(&mut self, old_t: &Triple, new_t: Triple) {
    self.erase(&old_t);
    self.add(new_t);
  }
}

//WIP
impl Web {
  //Odd length chains with Nones in them
  pub fn get(&self, q: &[Option<String>]) -> Vec<Vec<String>> {

    let mut q_vec = q.to_vec();
    if q_vec.len() % 2 == 0 { q_vec.push(None); }
    let query: &[Option<String>] = &q_vec;

    //Gather query triples from chain
    let mut q_triples: Vec<QueryTriple> = Vec::new();
    for i in (0..query.len()-2).step_by(2) {
      q_triples.push(
        (query[i].clone(), query[i+1].clone(), query[i+2].clone())
      );
    }

    //Initialise return list
    let mut ret_v: Vec<Vec<String>> = Vec::new();

    //Start processing
    let mut q_cursor: usize = 0; //Keeps track of which query triple we're looking at
    let mut r_cursor: usize = 0; //Keeps track of which vec in ret_v
    //Populates the return vec with the results of the first query triple
    let ts = self.get_triple(&q_triples[q_cursor]);
    for t in ts {
      let mut v: Vec<String> = Vec::new();
      v.push(t.0);
      v.push(t.1);
      v.push(t.2);
      ret_v.push(v);
    }
    //Sets the start point as the second query triple in the chain
    //  and evaluates the return values until all queries used up
    q_cursor = 1;
    while q_cursor < q_triples.len() {
      //Sets the r_cursor back to the beginning of ret_vals
      r_cursor = 0;
      let mut ret_v_len: usize = ret_v.len();
      while r_cursor < ret_v_len {
        //Query using the final value from the existing list in ret_vals
        //  as the Subject, store all the return triples in ts.
        let ts = self.get_triple(&(
          Some(ret_v[r_cursor][ret_v[r_cursor].len()-1].clone()),
          q_triples[q_cursor].1.clone(),
          q_triples[q_cursor].2.clone()
        ));
        let ts_len: usize = ts.len();
        let old_t = ret_v[r_cursor].clone(); //Store the unaffected triple for potential cloning
        //Extend the return values if needed as a result of the query.
        //  Clone the base, then extend, for multiple query results.
        if ts_len > 0 {
          ret_v[r_cursor].push(ts[0].1.clone());
          ret_v[r_cursor].push(ts[0].2.clone());
        }
        if ts_len > 1 {
          for t_cursor in 1..ts_len {
            ret_v.push(old_t.clone());
            ret_v_len += 1;
            ret_v[ret_v_len-1].push(ts[t_cursor].1.clone());
            ret_v[ret_v_len-1].push(ts[t_cursor].2.clone());
          }
        }
        r_cursor += 1;
      }
      //All ones of incorrect length were not a match in the last iteraton,
      //  so remove them.
      println!("{:?}", ret_v);
      ret_v = ret_v.into_iter().filter(|x| x.len() == ((q_cursor+1)*2)+1).collect();
      q_cursor += 1;
    }
    ret_v
  }
}

































