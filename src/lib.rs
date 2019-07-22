
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
mod tests {

  use super::util::Graph as Graph;
  use super::util::TripleStore as TripleStore;

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
}
