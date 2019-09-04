# OjiDB - Ojibwe Database
## A GraphDB Designed to Prevent Data-Related Nightmares

Oji, pronounced "Oh-Jee", is (or will be) a Graph Database implemented in Rust, C and WebAssembly in order to capitalise on their inherent benefits as tools; especially the increased control over how data is managed and runtime performance. It will be a fast solution to store and interact with RDF Triple data on the web, with the eventual aim of being published to NPM and allowing Javascript to interact with it as if it were ordinary JS.

### The aim of this project is to provide: 
 - **Import and Go**: To allow users to set-up a fully-functional graph database in just a few lines of code.
 - **Seems Functional**: Provide all of the basic features a developer would come to expect from the average database.
 - **Seems Practical**: Provide some extra luxuries where possible.
 - **Lightning Fast**: All the performance benefits that comes with implementing with Rust and WebAssembly as opposed to JS.

### What this project will not aim to do: (yet)
 - **We're not Google**: My aim is not to provide an Enterprise-level database solution.
 - **I'm Only One Man**: I have no guarantees of Oji working seamlessly with more than 1,000,000 RDF triples at once.

### What we do have: 
 - **Import and Go**: To set up a database simply import OjiDB and call `Graph::new()`.
 - **An Interface**: Full support for the essential CRUD actions on graph data.
 - **Tristore**: A fully implemented, and *slightly modified*, Hexastore architecture to provide lightning-fast data retrieval and a foundation for flexible queries.
 - **POINTERS**: All "branches" in the database exist separately on the heap rather than existing within one object, meaning modification of one part of the data will never cause more than the local branch to be re-allocated.
 - **O(1) Lookup**: Store everything in HashMaps and HashSets.
 - **Readable Documents!**: Optional import/export from/to JSON using the `.json()`, `.into_json()` and `.from_json(data: String)` methods on both Graph and TripleStore. Graph's methods will take and produce TripleStore JSON documents while handling all the multi-store stuff for you.

### What we don't have: (yet)
 - **Fancy Interactions**: Structs and types that allow for more sophisticated interactions with Oji.
 - **Query Chains**: Support for queries of arbitrary length.
 - **Faster Documents!**: A persistence model using Amazon ION. 
   - Using C and Rust's FFI.
 - **Embracing Standards**: Support for SPARQL-esque queries.
 - **Background Sorting**: Sorting of data to take place while saving to files.
   - Optimise operations to take advantage of the data being sorted.
   - Background sorting of data during periods of downtime using multi-threading.
 - **Embracing the Future**: A package that compiles to WebAssembly.
 - **OjiJS**: An interface layer to allow NodeJS to interact with the package as if it's Javascript.
   - An NPM package to make this even easier.
