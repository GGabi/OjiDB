
use super::{
  TripleStore::{TripleStore, TripleStoreRefIterator},
  super::{
    TOrdering, Triple, Double, QueryDouble, QueryTriple, QueryChain,
    Queries::{Query, QueryUnit},
    Results::{Result, ResultUnit}
  }
};

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