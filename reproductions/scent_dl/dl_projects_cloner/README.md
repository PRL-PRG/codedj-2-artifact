# last-commit-cloner

A crate containing ![Djanco](https://github.com/PRL-PRG/djanco) queries.

This crate was created to clone the deep learning projects as they were in 
1st april 2020, in main branch. For debugging purposes, the standard output
prints the commit from which each project is cloned in a form of a csv.

## Djanco

Djanco is a query system for querying GitHub datasets downloaded by 
![Parasite](https://github.com/PRL-PRG/codedj-parasite) as part of the CodeDJ
project.

## Running the queries

To generate a harness for executing the queries in this crate install 
![cargo-djanco](https://github.com/PRL-PRG/cargo-djanco):

```bash
cargo install --git https://github.com/PRL-PRG/cargo-djanco
```

Then, generate the harness:

```bash
cargo djanco
``` 

This generates s source file `src/bin/djanco.rs`, which you can run to execute all your queries:

```bash
cargo run --bin djanco --release -- --dataset-path DATASET_LIVES_HERE --output-path WRITE_RESULTS_HERE 
```

# Template

The template file for the last-commit-cloner crate comes from 
![here](https://github.com/PRL-PRG/djanco-query-template). 

To create a new crate from the template, first install 
![cargo-generate](https://github.com/cargo-generate/cargo-generate):

```bash
cargo install cargo-generate
```

Then, use the `generate` command in cargo to create a new crate from the 
template:

```bash
cargo generate --git https://github.com/PRL-PRG/djanco-query-template --name my-query-crate
```

Then you can add your query functions. There's an example function in 
`src/lib.rs` to get you started.

