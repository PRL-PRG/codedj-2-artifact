# method-chaining-query

A crate containing ![Djanco](https://github.com/PRL-PRG/djanco) queries.

This crate is used to get the frequency of length of method chains. 
It only process projects whose ID is in some file. The path of the file
is stored in an enviromental variable called `PATH_TO_IDS`.

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
IDS_PATH=PATH_TO_IDS cargo run --bin djanco --release -- --dataset-path DATASET_LIVES_HERE --output-path WRITE_RESULTS_HERE 
```

`PATH_TO_IDS` must be a csv with a single column. The column name must be `snapshot_id`

# Template

The template file for the method-chaining-query crate comes from 
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

