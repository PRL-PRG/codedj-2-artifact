# Pysmell
This tool runs under Python2. It is a clone of https://github.com/chenzhifei731/Pysmell.

First, you will need to clone the repositories at in a directory (i.e `OUTPUT_DIRECTORY`).

Then, you run `python metric_explore.py OUTPUT_DIRECTORY REPOSITORIES_TO_ANALYZE`. You have to be in  `pysmell/detection/` to run it.

Finally, the results will be at `OUTPUT_DIRECTORY`. You will have a file for each type of smell, and one file of summaries called `metric.txt`.


# cloner.sh
This script uses a parasite built-in tool to clone repositories.

run `bash cloner.sh PATH_TO_PROJECT_IDS_CSV OUTPUT_DIRECTORY`

- `PATH_TO_PROJECT_IDS_CSV ` is a csv of one column that contains the project_ids of a datastore. This project ids can be found at `artifacts/scent_dl/project_indices/` . 
- `OUTPUT_DIRECTORY` is the path where are the projects are going to be cloned
