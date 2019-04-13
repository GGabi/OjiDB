
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

	println!("{:#?}", g);
}
