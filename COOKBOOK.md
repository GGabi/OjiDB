
# OjiDB Cookbook
For those that dislike trawling through spaghetti code, this document will hopefully clarify the basics of:
- Setting up a Graph
- Adding and removing triples
- OjiQueries and OjiResults
- Querying the Graph

Note: This document contains everything I can remember to include, if something doesn't seem right or wasn't included raise a issue and I'll deal with it :sunglasses:
## Setting up a Graph
**Disclaimer:** *I've been focusing on getting the functionality done as much as possible, maybe one day I'll make it a proper crate for your convenience.*
1) Download this repo and move `src/lib` to the desired location on your filesystem. Rename the folder `lib` to be anything you like, or don't, it's totally up to you; I'll be referring to this folder as `OjiDB` for the remainder of this document.
(Protip: Maybe call it `OjiDB` for consistency's sake :eyes:)
2) In `main.rs`, or wherever you want to use OjiDB, add the following lines to the top of the file: 
```
mod OjiDB;
use OjiDB::Graph;
```
3) Create your first Graph  instance with the following:
```
let mut g = Graph:new()
```
4) And that's it!

**Fun Graph Facts!**
- It's actually just a struct containing 3 TripleStores (more on them later).
- Easily iterate over your triples in (Subject, Predicate, Object) ordering with `.iter()`
  - I've only made it possible to make a non-consuming iterator for a Graph; it doesn't make sense to consume a database whenever you want to read it now does it?
## Adding and Removing Triples
Simply add a triple to your graph with:
```
//fn add(t: Triple) {..}
let t = (String::from("Gabe"),
        String::from("likes"),
        String::from("Rust"));
g.add(t);
```
Remove a triple with:
```
//fn erase(t: Triple) {..}
let t = (String::from("Gabe"),
        String::from("likes"),
        String::from("James"));
g.erase(t);
```
Finally, replace a triple with:
```
//fn replace(old_t: Triple, new_t: Triple) {..}
let old_t = (String::from("Fire"),
            String::from("is"),
            String::from("cold"));
let new_t = (String::from("Fire"),
            String::from("is"),
            String::from("hot"));
g.replace(old_t, new_t);
```
## OjiQueries and OjiResults
What use would a database be without queries and results? Get them with `use OjiDB::{OjiQuery, OjiResult};`.
### QueryUnits and ResultUnits
Queries and Results are made up of structs, almost exactly how `Option<T>` behaves.
- **Query Units:**
  - String literal
  - Variable get
    - This will replaced by all matching entries found in the Graph and stored in the Result under the variable name specified.
    - Start a query parameter with '$' to make it a variable.
  - Anonymous get
    - This will be replaced by all matching entries found in the Graph but will not be stored under any variable name.
    - Start a query parameter with '?' to make it an anonymous variable.
  - Ignore
    - This is almost entirely used for processing under the hood.
    - If you really want one in your query, start a query parameter with '_' or leave it as a blank string.
- **Result Units:**
  - String literal
  - Ignore

### Creating a Query
There are 2 methods of constructing a query: `from_str()` and `from()`:
```
//from() Method
let param1 = OjiQueryUnit::from("$subject");
let param2 = OjiQueryUnit::from("?predicate");
let param3 = OjiQueryUnit::from("object");
let query = OjiQuery::from(&[param1, param2, param3]);
//from_str() Method
let query_str = OjiQuery::from_str(&["$subject", "?predicate", "object"]);
```
What the above query, `("$subject", "?predicate", "object")`, translates to is the following: "Give me every triple from the Graph that matches the pattern of: (Something, Something, "object"). In each Result, store the first part of the resulting triples under a variable called "subject" and don't bother assigning the middle part of the resulting triples to a named variable."
If that's a bit wordy, here's some examples of some possible results:
`(subject: "Gabe", "has an", "object")`
`(subject: "This", "is an", "object")`
### Reading Results
OjiResult structs come with the following methods, starting with the ones most users will end up using:
- `.get_anon(pos: usize)`, Returns a `Some(String` if a ResultUnit exists at the index `pos`.
- `.get_var(var: &str)`, Returns a `Some(String)` if the variable exists or it refers to a `ResultUnit::Value`, `None` if it doesn't exist or if it refers to a `ResultUnit::Ignore`.
- `.iter()`, Returns a non-consuming iterator for the contained ResultUnits.
And the rest...
- `OjiResult::new()`, Creates a blank Result.
- `.add_anon(val: ResultUnit)`, Adds a ResultUnit to the end of the Result.
- `.add_var(var: String, val: String)`, Adds a ResultUnit containing `val` and make it accessible via `.get_var(var)`.

One more thing:
- OjiResult will automatically Deref to a Vec\<ResultUnit\> in most cases, meaning that you can treat it as a fancy Vec with some extra getters.
### Aggregated Results
We also have a OjiResultCollection struct for queries that return multiple OjiResults. They have the following methods:
- TODO
- Psst... this thing Derefs to a Vec\<Result\>...

## Querying the Graph (Advanced and WIP)
TODO
