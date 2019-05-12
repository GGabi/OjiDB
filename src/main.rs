mod lib;

use lib::{Graph, DBQuery};

fn main() {
  let mut g = Graph::new();

  g.add(("Gabe".into(), "likes".into(), "Rust".into()));
  g.add(("Gabe".into(), "likes".into(), "C++".into()));
  g.add(("Gabe".into(), "likes".into(), "Scala".into()));
  g.add(("Gabe".into(), "likes".into(), "Python".into()));
  g.add(("Gabe".into(), "likes".into(), "Kotlin".into()));
  g.add(("Gabe".into(), "likes".into(), "Rust".into()));
  g.add(("Gabe".into(), "hates".into(), "Java".into()));
  g.add(("Gabe".into(), "hates".into(), "Pascal".into()));
  g.add(("Gabe".into(), "hates".into(), "Perl5".into()));
  g.add(("Matt".into(), "likes".into(), "JS".into()));
  g.add(("James".into(), "likes".into(), "Java".into()));
  g.add(("James".into(), "likes".into(), "C#".into()));
  g.add(("James".into(), "likes".into(), "C++".into()));
  g.add(("Gabe".into(), "likes".into(), "James".into()));

  println!("Graph Structure:\n{:#?}", g.spo);

  println!("\nList of Triples in Graph:");
  for triple in g.iter() {
    println!("{:?}", triple);
  }
  let q = [None,
           Some("likes".into()),
           None,
           None,
           None];
  let v = g.get(&q);
  println!("\nBasic Query:\n{:?}", q);
  println!("Basic Query Results:\n{:#?}", v);

  let q = DBQuery::from_str(&["?", "$opinion", "$object"]);
  let rc = g.get_trial(q);
  println!("\nBetter Query:\n{:?}\nResults:", rc.query);
  for r in &rc {
    println!("{:?}", r);
  }
  println!("\nInterpretation using vars: (Someone $opinion $object)");
  for r in rc.iter() {
    println!("Someone {} {}", r.get_var("opinion").unwrap(), r.get_var("object").unwrap());
  }
}
