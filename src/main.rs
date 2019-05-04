mod lib;

use crate::lib::Web;

fn main() {
  let mut g = Web::new();

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
                  Some("likes".into()),
                  None
                ]);

  println!("{:#?}", g);
  println!("{:#?}", v);
}
