# Artifact Contents

`dataset.Rmd` - dataset load & analysis. Loads the summary information for all projects from the datasets and prints their info. 
`stars-analysis.Rmd` - manual analysis of outliers in top stars selections. Must be executed after `dataset.Rmd`
`helpers.R` - helper functions for the paper
`latex-log.R` - helper functions for reporting numbers directly to latex 
`selection.Rmd` - analysis of the subsets obtained by 5% dataset and 5% top stars cutoffs. Must be executed after `dataset.Rmd`