
use djanco::time::Duration;
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
    let commits = project.commits_with_data().unwrap_or_else(Vec::new);
    commits.into_iter()
        .flat_map(|commit: ItemWithData<Commit>| commit.author_timestamp())
        .map(|author_time| Utc.timestamp(author_time /*seconds*/, 0 /*nanos*/).date().year())
        .any(|commit_year| commit_year == year)
}

pub fn query_2010(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    query(database, log, output, 2010, 1);
    query(database, log, output, 2010, 2); 
    query(database, log, output, 2010, 3)
}

pub fn query_2018(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    query(database, log, output, 2018,1);
    query(database, log, output, 2018,2);
    query(database, log, output, 2018,3)
    
}


#[djanco(May, 2021, subsets(Generic))]
pub fn query_developed_projects_2010(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    query_developed_projects(database, log, output, 2010,1);
    query_developed_projects(database, log, output, 2010,2);
    query_developed_projects(database, log, output, 2010,3);
    query_developed_projects(database, log, output, 2010,4);
    query_developed_projects(database, log, output, 2010,5);
    query_developed_projects(database, log, output, 2010,6);
    query_developed_projects(database, log, output, 2010,7);
    query_developed_projects(database, log, output, 2010,8);
    query_developed_projects(database, log, output, 2010,9);
    query_developed_projects(database, log, output, 2010,10)
}

#[djanco(May, 2021, subsets(Generic))]
pub fn query_developed_projects_2018(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    query_developed_projects(database, log, output, 2018,1);
    query_developed_projects(database, log, output, 2018,2);
    query_developed_projects(database, log, output, 2018,3);
    query_developed_projects(database, log, output, 2018,4);
    query_developed_projects(database, log, output, 2018,5);
    query_developed_projects(database, log, output, 2018,6);
    query_developed_projects(database, log, output, 2018,7);
    query_developed_projects(database, log, output, 2018,8);
    query_developed_projects(database, log, output, 2018,9);
    query_developed_projects(database, log, output, 2018,10)
}

pub fn query_developed_projects(database: &Database, _log: &Log, output: &Path, target_year: i32, seed: u128) -> Result<(), std::io::Error>  {
    // load from file projects.csv the ids of projects we are interested on
    let ids_path = env::var("IDS_PATH").unwrap();
    let project_ids: Vec<ProjectId> = ProjectId::from_csv(ids_path).unwrap();
    let projects = database.projects().filter(|project| {
                        project_ids.contains(&project.id())
                    })
                    .filter_by(Equal(project::Language, Language::Java))
                    .filter_by(AtLeast(project::MaxHIndex1, 3))
                    .filter_by(AtLeast(project::Age, Duration::from_days(364)))
                    .filter_by(AtLeast(Count(project::Users), 3))
                    .filter_by(AtLeast(project::Locs, 716))
                    .filter_by(AtLeast(Count(project::Snapshots), 20))
                    .filter_by(AtLeast(Count(project::Commits), 26))
                    .filter(|project| {
                                has_commits_in_year(project, target_year)
                    }).sample(Random(250, Seed(seed)));

    // open file to write
    let mut path: PathBuf = PathBuf::from(output);
    path.push(format!("dev_chain_lengths_{}_{}", target_year, seed));
    path.set_extension("csv");

    let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)?;

    // write header of csv file
    writeln!(file, "project_id,year,chain_length,frequency")?;

    for project in projects {
        let project_id = project.id(); // save project id before it is borrowed
        let last_commits = get_year_end_revision(project, target_year);
        
        if last_commits.len() == 0 {
            println!("no last commits");
            writeln!(file,"{},{},{},{}", project_id, target_year, 0, 0)?;
        }
        for (year, commit) in last_commits {
            let chain_lengths = get_code_year_end_revision(commit, project_id);
            let size_chains = chain_lengths.len();
            for (chain_length, freq) in chain_lengths {
                writeln!(file,"{},{},{},{}", project_id, year, chain_length, freq)?;
            }

            if size_chains == 0 {
                writeln!(file,"{},{},{},{}", project_id, year, 0, 0)?;
            }
        }
    }

   Ok(())
}

pub fn query(database: &Database, _log: &Log, output: &Path, target_year: i32, seed: u128) -> Result<(), std::io::Error>  {

    // load from file projects.csv the ids of projects we are interested on
    let ids_path = env::var("IDS_PATH").unwrap();
    println!("{:?}", ids_path);
    let project_ids: Vec<ProjectId> = ProjectId::from_csv(ids_path).unwrap();

    let projects = database.projects().filter(|project| {
        project_ids.contains(&project.id())
    }).filter(|project| {
        has_commits_in_year(project, target_year)
    }).sample(Random(250, Seed(seed)));
    println!("({}) initial length: {}",target_year, projects.len());
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
        let last_commits = get_year_end_revision(project, target_year);
        
        if last_commits.len() == 0 {
            println!("no last commits");
            writeln!(file,"{},{},{},{}", project_id, target_year, 0, 0)?;
        }
        for (year, commit) in last_commits {
            let chain_lengths = get_code_year_end_revision(commit, project_id);
            let size_chains = chain_lengths.len();
            for (chain_length, freq) in chain_lengths {
                writeln!(file,"{},{},{},{}", project_id, year, chain_length, freq)?;
            }

            if size_chains == 0 {
                writeln!(file,"{},{},{},{}", project_id, year, 0, 0)?;
            }
        }
    }

   Ok(())
}

pub fn get_year_end_revision<'a>(project : ItemWithData<'a,Project>, target_year : i32 ) -> BTreeMap<i32, ItemWithData<'a,Commit> > {
    
    let mut commits_per_year = BTreeMap::<i32, Vec<ItemWithData<Commit>> >::new();
    let mut last_commit_per_year =  BTreeMap::<i32, ItemWithData<Commit> >::new();

    if let Some(commits) = project.commits_with_data(){
        // First, group all the commits by year
        for commit in commits {

            if let Some(timestamp) = commit.committer_timestamp() {
                let time =  NaiveDateTime::from_timestamp(timestamp, 0);
                let year = time.date().year();
                if year == target_year {
                    commits_per_year.entry(year).or_insert(Vec::new()).push(commit);
                }
                    
            }else{
                println!("null timestamp");
            }
        }       

        // finds the latest commit per year
        for (year, commits) in commits_per_year {
            let last_commit = commits.into_iter().max_by_key(|commit| {
                if let Some(timestamp) = commit.committer_timestamp() {
                    timestamp
                }else{
                    0 as i64
                }
                
            });
            
            if let Some(last_commit_unwrapped) = last_commit {
                if let Some(_timestamp) = last_commit_unwrapped.committer_timestamp() {
                    last_commit_per_year.insert(year, last_commit_unwrapped);
                }else{
                    println!("null _timestamp");
                }   
            }else{
                println!("null last_commit_unwrapped");
            }
        }
    }else{
        println!("null commits");
    }

    // This function return the last commit for each year 
    last_commit_per_year    
}

pub fn get_code_year_end_revision<'a>(commit : ItemWithData<'a,Commit>, project_id: ProjectId) -> BTreeMap<usize, usize>{
    
    let tree = commit.tree_with_data();
    let mut chain_lengths = Vec::<usize>::new();
    let commit_id = commit.id();
    for change in tree.changes_with_data() {
        if let Some(snapshot) = change.snapshot() {
            // let file = change.path().unwrap().location();
            let contents = snapshot.contents();
            let result = contents.method_chain_counts(1000);

            match result {
                Ok(chainings_counts) => {
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