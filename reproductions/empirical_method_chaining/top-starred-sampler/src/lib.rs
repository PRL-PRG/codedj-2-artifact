use std::path::Path;

use djanco::*;
use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco_ext::*;
use chrono::{NaiveDateTime, Datelike};

#[djanco(May, 2021, subsets(Generic))]
pub fn my_query(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    // this query samples 3k projects from a population that is formed by stratifying sampling 
    // the top 1k starred projects for each creation year that goes from 2007 to 2018.

    // variable that will store the top 1k starred projects per year
    let mut all_projects : Vec<djanco::objects::ItemWithData<djanco::objects::Project>> = vec![];

    // we are intested in projects from 2007 to 2018
    for year in 2007..2019 {
        let projects = (database.projects()
            .filter( |project| {
                if let Some(created) = project.created() {
                    let time =  NaiveDateTime::from_timestamp(created, 0);
                    let project_year = time.date().year();

                    return project_year == year;
                }
                false
            })) // at this point we only have projects that are created in a given year
            // we sample the top 1k starred of them.
            .sort_by(project::Stars)
            .sample(Top(1000));
            all_projects.extend(projects);
    }

    all_projects
        .into_iter()
        .sample(Random(3000, Seed(43))) // sample 3k of them
        .into_csv_in_dir(output, "top_starred.csv")
}
