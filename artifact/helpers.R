# Really dumb R wrapper for codeDJ. At the moment just collection of some extremely basic functions. Should grow into something more useful in time. Calls the codedj executables under the hood. 

library("readr")
library("dplyr")
library("tidyr")
library("maditr")
library("ggplot2")
library("ggridges")
library("GGally")
library("ggmulti")
library("factoextra")
library("DT")
library("scales")
library("hexbin")
#library("ggExtra")
library("ggpubr")
library("patchwork")
library("ggalluvial")

pct = function(x, total, rounding = 1, noPct = F) {
  x = round(x * 100 / total, rounding)
  if (noPct) {
      x
  } else {
    paste0(x, "%")
  }
}

to_hours = function(x, rounding = 0) {
    round(x / 3600, rounding)
}

to_days = function(x, rounding = 0) {
    round(x / (3600 * 24), rounding)
}

# performs no normalization, a specially named identity function
normalize_none = function(m) {
  m
}
# linear normalization that makes all the values fit inside the <0,1> interval, stretching, or compressing as needed.
normalize_linear <- function(m){
  (m - min(m))/(max(m)-min(m))
}

# normalizes the values by their order based on unique values. 
normalize_rank = function(m) {
  if (is.logical(m))
    return(m)
  # create a factor, then order the factor
  f = sort(unique(m))
  t = 1:length(f)
  names(t) = f
  recode(m, !!!t)
}

# log10 scales the data
normalize_log <- function(m) {
  if (max(m) > 6) {
    if (min(m) == 0)
      m = m + 1
    m = log10(m)
  }
  m
}

normalize_log_linear <- function(m) {
  normalize_linear(normalize_log(m))
}

normalize_rank_linear = function(m) {
  normalize_linear(normalize_rank(m))
}

normalize = function(data, normalizer = normalize_linear, exclude_columns = c("pid")) {
    # now normalize the columns
    for (column in names(data)) {
      if (column %in% exclude_columns)
        next;
      cat(paste0("Normalizing column ", column, "\n"))
      data[[column]] = normalizer(data[[column]])
    }
    data
}

exp10 = function(x) {
  x ** 10
}


# returns the summary table of the projects in the dataset. See djanco's information for more details. 
#
# on top of the basic djanco table, we also perform some minor tidying such as renaming columns, adding units and creating extra variables
get_project_summaries = function(filename) {
    projects = read_csv(filename)
    projects = projects %>% rename(
      "pid" = "project_id",
      # this is just snapshots for which we have data I think... do not care really
      #"snapshots" = "snapshots",
      "all_forks" = "all_forks_count",
      "max_c_delta" = "max_commit_delta",
      "avg_c_delta" = "avg_commit_delta",
      "lang_ratio" = "major_language_ratio",
      "f_unique" = "unique_files",
      "f_orig" = "original_files",
      "dup" = "duplicated_code",
      "exp_max" = "max_experience",
      "exp" = "project_experience",
    )
    projects = projects %>% mutate(
        exp = round(exp, 2),
        lifetime = to_days(lifetime),
        avg_c_delta = to_days(avg_c_delta, 2),
        max_c_delta = to_days(max_c_delta, 2),
        # TODO this is an approximation, what we really want is to have the last commit distance from the time we checked the project. Here we check it from the latest code in the database, which will make some projects look older than they are really, but for now, it is a reasonable approximation
        last_commit = to_days(max(last_commit) - last_commit),
        lang_ratio = round(lang_ratio, 2),
    )
    projects = projects %>% mutate(
        a_commits_95 = round(`authors_contributing_95%_commits` / authors, 2),
        a_commits_80 = round(`authors_contributing_80%_commits` / authors, 2),
        a_commits_50 = round(`authors_contributing_50%_commits` / authors, 2),
        a_changes_95 = round(`authors_contributing_95%_changes` / authors, 2),
        a_changes_80 = round(`authors_contributing_80%_changes` / authors, 2),
        a_changes_50 = round(`authors_contributing_50%_changes` / authors, 2),
        originality = round(f_orig / files, 2),
        uniqueness = round(f_unique / files, 2),
        avg_impact = round(impact/files, 2)
    )
    projects
}
