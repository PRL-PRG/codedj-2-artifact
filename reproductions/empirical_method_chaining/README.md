# "An Empirical Study of Method Chaining in Java": Reproduction Package
In order to run the reproduction package you first must execute the file  `artifact/dataset.Rmd`.

Then, you can execute `reproduction.Rmd`.

## Contents of this reproduction package

A brief description of the contents are provided. For more information, each of the crates contain additional documentation.

- `method-chaining/` : rust crate on djanco that is used to detect method chainings and report frequencies on each length.
- `top-starred-sampler/`: Djanco receipt to replicate the selection process of the authors.
- `query_method_chainings/`: rust crate that we used to compute frequencies of method chainings over the sample in terms of top starred projects (a.k.a replication of the sample process the authors did). This crate calls the `method-chaining/` one.
- `sampler-by-commit/`: Djanco receipt that samples the target population we had identified the authors want. Additionally, it also computes frequencies of lengths of method chainings. It uses as dependecy `method-chaining/`.

