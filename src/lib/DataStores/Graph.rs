
use super::{
  TripleStore::{TripleStore, TripleStoreRefIterator},
  super::{
    Ordering, Triple, Double, QueryDouble, QueryTriple, QueryChain,
    Queries::{Query, QueryUnit},
    Results::{Result, ResultUnit, ResultCollection}
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
        let mut ret_v: Vec<Triple> = Vec::new();
        let triples = self.osp.get_triple(&(
          Some(o.to_string()),
          Some(s.to_string()),
          None));
        for triple in triples {
          ret_v.push(t_order(triple, &Ordering::OSP));
        }
        ret_v
      },
      (None, Some(p), Some(o)) => {
        let mut ret_v: Vec<Triple> = Vec::new();
        let triples = self.pos.get_triple(&(
          Some(p.to_string()),
          Some(o.to_string()),
          None));
        for triple in triples {
          ret_v.push(t_order(triple, &Ordering::POS));
        }
        ret_v
      },
      (Some(s), None, None) => {
        self.spo.get_triple(&(
          Some(s.to_string()),
          None,
          None
          ))
      },
      (None, Some(p), None) => {
        let mut ret_v: Vec<Triple> = Vec::new();
        let triples = self.pos.get_triple(&(
          Some(p.to_string()),
          None,
          None));
        for triple in triples {
          ret_v.push(t_order(triple, &Ordering::POS));
        }
        ret_v
      },
      (None, None, Some(o)) => {
        let mut ret_v: Vec<Triple> = Vec::new();
        let triples = self.osp.get_triple(&(
          Some(o.to_string()),
          None,
          None));
        for triple in triples {
          ret_v.push(t_order(triple, &Ordering::OSP));
        }
        ret_v
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
  pub fn erase(&mut self, (s, p, o): &Triple) {
    self.spo.erase(&(s.to_string(), p.to_string(), o.to_string()));
    self.pos.erase(&(p.to_string(), o.to_string(), s.to_string()));
    self.osp.erase(&(o.to_string(), s.to_string(), p.to_string()));
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
  pub fn get(&self, q: QueryChain) -> Vec<Vec<String>> {

    /*  Traced algorithm:
     *  If query length 0: Return empty vec
     *  If query length 1: Return vec with get_single() results
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
      for s in self.spo.get_single(&q[0]) {
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
      for double in self.spo.get_double(&q_double) {
        let mut v: Vec<String> = Vec::new();
        v.push(double.0.clone());
        v.push(double.1.clone());
        ret_v.push(v);
      }
      return ret_v
    }

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
          let ds = self.spo.get_double(&(
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
    ret_v = ret_v.into_iter().filter(|x| x.len() == q_len).collect();
    ret_v
  }
}

impl Graph {
  fn get_double(&self, qd: &QueryDouble, ord: [Ordering; 2]) -> Vec<Double> {
    use Ordering::{S, P, O};
    let store = match &ord {
      [S,P] => { &self.spo },
      [P,O] => { &self.pos },
      [O,S] => { &self.osp },
      _  => { return Vec::new() },
    };
    store.get_double(qd)
  }
  pub fn get_trial(&self, q: Query) -> ResultCollection {
    use QueryUnit::{Val, Var, Anon, Ignore};
    use Ordering::{S, P, O};
    let mut rc = ResultCollection::new();
    rc.query = q.clone();
    match q {
      Query::Null => {/*Do nothing*/},
      Query::Single(s, _) => {},
      Query::Double(h, t, ord) => {
        //Filter out all the Ignores, call again with corrected query
        match (&h, &t) {
          (Ignore, _) => {
            return self.get_trial(Query::Single(t, ord[1].clone()))
          },
          (_, Ignore) => {
            return self.get_trial(Query::Single(h, ord[0].clone()))
          },
          (Ignore, Ignore) => {
            return rc
          },
          _ => {},
        };
        //Actually start processing now
        let mut q1: Option<String>;
        let mut q2: Option<String>;
        match &h {
          Val(a) => { q1 = Some(a.clone()); },
          Var(_)
          | Anon
          | Ignore => { q1 = None; },
        };
        match &t {
          Val(b) => { q2 = Some(b.clone()); },
          Var(_)
          | Anon
          | Ignore => { q2 = None; },
        };
        //Rearrange the Ordering to match the stores Graph has
        let query_res = match &ord {
          [P,S] => { self.get_double(&(q1, q2), [S,P]) },
          [S,O] => { self.get_double(&(q1, q2), [O,S]) },
          [O,P] => { self.get_double(&(q1, q2), [P,O]) },
          _ => { self.get_double(&(q1, q2), ord) }
        };
        if query_res.len() > 0 {
          for i in 0..query_res.len() {
            let mut r = Result::new();
            match &h {
              Val(a) => { r.add_anon(ResultUnit::Value(a.to_string())); },
              Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
              Anon   => { r.add_anon(ResultUnit::Value(query_res[i].0.clone())); },
              Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
            match &t {
              Val(b) => { r.add_anon(ResultUnit::Value(b.to_string())); },
              Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
              Anon   => { r.add_anon(ResultUnit::Value(query_res[i].1.clone())); },
              Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
            rc.results.push(r);
          }
        }
      },
      Query::Triple(s, p, o, _) => {
        //Filter out all the Ignores, call again with corrected query
        match (&s, &p, &o) {
          (Ignore, _, _) => {
            return self.get_trial(Query::Double(p, o, [P, O]))
          },
          (_, Ignore, _) => {
            return self.get_trial(Query::Double(s, o, [S, O]))
          },
          (_, _, Ignore) => {
            return self.get_trial(Query::Double(s, p, [S, P]))
          },
          (Ignore, Ignore, _) => {
            return self.get_trial(Query::Single(o, O))
          },
          (Ignore, _, Ignore) => {
            return self.get_trial(Query::Single(p, P))
          },
          (_, Ignore, Ignore) => {
            return self.get_trial(Query::Single(s, S))
          },
          (Ignore, Ignore, Ignore) => {
            return rc
          },
          _ => {},
        };
        //Actually start processing now
        let mut q1: Option<String>;
        let mut q2: Option<String>;
        let mut q3: Option<String>;
        match &s {
          Val(a) => { q1 = Some(a.clone()); },
          Var(_)
          | Anon
          | Ignore => { q1 = None; },
        };
        match &p {
          Val(b) => { q2 = Some(b.clone()); },
          Var(_)
          | Anon
          | Ignore => { q2 = None; },
        };
        match &o {
          Val(b) => { q3 = Some(b.clone()); },
          Var(_)
          | Anon
          | Ignore => { q3 = None; },
        };
        let query_res = self.get_triple(&(q1, q2, q3));
        if query_res.len() > 0 {
          for i in 0..query_res.len() {
            let mut r = Result::new();
            match &s {
              Val(a) => { r.add_anon(ResultUnit::Value(a.to_string())); },
              Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
              Anon   => { r.add_anon(ResultUnit::Value(query_res[i].0.clone())); },
              Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
            match &p {
              Val(b) => { r.add_anon(ResultUnit::Value(b.to_string())); },
              Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
              Anon   => { r.add_anon(ResultUnit::Value(query_res[i].1.clone())); },
              Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
            match &o {
              Val(c) => { r.add_anon(ResultUnit::Value(c.to_string())); },
              Var(c) => { r.add_var(c.to_string(), query_res[i].2.clone()); },
              Anon   => { r.add_anon(ResultUnit::Value(query_res[i].2.clone())); },
              Ignore => { r.add_anon(ResultUnit::Ignore); },
            }
            rc.results.push(r);
          }
        }
      }
      // Query::Chain(chain) => {},
      _ => {},
    };
    rc
  }
}

fn t_order(t: Triple, curr_ordering: &Ordering) -> Triple {
  use Ordering::{POS, OSP};
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


















