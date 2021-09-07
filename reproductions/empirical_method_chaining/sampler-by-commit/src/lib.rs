use std::path::Path;
use djanco::database::*;
use djanco::log::*;
use djanco_ext::*;
use djanco::csv::*;
use djanco::objects::*;
use chrono::{NaiveDateTime, Datelike};
use std::collections::BTreeMap;
use method_chains::MethodChaining;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::env;
use djanco::AttributeIterator;
use djanco::*;
use chrono::Utc;
use chrono::TimeZone;



pub fn has_commits_in_year(project: &ItemWithData<Project>, year: i32) -> bool {
    // Returns true if a project has a commit in a given year
    let commits = project.commits_with_data().unwrap_or_else(Vec::new);
    commits.into_iter()
        .flat_map(|commit: ItemWithData<Commit>| commit.author_timestamp())
        .map(|author_time| Utc.timestamp(author_time /*seconds*/, 0 /*nanos*/).date().year())
        .any(|commit_year| commit_year == year)
}

#[djanco(May, 2021, subsets(Generic))]
pub fn query_2010(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    // sample 3 times projects with commits in 2010
    query(database, log, output, 2010, 1); //sample with seed=1
    query(database, log, output, 2010, 2); //sample with seed=2
    query(database, log, output, 2010, 3) //sample with seed=3
}

#[djanco(May, 2021, subsets(Generic))]
pub fn query_2018(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    // sample 3 times projects with commits in 2018

    query(database, log, output, 2018,1); //sample with seed=1
    query(database, log, output, 2018,2); //sample with seed=2
    query(database, log, output, 2018,3) //sample with seed=3
}




pub fn query(database: &Database, _log: &Log, output: &Path, target_year: i32, seed: u128) -> Result<(), std::io::Error>  {
    
    // this function return the frequencies of method chains lengths for a given year. It samples 250 projects
    // using the  parameter specified at seed
    // results will be saved at chain `chain_lengths_YEAR_SEED.csv`

    let projects = database.projects().filter(|project| {
        has_commits_in_year(project, target_year) // gather projects that have a commit in a given year
    }).sample(Random(250, Seed(seed))); // sample 250 of these projects

    
    // open file to write
    let mut path: PathBuf = PathBuf::from(output);
    path.push(format!("chain_lengths_{}_{}", target_year, seed));
    path.set_extension("csv");
    let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)?;

    
    // write header of csv file
    writeln!(file, "project_id,year,chain_length,frequency")?;

    for project in projects {
        let project_id = project.id(); // save project id before it is borrowed
        let last_commits = get_year_end_revision(project, target_year); // get last commit from each year
        
        if last_commits.len() == 0 {
            // if a project have no commits we still want to output that there are no chain lengths
            // by using as chain length 0, and frequency 0
            writeln!(file,"{},{},{},{}", project_id, target_year, 0, 0)?;
        }

        // iterate through each year and the corresponding last commit
        for (year, commit) in last_commits {
            
            // get frequencies of the method chainings from the last commit of a project
            let chain_lengths = get_code_year_end_revision(commit, project_id);
            let size_chains = chain_lengths.len();
            for (chain_length, freq) in chain_lengths {
                writeln!(file,"{},{},{},{}", project_id, year, chain_length, freq)?;
            }

            if size_chains == 0 {
                // if a project does not outputs any frequency for any length of method chain
                // we indicate that by using as chain length 0, and frequency 0
                writeln!(file,"{},{},{},{}", project_id, year, 0, 0)?;
            }
        }
    }

   Ok(())
}

pub fn get_year_end_revision<'a>(project : ItemWithData<'a,Project>, target_year : i32 ) -> BTreeMap<i32, ItemWithData<'a,Commit> > {
    
    // used to map a year to all the commits that belong to that year
    let mut commits_per_year = BTreeMap::<i32, Vec<ItemWithData<Commit>> >::new();

    // used to map a year to the last commit in that year
    let mut last_commit_per_year =  BTreeMap::<i32, ItemWithData<Commit> >::new();

    if let Some(commits) = project.commits_with_data(){

        // First, group all the commits by year
        for commit in commits {
            if let Some(timestamp) = commit.committer_timestamp() {
                
                // get date object
                let time =  NaiveDateTime::from_timestamp(timestamp, 0);

                // get year from the date object
                let year = time.date().year();
                
                if year == target_year {
                    // we only care about commits that belong to a given years
                    commits_per_year.entry(year).or_insert(Vec::new()).push(commit);
                }   
            }
        }       

        // finds the latest commit per year
        for (year, commits) in commits_per_year {

            // get the last commit for current year
            let last_commit = commits.into_iter().max_by_key(|commit| {
                if let Some(timestamp) = commit.committer_timestamp() {
                    timestamp
                }else{
                    0 as i64
                }
                
            });
            
            // sets the last commit of the year in  BTreeMap if it exists
            if let Some(last_commit_unwrapped) = last_commit {
                if let Some(_timestamp) = last_commit_unwrapped.committer_timestamp() {
                    last_commit_per_year.insert(year, last_commit_unwrapped);
                }
            }
        }
    }

    // This function return the last commit for each year 
    last_commit_per_year    
}

pub fn get_code_year_end_revision<'a>(commit : ItemWithData<'a,Commit>, project_id: ProjectId) -> BTreeMap<usize, usize>{
    
    // get the tree from a given commit, we can get the files from the tree
    let tree = commit.tree_with_data();

    // we will save the all chain lengths to then group them to get the frequencies
    let mut chain_lengths = Vec::<usize>::new();

    let commit_id = commit.id();
    
    for change in tree.changes_with_data() {
        if let Some(snapshot) = change.snapshot() {

            let contents = snapshot.contents();
            let result = contents.method_chain_counts(1000); // the parameter specifies a recursion limit

            match result {
                Ok(chainings_counts) => {
                    // append the chain lengths
                    chain_lengths.extend(chainings_counts.into_iter())
                }
                Err(error) => {
                    eprintln!("Error processing snapshot for path {} (id={}) in commit {}: {}", 
                        change.path().map_or("<unknown>".to_owned(), |path| path.location()),
                        snapshot.id(),
                        commit.hash().unwrap_or("<unknown>".to_owned()),
                        error
                    )
                }
            }   
        }
    }
    
    // group method chaings by their lengths and return a BTreeMap that maps a length to its frequency
    chain_lengths
        .into_iter()
        .fold(
            BTreeMap::new(), 
            |mut accumulator, chain_length| {
                *accumulator.entry(chain_length).or_insert(0) += 1;
                accumulator
            }
        )
}