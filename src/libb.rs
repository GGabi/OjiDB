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
pub struct TripleStore(Vec<(String, Box<Vec<(String, Box<Vec<String>>)>>)>);
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
  store: &'a TripleStore,
  curr_head: usize,
  curr_mid:  usize,
  curr_tail: usize,
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

/*
A data-structure that sacrifices space for fast data access
via storing 3 versions of the same "Triple data" in
unique orderings inspired by Hexastore.
*/
#[derive(Clone, Debug)]
pub struct Graph {
  pub spo: TripleStore,
  pub pos: TripleStore,
  pub osp: TripleStore,
}
impl Graph {
  pub fn new() -> Self {
    Graph {
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
  pub fn iter(&self) -> TripleStoreRefIterator {
    TripleStoreRefIterator {
      store: &self.spo,
      curr_head: 0,
      curr_mid: 0,
      curr_tail: 0,
    }
  }
}
//WIP
impl Graph {
  pub fn get(&self, q: QueryChain) -> Vec<Vec<String>> {

    /*  Traced algorithm:
     *  If query length 0: Return empty vec
     *  If query length 1: Return vec with get_subject() results
     *  If query length 2: Return vec with get_double() results
     *  If query length 3+:
     *  - If query length even:
     *  - - Store double for query at end
     *  - Break up query into triples to be queried one-after-another
     *  - Call get_triple() on the first triple to populate the return vec
     *  - For each query triple:
     *  - - For each chain in return values:
     *  - - - Call get_triple() with triple: (final value of chain, query triple mid, query triple tail)
     *  - - - If no results: Pass
     *  - - - If 1 result:   Extend return chain by the 2 tail values in return triple
     *  - - - If 2+ results:
     *  - - - - Store copy of return chain
     *  - - - - Extend old return chain by 2 tail values of first result
     *  - - - - For each subsequent result:
     *  - - - - - Append copy of return chain
     *  - - - - - Extend said copy by the 2 tail values of result
     *  - - Remove all return chains that had no query matches in the last pass
     *  - If query chain length is even:
     *  - - For each query chain:
     *  - - - Call get_double() with double: (final value of chain, query double tail)
     *  - - - If no results: Pass
     *  - - - If 1 result:   Extend return chain by the tail value of return double
     *  - - - If 2+ results: 
     *  - - - - Store copy of return chain
     *  - - - - Extend old return chain by tail value of first result
     *  - - - - For each subsequent result:
     *  - - - - - Append copy of return chain
     *  - - - - - Extend said copy by tail value of result
     *  Remove all return chains that do not match the length of the query chain
     *  Return Vec of return chains
     */

    //Initialise return list
    let mut ret_v: Vec<Vec<String>> = Vec::new();

    //Do one-time calc on the query length
    let q_len = q.len();

    //Handle the special cases of length
    //  0 and 1.
    if q_len == 0 {
      return Vec::new()
    }
    else if q_len == 1 {
      for s in self.get_subject(&q[0]) {
        ret_v.push(vec!(s.clone()));
      }
      return ret_v
    }

    //If the query chain is of even length,
    //  then there will be a trailing double
    //  that needs to be resolved at the end.
    //Store this double for later use, mark
    //  if we have one or not.
    let mut q_double = (None, None);
    let mut has_tail_double = false;
    if q_len % 2 == 0 {
      has_tail_double = true;
      q_double = (q[q_len-2].clone(), q[q_len-1].clone());
    }

    //Handle the case of a query length of exactly 2,
    //  else proceed to handle all queries of length
    //  3 or more.
    if q_len == 2 {
      for double in self.get_double(&q_double) {
        let mut v: Vec<String> = Vec::new();
        v.push(double.0.clone());
        v.push(double.1.clone());
        ret_v.push(v);
      }
    }
    else {
      //Gather query triples from chain
      let mut q_triples: Vec<QueryTriple> = Vec::new();
      for i in (0..q.len()-2).step_by(2) {
        q_triples.push(
          (q[i].clone(), q[i+1].clone(), q[i+2].clone())
        );
      }

      //Start processing
      let mut q_cursor: usize = 0; //Keeps track of which query triple we're looking at
      let mut r_cursor: usize = 0; //Keeps track of which vec in ret_v

      //Populate the return vec with the results of the first query triple
      let ts = self.get_triple(&q_triples[q_cursor]);
      for t in ts {
        let mut v: Vec<String> = Vec::new();
        v.push(t.0);
        v.push(t.1);
        v.push(t.2);
        ret_v.push(v);
      }

      //Set the start point as the second query triple in the chain
      //  and evaluate the return values until all queries used up
      q_cursor = 1;
      while q_cursor < q_triples.len() {
        //Set the r_cursor back to the beginning of ret_vals
        r_cursor = 0;
        let mut ret_v_len: usize = ret_v.len();
        while r_cursor < ret_v_len { //For each item in ret_v
          //Query using the final value from the existing list in ret_vals
          //  as the Subject, store all the return triples in ts.
          let ts = self.get_triple(&(
            Some(ret_v[r_cursor][ret_v[r_cursor].len()-1].clone()),
            q_triples[q_cursor].1.clone(),
            q_triples[q_cursor].2.clone()
          ));
          let ts_len: usize = ts.len();
          let old_t = ret_v[r_cursor].clone(); //Store the unaffected triple for potential cloning
          //Extend the return value if the query brings back a positive result.
          if ts_len > 0 {
            ret_v[r_cursor].push(ts[0].1.clone());
            ret_v[r_cursor].push(ts[0].2.clone());
          }
          //If more than one result from the query,
          //  push a copy of old_t to ret_v then extend with results,
          //  Repeat for every result after the first.
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
        //All return values of incorrect length were indicate
        //  that they did not find  a match in the last iteraton,
        //  so remove them.
        //  Valid length examples: 5, 7, 9...
        //  (start at 5, increase by 2 for each query triple)
        ret_v = ret_v.into_iter().filter(|x| x.len() == ((q_cursor+1)*2)+1).collect();
        q_cursor += 1;
      }
      if has_tail_double {
        r_cursor = 0;
        let mut ret_v_len: usize = ret_v.len();
        while r_cursor < ret_v_len {
            let ds = self.get_double(&(
              Some(ret_v[r_cursor][ret_v[r_cursor].len()-1].clone()),
              q_double.1.clone()
            ));
            let ds_len: usize = ds.len();
            let old_t = ret_v[r_cursor].clone(); //Store the unaffected chain for potential cloning
            //Extend the return value if the query brings back a positive result.
            if ds_len > 0 {
              ret_v[r_cursor].push(ds[0].1.clone());
              //If more than one result from the query,
              //  push a copy of old_t to ret_v then extend with results,
              //  Repeat for every result after the first.
              if ds_len > 1 {
                for d_cursor in 1..ds_len {
                  ret_v.push(old_t.clone());
                  ret_v_len += 1;
                  ret_v[ret_v_len-1].push(ds[d_cursor].1.clone());
                }
              }
            }
          r_cursor += 1;
        }
      }
    }
    ret_v = ret_v.into_iter().filter(|x| x.len() == q_len).collect();
    ret_v
  }
  fn get_subject(&self, s: &Option<String>) -> Vec<String> {
    let mut ret_v: Vec<String> = Vec::new();
    if let Some(subject) = s {
      if let Some(_) = self.spo.0.iter().find(|(val, _)| val == subject) {
        ret_v.push(subject.clone());
      }
    }
    else {
      for (subject, _) in self.spo.0.iter() {
        ret_v.push(subject.clone());
      }
    }
    ret_v
  }
  fn get_double(&self, qd: &QueryDouble) -> Vec<Double> {
    let triples = match qd {
      (Some(s), Some(p)) => {
        self.spo.get_triple(&(
          Some(s.to_string()),
          Some(p.to_string()),
          None
          ))
      },
      (Some(s), None) => {
        self.spo.get_triple(&(
          Some(s.to_string()),
          None,
          None
          ))
      },
      _ => {
        Vec::new()
      },
    };
    let mut ret_v: Vec<Double> = triples.into_iter().map(|(h, m, t)| (h, m)).collect();
    ret_v.dedup();
    ret_v
  }
}

impl Graph {
  pub fn get_trial(&self, q: Query) -> Result {
    let mut r = Result::new();
    match q {
      Query::Double(s, p) => {
        let mut q1: Option<String>;
        let mut q2: Option<String>;
        match &s {
          QueryUnit::Val(a) => { q1 = Some(a.clone()); },
          QueryUnit::Var(_)
          | QueryUnit::Anon
          | QueryUnit::Ignore => { q1 = None; },
        }
        match &p {
          QueryUnit::Val(b) => { q2 = Some(b.clone()); },
          QueryUnit::Var(_)
          | QueryUnit::Anon
          | QueryUnit::Ignore => { q2 = None; },
        }
        let query_res = self.get_double(&(q1, q2));
        if query_res.len() > 0 {
          match s {
            QueryUnit::Val(a) => { r.add_anon(ResultUnit::Value(a)); },
            QueryUnit::Var(a) => { r.add_var(a, query_res[0].0.clone()); },
            QueryUnit::Anon   => { r.add_anon(ResultUnit::Value(query_res[0].0.clone())); },
            QueryUnit::Ignore => { r.add_anon(ResultUnit::Ignore); },
          }
          match p {
            QueryUnit::Val(b) => { r.add_anon(ResultUnit::Value(b)); },
            QueryUnit::Var(b) => { r.add_var(b, query_res[0].1.clone()); },
            QueryUnit::Anon   => { r.add_anon(ResultUnit::Value(query_res[0].1.clone())); },
            QueryUnit::Ignore => { r.add_anon(ResultUnit::Ignore); },
          }
        }
      },
      Query::Triple(s, p, o) => {
        let mut q1: Option<String>;
        let mut q2: Option<String>;
        let mut q3: Option<String>;
        match &s {
          QueryUnit::Val(a) => { q1 = Some(a.clone()); },
          QueryUnit::Var(_)
          | QueryUnit::Anon
          | QueryUnit::Ignore => { q1 = None; },
        }
        match &p {
          QueryUnit::Val(b) => { q2 = Some(b.clone()); },
          QueryUnit::Var(_)
          | QueryUnit::Anon
          | QueryUnit::Ignore => { q2 = None; },
        }
        match &o {
          QueryUnit::Val(b) => { q3 = Some(b.clone()); },
          QueryUnit::Var(_)
          | QueryUnit::Anon
          | QueryUnit::Ignore => { q3 = None; },
        }
        let query_res = self.get_triple(&(q1, q2, q3));
        if query_res.len() > 0 {
          for i in 0..query_res.len() {
            match &s {
              QueryUnit::Val(a) => { r.add_anon(ResultUnit::Value(a.to_string())); },
              QueryUnit::Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
              QueryUnit::Anon   => { r.add_anon(ResultUnit::Value(query_res[i].0.clone())); },
              QueryUnit::Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
            match &p {
              QueryUnit::Val(b) => { r.add_anon(ResultUnit::Value(b.to_string())); },
              QueryUnit::Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
              QueryUnit::Anon   => { r.add_anon(ResultUnit::Value(query_res[i].1.clone())); },
              QueryUnit::Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
            match &o {
              QueryUnit::Val(c) => { r.add_anon(ResultUnit::Value(c.to_string())); },
              QueryUnit::Var(c) => { r.add_var(c.to_string(), query_res[i].2.clone()); },
              QueryUnit::Anon   => { r.add_anon(ResultUnit::Value(query_res[i].2.clone())); },
              QueryUnit::Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
          }
        }
      }
      _ => {},
    };
    r
  }
}

//Trial
use std::collections::HashMap;

//Queries
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
#[derive(Clone, Debug)]
pub enum Query {
  Null,
  Single(QueryUnit),
  Double(QueryUnit, QueryUnit),
  Triple(QueryUnit, QueryUnit, QueryUnit),
  Chain(Vec<QueryUnit>),
}
impl Query {
  fn from(vals: &[QueryUnit]) -> Option<Self> {
    match vals.len() {
      0 => None,
      1 => Some(Query::Single(vals[0].clone())),
      2 => Some(Query::Double(vals[0].clone(), vals[1].clone())),
      3 => Some(Query::Triple(vals[0].clone(), vals[1].clone(), vals[2].clone())),
      _ => Some(Query::Chain(vals.to_vec())),
    }
  }
  pub fn from_str(vals: &[&str]) -> Option<Self> {
    match vals.len() {
      0 => None,
      1 => Some(Query::Single(QueryUnit::from(vals[0]))),
      2 => Some(Query::Double(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1])
                              )),
      3 => Some(Query::Triple(QueryUnit::from(vals[0]),
                              QueryUnit::from(vals[1]),
                              QueryUnit::from(vals[2])
                              )),
      _ => {
        let mut chain: Vec<QueryUnit> = Vec::new();
        for v in vals {
          chain.push(QueryUnit::from(v));
        }
        Some(Query::Chain(chain))
      },
    }
  }
}

//Results
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
  fn new() -> Self {
    Result {
      values: Vec::new(),
      var_map: HashMap::new(),
    }
  }
  fn add_anon(&mut self, val: ResultUnit) {
    self.values.push(val);
  }
  fn add_var(&mut self, var: String, val: String) {
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
}

pub struct ResultCollection {
  results: Vec<Result>,
  query: Query,
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

/*
Function: Extend chain, which takes in a vector of strings as the subject and 2 Option<String>s as the pred and obj.
          Return a Vec of result chains of strings

Query Types:
- Single(Value)
- Double(Value, Value)
- Triple(Value, Value, Value)
- Chain(Vec<Value>)

Query Values:
- Some(String)
- ???
- ???(String)
- None

Result Values:
- Vec<String>
- HashMap(Variable name, Value position)
- Methods to get values of variables etc.

Result Collection:
- Vec<Results>
- Original Query
- Methods to get vec of all values of a return variable etc. 













*/





