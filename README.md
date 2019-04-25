# OjiD - Ojibwe Dreamcatcher
## A GraphDB Designed to Prevent Data-Related Nightmares

OjiD, pronounced "Oh-Jee-Dee", is (or will be) a Graph Database implemented in Rust, C and WebAssembly in order to capitalise on their inherent benefits as tools; especially the increased control over how data is managed and runtime performance. It will be a fast solution to store and interact with RDF Triple data on the web, with the eventual aim of being published to NPM and allowing Javascript to interact with it as if it were ordinary JS.

### The aim of this project is to provide: 
 - **Import and Go**: To Allow users to set-up a fully-functional graph database in just a few lines of code.
 - **Seems Functional**: Provide all of the basic features a developer would come to expect from the average database.
 - **Seems Practical**: Provide some extra luxuries where possible.
 - **Boy is this Fast!**: All the performance benefits that comes with implementing with Rust and WebAssembly as opposed to JS.

### What this project will not aim to do: (yet)
 - **We're not Google**: My aim is not to provide an Enterprise-level database solution.
 - **I'm Only One Man**: I have no guarantees of OjiD working seamlessly with more than 1,000,000 RDF triples at once.

### What we do have: 
 - **Import and Go**: To set up a database simply import Web and call `Web::new()`.
 - **An Interface**: Full support for the essential CRUD actions on graph data.
 - **Tristore**: A fully implemented, and *slightly modified*, Hexastore architecture to provide lightning-fast data retrieval and a foundation for flexible queries.
 - **POINTERS**: All "branches" in the database are linked via pointers rather than existing within one object, meaning modification of one part of the data will never cause more than the local branch to be re-allocated.
 - **Bulk Re-allocation**: Branches will explicitly bulk allocate-deallocate when needed in order to minimise the frequency of large amounts of data being moved around.

### What we don't have: (yet)
 - **Query Chains**: Support for queries of arbitrary length.
 - **O(1) Lookup**: Store everything in HashMaps and HashSets rather than Vecs, without affecting the custom re-allocation.
 - **Documents!**: A persistence model using Amazon ION. 
   - Using C and Rust's FFI.
 - **Readable Documents!**: Optional import/export from/to JSON.
 - **Background Sorting**: Sorting of data to take place while saving to files.
   - Optimise operations to take advantage of the data being sorted.
   - Background sorting of data during periods of downtime using multi-threading.
 - **Embracing the Future**: A package that compiles to WebAssembly.
 - **The Mask**: An interface layer to allow NodeJS to interact with the package as if it's Javascript.
   - An NPM package to make this even easier.