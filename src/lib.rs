#![allow(non_snake_case)]

/* Exports */
mod util;

pub use util::Graph as Graph;
pub use util::TripleStore as TripleStore;
pub use util::OjiQuery as OjiQuery;
pub use util::OjiQueryUnit as OjiQueryUnit;
pub use util::OjiResult as OjiResult;
pub use util::OjiResultUnit as OjiResultUnit;
pub use util::OjiResultCollection as OjiResultCollection;

#[cfg(test)]
mod manual_tests {
  use super::*;
  #[test]
  fn gabe() {
    let mut g = Graph::new();
    g.add(("Gabe".into(), "is".into(), "male".into()));
    g.add(("James".into(), "is".into(), "cool".into()));
    g.add(("Gabe".into(), "is".into(), "cool".into()));
    let q = OjiQuery::new().from(&g)
                           .select(&["$name"])
                           .filter(&[("$name", "is", "cool")])
                           .fetch();
    println!("{:#?}", q);
  }
}

#[cfg(test)]
mod graph_basic {

  use super::*;
  use super::util::TripleRefIter as TripleRefIter;

  #[test]
  fn create_graph() {
      let g = Graph::new();
      let expected_g = Graph {
        spo: TripleStore::new(),
        pos: TripleStore::new(),
        osp: TripleStore::new(),
      };
      assert_eq!(g, expected_g);
  }
  #[test]
  fn insert_triples() {
    let mut g = Graph::new();
    g.add(("Gabe".into(), "likes".into(), "Rust".into()));
    g.add(("Gabe".into(), "likes".into(), "C++".into()));
    g.add(("Gabe".into(), "likes".into(), "Scala".into()));
    g.add(("Matt".into(), "likes".into(), "JS".into()));
    g.add(("James".into(), "likes".into(), "Java".into()));
    g.add(("James".into(), "likes".into(), "C#".into()));
    g.add(("Gabe".into(), "likes".into(), "James".into()));
    let empty_g = Graph {
      spo: TripleStore::new(),
      pos: TripleStore::new(),
      osp: TripleStore::new(),
    };
    assert_ne!(g, empty_g);
  }
  #[test]
  fn insert_dupes_across_triplestores() {
    let mut g = Graph::new();
    g.add(("Gabe".into(), "likes".into(), "Rust".into()));
    let expected_spo = TripleStore(
      vec!((String::from("Gabe"),
        Box::new(
          vec!((String::from("likes"),
            Box::new(
              vec!(String::from("Rust"))
            )
          ))
        )
      ))
    );
    let expected_pos = TripleStore(
      vec!((String::from("likes"),
        Box::new(
          vec!((String::from("Rust"),
            Box::new(
              vec!(String::from("Gabe"))
            )
          ))
        )
      ))
    );
    let expected_osp = TripleStore(
      vec!((String::from("Rust"),
        Box::new(
          vec!((String::from("Gabe"),
            Box::new(
              vec!(String::from("likes"))
            )
          ))
        )
      ))
    );
    let expected_g = Graph {
      spo: expected_spo,
      pos: expected_pos,
      osp: expected_osp,
    };
    assert_eq!(g, expected_g);
  }
  #[test]
  fn insert_then_remove() {
    let mut g = Graph::new();
    let t = ("Gabe".into(), "likes".into(), "Rust".into());
    g.add(t.clone());
    g.erase(&t);
    let empty_g = Graph {
      spo: TripleStore::new(),
      pos: TripleStore::new(),
      osp: TripleStore::new(),
    };
    assert_eq!(g, empty_g);
  }
  #[test]
  fn remove_the_correct_triple() {
    let mut g = Graph::new();
    g.add(("Gabe".into(), "is".into(), "male".into()));
    let t = ("Gabe".into(), "likes".into(), "Rust".into());
    g.add(t.clone());
    g.erase(&t);
    let mut expected_g = Graph::new();
    expected_g.add(("Gabe".into(), "is".into(), "male".into()));
    assert_eq!(g, expected_g);
  }
  #[test]
  fn replace_triple() {
    let mut g = Graph::new();
    let old_t = ("Gabe".into(), "is".into(), "male".into());
    let new_t = ("Gabe".into(), "likes".into(), "Rust".into());
    g.add(old_t.clone());
    g.replace(&old_t, new_t.clone());
    let mut expected_g = Graph::new();
    expected_g.add(new_t.clone());
    assert_eq!(g, expected_g);
  }
  #[test]
  fn get_iterator() {
    let g = Graph::new();
    let g_iter = g.iter();
    let expected_iter = TripleRefIter {
      store: &g.spo,
      curr_head: 0,
      curr_mid: 0,
      curr_tail: 0,
    };
    assert_eq!(g_iter, expected_iter);
  }
}

#[cfg(test)]
mod result_create {
  use super::*;
  #[test]
  fn null_result() {
    use std::collections::HashMap;
    let r = OjiResult::new();
    let expected_r = OjiResult {
      values: Vec::new(),
      var_map: HashMap::new(),
    };
    assert_eq!(r, expected_r);
  }
  #[test]
  fn add_val() {
    use std::collections::HashMap;
    let mut r = OjiResult::new();
    r.add_val(OjiResultUnit::Val(String::from("Gabe")));
    let v = vec!(OjiResultUnit::Val(String::from("Gabe")));
    let expected_r = OjiResult {
      values: v,
      var_map: HashMap::new(),
    };
    assert_eq!(r, expected_r);
  }
  #[test]
  fn add_var() {
    use std::collections::HashMap;
    let mut r = OjiResult::new();
    r.add_var("name".into(), "Gabe".into());
    let v = vec!(OjiResultUnit::Val(String::from("Gabe")));
    let mut h: HashMap<String, usize> = HashMap::new();
    h.insert(String::from("name"), 0);
    let expected_r = OjiResult {
      values: v,
      var_map: h,
    };
    assert_eq!(r, expected_r);
  }
  #[test]
  fn get_var() {
    use std::collections::HashMap;
    let v = vec!(OjiResultUnit::Val(String::from("Gabe")));
    let mut h: HashMap<String, usize> = HashMap::new();
    h.insert(String::from("name"), 0);
    let r = OjiResult {
      values: v,
      var_map: h,
    };
    assert_eq!(r.get_var("name"), Some(String::from("Gabe")));
  }
}
