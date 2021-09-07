use std::path::Path;

use djanco::*;
use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco_ext::*;


#[djanco(May, 2021, subsets(Generic))]
pub fn starred_query(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .sort_by(project::Stars)
        .sample(Top(106)) // gets the top 106 starred projects
        .sample(Random(57, Seed(43))) // samples 57 of them
        .into_csv_in_dir(output, "top_starred.csv")
}


#[djanco(May, 2021, subsets(Generic))]
pub fn custom_query(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {

    // filter projects with: 
        // max_hindex1 >= 5 | lifetime >= 180 | locs >= 10000 | commits >= 100

    // perform stratified sample

    // samples 6 small projects (locs>=4000)
    database.projects().filter(|project| {
            // BEGIN checking if all the attributes we need exist
            if let Some(hindex) = project.max_h_index1() {
                if let Some(locs) = project.project_locs() {
                    if let Some(oldest_commit) = project.time_since_first_commit() {
                        if let Some(newest_commit) = project.time_since_last_commit() {
                            if let Some(latest_update_time) = project.latest_update_time() {
                                if let Some(commits) = project.commit_count() {
                                    // END checking if all the attributes we need exist

                                    let lifetime = (( (newest_commit as f64).min( latest_update_time as f64) - (oldest_commit as f64)) / 3600.0 / 24.0).ceil();
                                    return hindex >=5 || lifetime >=180.0 || locs>=10000 || commits >= 100;
                                }
                            }
                        }
                    }
                }
            }
            false
        })
        .filter_by(
            AtMost(project::Locs,4000)
        )
        .sample(Random(6, Seed(43)))
        .into_csv_in_dir(output, "custom_small.csv")?;
    
    // samples 13 medium projects (4000<locs<=15000)
    database.projects().filter(|project| {
            // BEGIN checking if all the attributes we need exist
            if let Some(hindex) = project.max_h_index1() {
                if let Some(locs) = project.project_locs() {
                    if let Some(oldest_commit) = project.time_since_first_commit() {
                        if let Some(newest_commit) = project.time_since_last_commit() {
                            if let Some(latest_update_time) = project.latest_update_time() {
                                if let Some(commits) = project.commit_count() {
                                    // END checking if all the attributes we need exist

                                    let lifetime = (( (newest_commit as f64).min( latest_update_time as f64) - (oldest_commit as f64)) / 3600.0 / 24.0).ceil();
                                    if hindex >=5 || lifetime >=180.0 || locs>=10000 || commits >= 100 {
                                        return true
                                    }
                                }
                            }
                        }
                    }
                }
            }
            false
        })
        .filter_by(
            MoreThan(project::Locs,4000)
        )
        .filter_by(
            AtMost(project::Locs,15000)
        )
        .sample(Random(13, Seed(43)))
        .into_csv_in_dir(output, "custom_medium.csv")?;

    // samples 38 large projects (locs>15000)
    database.projects().filter(|project| {

            // BEGIN checking if all the attributes we need exist
            if let Some(hindex) = project.max_h_index1() {
                if let Some(locs) = project.project_locs() {
                    if let Some(oldest_commit) = project.time_since_first_commit() {
                        if let Some(newest_commit) = project.time_since_last_commit() {
                            if let Some(latest_update_time) = project.latest_update_time() {
                                if let Some(commits) = project.commit_count() {
                                    // END checking if all the attributes we need exist

                                    let lifetime = (( (newest_commit as f64).min( latest_update_time as f64) - (oldest_commit as f64)) / 3600.0 / 24.0).ceil();
                                    if hindex >=5 || lifetime >=180.0 || locs>=10000 || commits >= 100 {
                                        return true
                                    }
                                }
                            }
                        }
                    }
                }
            }
            false
        })
        .filter_by(
            MoreThan(project::Locs,15000)
        )
        .sample(Random(38, Seed(43)))
        .into_csv_in_dir(output, "custom_large.csv")
}
