use std::path::Path;

use djanco::*;
use djanco::objects::*;
use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco_ext::*;

use chrono::{NaiveDate, NaiveDateTime};




#[djanco(May, 2021, subsets(All))]
pub fn my_query(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    // This query clones projects as they were in April 1st, 2020. The clones is from the main branch.
   
    // create a date object with date April 1st 2020 00:00 hours
    let naive_target_date: NaiveDateTime = NaiveDate::from_ymd(2020, 4, 1).and_hms(0, 0, 0);

    // get timestamp
    let unix_target_date: i64 = naive_target_date.timestamp();

    // prints the header of the csv. This is for debug purposes.
    println!("project_id,url,commit_hash");

    // iterate all the projects in the datastore
    for project in database.projects() {
        
        // get commits from main branch 
        if let Some(commits) = project.main_branch_commits_with_data() {
            let wrapped_just_the_one_commit = commits.into_iter().min_by_key(|commit| {
                // search for the closest commit to 1st april that happened before this date.
                // therefore, we are interested in the smallest difference of time between this date and 
                // date where the commit happened.
                if let Some(timestamp) = commit.committer_timestamp() {
                    let diff = unix_target_date - timestamp;

                    if diff < 0 {
                        // if the timestamp we return the timestamp itself which must always be higher that the difference
                        timestamp
                    }else {
                        // return difference
                        return diff
                    }
                }else{
                    // if the commit does not has timestamp we return the actual timestamp. A very high one.
                    1627986007 as i64
                }
                
            });

            // process the commit commit closest to 1st April, 2020. 
            if let Some(just_the_one_commit) = wrapped_just_the_one_commit {

                // print values of the csv. (DEBUG purposes)
                println!("{},{},{}", project.id(), project.url(), just_the_one_commit.hash().unwrap());

                // open path in which the projects are going to be cloned
                let mut project_dir = std::path::PathBuf::from(&output);
                project_dir.push(project.id().to_string());
                std::fs::create_dir_all(&project_dir).unwrap();

                // get all te files and put them in the output directory.
                let project_tree: ItemWithData<Tree> = just_the_one_commit.tree_with_data();
                project_tree.into_files_in_dir(&project_dir).unwrap();
            }

        }else{
            //println!("No Commits in main branch for project_id={}", project.id());
        }
    }

    Ok(())
}
