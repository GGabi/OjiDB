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

  let v = g.get(&[None,
                  Some("likes".into()),
                  None,
                  None,
                  None,
                ]);

  println!("{:#?}", g.spo);
  println!("{:#?}", v);

  for triple in g.iter() {
    println!("{:?}", triple);
  }

  let q = DBQuery::from_str(&["James", "$opinion", "$lang"]).unwrap();
  println!("{:?}", q); //Prints Double(Val("Gabe"), Var("value"))
  let r = g.get_trial(q);
  println!("{:#?}", r);
  println!("{:?}", r.get_var("opinion")); //Prints "likes"
  println!("{:?}", r.get_var("lang")); //Prints "likes"
}
