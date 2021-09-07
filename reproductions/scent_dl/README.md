# The Scent of Deep Learning Code: An Empirical Study: Reproduction Package

In order to run the reproduction package you first must execute the file  `artifact/dataset.Rmd`.

Then, you can execute `scent_dl.Rmd`. 

## Contents of this reproduction package

A brief description of the contents are provided. For more information, each of the crates and the `tools/` folder contain additional documentation. 

- `dl_projects_cloner/`: Djanco crate. Clones projects as they were in April 1st 2020 into some directory. 
- `project_indices/`: contains CSVs of the project indices of the various samples we use. This indices are used to clone the repositories so the smells tool can be executed.
- `smells/`: Has 1 folder per sample. Each folder contains a files whose names are smells. These files contain the frequencies of that smells in the projects of a given sample.
- `tools/Pysmell`: The smell detector tool the authors use. We have modified it a little bit so it can receive as arguments the folder where the projects to analyze are, and where to save the output files.
- `tools/cloner.sh`: this script clones repositories as there currently are from main branch from a list of indices. We used the indices that are in `project_indices`.
- `traditional-queries`: Contains the queries to replicate a traditional selection.