
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
