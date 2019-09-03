
use super::{
  TripleStore::{TripleStore, TripleStoreIterator},
  super::{
    Ordering
    // Results::{Result, ResultUnit, ResultCollection}
  }
};

type Triple = (String, String, String);
type QueryTriple = (Option<String>, Option<String>, Option<String>);
// type Double = (String, String);
// type QueryDouble = (Option<String>, Option<String>);

/*
A data-structure that sacrifices space for fast data access
via storing 3 versions of the same "Triple data" in
unique orderings inspired by Hexastore.
*/
#[derive(Clone, Debug, PartialEq)]
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
    /* Add should eventually consume the input */
    self.spo.add((s.to_string(), p.to_string(), o.to_string()));
    self.pos.add((p.to_string(), o.to_string(), s.to_string()));
    self.osp.add((o, s, p));
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
  pub fn iter(&self) -> TripleStoreIterator {
    self.spo.iter()
  }
}
impl Graph {
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
  // fn get_double(&self, qd: &QueryDouble, ord: [Ordering; 2]) -> Vec<Double> {
  //   use Ordering::{S, P, O};
  //   let store = match &ord {
  //     [S,P] => { &self.spo },
  //     [P,O] => { &self.pos },
  //     [O,S] => { &self.osp },
  //     _  => { return Vec::new() },
  //   };
  //   store.get_double(qd)
  // }
  // fn get_single(&self, q: &Option<String>, ord: Ordering) -> Vec<String> {
  //   use Ordering::{S, P, O};
  //   let store = match &ord {
  //     S => { &self.spo },
  //     P => { &self.pos },
  //     O => { &self.osp },
  //     _  => { return Vec::new() },
  //   };
  //   store.get_single(q)
  // }
  // pub fn get(&self, q: Query) -> ResultCollection {
  //   use QueryUnit::{Val, Var, Anon, Ignore};
  //   use Ordering::{S, P, O};
  //   let mut rc = ResultCollection::new();
  //   rc.query = q.clone();
  //   match q {
  //     Query::Null => {/*Do nothing*/},
  //     Query::Single(h, ord) => {
  //       //Filter out all the Ignores, call again with corrected query
  //       match &h {
  //         Ignore => {
  //           return rc
  //         },
  //         _ => {},
  //       };
  //       //Actually start processing now
  //       let mut q: Option<String>;
  //       match &h {
  //         Val(a) => { q = Some(a.clone()); },
  //         Var(_)
  //         | Anon
  //         | Ignore => { q = None; },
  //       };
  //       //Rearrange the Ordering to match the stores Graph has
  //       let query_res = match &ord {
  //         P => { self.get_single(&q, P) },
  //         S => { self.get_single(&q, S) },
  //         O => { self.get_single(&q, O) },
  //         _ => { self.get_single(&q, S) },
  //       };
  //       if query_res.len() > 0 {
  //         for i in 0..query_res.len() {
  //           let mut r = Result::new();
  //           match &h {
  //             Val(a) => { r.add_anon(ResultUnit::Value(a.to_string())); },
  //             Var(a) => { r.add_var(a.to_string(), query_res[i].clone()); },
  //             Anon   => { r.add_anon(ResultUnit::Value(query_res[i].clone())); },
  //             Ignore => { r.add_anon(ResultUnit::Ignore); },
  //           }
  //           rc.results.push(r);
  //         }
  //       }
  //     },
  //     Query::Double(h, t, ord) => {
  //       //Filter out all the Ignores, call again with corrected query
  //       match (&h, &t) {
  //         (Ignore, Ignore) => {
  //           return rc
  //         },
  //         (Ignore, _) => {
  //           return self.get(Query::Single(t, ord[1].clone()))
  //         },
  //         (_, Ignore) => {
  //           return self.get(Query::Single(h, ord[0].clone()))
  //         },
  //         _ => {},
  //       };
  //       //Actually start processing now
  //       let mut q1: Option<String>;
  //       let mut q2: Option<String>;
  //       match &h {
  //         Val(a) => { q1 = Some(a.clone()); },
  //         Var(_)
  //         | Anon
  //         | Ignore => { q1 = None; },
  //       };
  //       match &t {
  //         Val(b) => { q2 = Some(b.clone()); },
  //         Var(_)
  //         | Anon
  //         | Ignore => { q2 = None; },
  //       };
  //       //Rearrange the Ordering to match the stores Graph has
  //       let query_res = match &ord {
  //         [P,S] => { self.get_double(&(q1, q2), [S,P]) },
  //         [S,O] => { self.get_double(&(q1, q2), [O,S]) },
  //         [O,P] => { self.get_double(&(q1, q2), [P,O]) },
  //         _ => { self.get_double(&(q1, q2), ord) }
  //       };
  //       if query_res.len() > 0 {
  //         for i in 0..query_res.len() {
  //           let mut r = Result::new();
  //           match &h {
  //             Val(a) => { r.add_anon(ResultUnit::Value(a.to_string())); },
  //             Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
  //             Anon   => { r.add_anon(ResultUnit::Value(query_res[i].0.clone())); },
  //             Ignore => { r.add_anon(ResultUnit::Ignore); },
  //           }
  //           match &t {
  //             Val(b) => { r.add_anon(ResultUnit::Value(b.to_string())); },
  //             Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
  //             Anon   => { r.add_anon(ResultUnit::Value(query_res[i].1.clone())); },
  //             Ignore => { r.add_anon(ResultUnit::Ignore); },
  //           }
  //           rc.results.push(r);
  //         }
  //       }
  //     },
  //     Query::Triple(s, p, o, _) => {
  //       //Filter out all the Ignores, call again with corrected query
  //       match (&s, &p, &o) {
  //         (Ignore, Ignore, Ignore) => {
  //           return rc
  //         },
  //         (Ignore, Ignore, _) => {
  //           return self.get(Query::Single(o, O))
  //         },
  //         (Ignore, _, Ignore) => {
  //           return self.get(Query::Single(p, P))
  //         },
  //         (_, Ignore, Ignore) => {
  //           return self.get(Query::Single(s, S))
  //         },
  //         (Ignore, _, _) => {
  //           return self.get(Query::Double(p, o, [P, O]))
  //         },
  //         (_, Ignore, _) => {
  //           return self.get(Query::Double(s, o, [S, O]))
  //         },
  //         (_, _, Ignore) => {
  //           return self.get(Query::Double(s, p, [S, P]))
  //         },
  //         _ => {},
  //       };
  //       //Actually start processing now
  //       let mut q1: Option<String>;
  //       let mut q2: Option<String>;
  //       let mut q3: Option<String>;
  //       match &s {
  //         Val(a) => { q1 = Some(a.clone()); },
  //         Var(_)
  //         | Anon
  //         | Ignore => { q1 = None; },
  //       };
  //       match &p {
  //         Val(b) => { q2 = Some(b.clone()); },
  //         Var(_)
  //         | Anon
  //         | Ignore => { q2 = None; },
  //       };
  //       match &o {
  //         Val(b) => { q3 = Some(b.clone()); },
  //         Var(_)
  //         | Anon
  //         | Ignore => { q3 = None; },
  //       };
  //       let query_res = self.get_triple(&(q1, q2, q3));
  //       if query_res.len() > 0 {
  //         for i in 0..query_res.len() {
  //           let mut r = Result::new();
  //           match &s {
  //             Val(a) => { r.add_anon(ResultUnit::Value(a.to_string())); },
  //             Var(a) => { r.add_var(a.to_string(), query_res[i].0.clone()); },
  //             Anon   => { r.add_anon(ResultUnit::Value(query_res[i].0.clone())); },
  //             Ignore => { r.add_anon(ResultUnit::Ignore); },
  //           }
  //           match &p {
  //             Val(b) => { r.add_anon(ResultUnit::Value(b.to_string())); },
  //             Var(b) => { r.add_var(b.to_string(), query_res[i].1.clone()); },
  //             Anon   => { r.add_anon(ResultUnit::Value(query_res[i].1.clone())); },
  //             Ignore => { r.add_anon(ResultUnit::Ignore); },
  //           }
  //           match &o {
  //             Val(c) => { r.add_anon(ResultUnit::Value(c.to_string())); },
  //             Var(c) => { r.add_var(c.to_string(), query_res[i].2.clone()); },
  //             Anon   => { r.add_anon(ResultUnit::Value(query_res[i].2.clone())); },
  //             Ignore => { r.add_anon(ResultUnit::Ignore); },
  //           }
  //           rc.results.push(r);
  //         }
  //       }
  //     },
  //   };
  //   rc
  // }
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

















