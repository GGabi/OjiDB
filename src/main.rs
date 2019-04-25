
// OjiD
// A web that wards off, rather than causing, data related nightmares

mod util;

use crate::util::{Web, Triple, QueryTriple};

//I am Gabe, Gabe likes Rust
//I --> am --> Gabe --> likes --> Rust
//[a, b, ..., y, z]
//get(a, b, c)
//c, d, e

fn main() {
  let mut g = Web::new();

  g.add(&Triple::from(("Gabe", "likes", "Rust")));
  g.add(&Triple::from(("Rust", "is", "fast")));
  g.add(&Triple::from(("Java", "is", "slow")));
  g.add(&Triple::from(("Java", "is", "bad")));
  g.add(&Triple::from(("Java", "is", "old")));
  g.add(&Triple::from(("C++", "is", "fast")));
  g.add(&Triple::from(("C++", "is", "hard")));
  g.add(&Triple::from(("fast", "is", "good")));
  g.add(&Triple::from(("slow", "is", "bad")));
  g.add(&Triple::from(("Gabe", "likes", "Scala")));
  g.add(&Triple::from(("Gabe", "likes", "Java")));
  g.add(&Triple::from(("Gabe", "likes", "Python")));
  g.add(&Triple::from(("Gabe", "likes", "C")));
  g.add(&Triple::from(("Gabe", "likes", "C++")));
  g.add(&Triple::from(("Gabe", "likes", "JS")));
  g.add(&Triple::from(("Gabe", "is", "short")));
  g.add(&Triple::from(("Gabe", "hates", "PHP")));
  g.add(&Triple::from(("Matt", "likes", "Node")));
  g.erase(&Triple::from(("Gabe", "likes", "Scala")));

  let v = g.get_chain(&[
                Some(String::from("Gabe")),
                None,
                None,
                None,
                None,
                None,
                None
               ]);

  // println!("{:#?}", g);
  println!("{:#?}", v);
}
