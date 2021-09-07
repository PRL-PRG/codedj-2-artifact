use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use chrono::Datelike;
use chrono::TimeZone;
use chrono::Utc;
use regex;
use ::csv as csv_reader;
use serde::Deserialize;

use djanco::*;

use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco::objects::*;

use djanco::time::Duration;
use djanco_ext::*;
use regex::Regex;

const SELECTIONS: usize = 10;
const SELECTED_PROJECTS: usize = 30;
const SEEDS: [u128; 10] = [1,2,3,5,7,11,13,17,19,23]; // one seed per selection

// Base commit is going to be a commit this many percent commits in the past.
//
// Eg. if there are 12 commits and BASE_COMMIT_OFFSET_RATIO is 25, then
// the base commit will be 12 * 25 / 100 = 3 commits pushed back from the head.
//
// A B C D E F G H I J K L
//                 ^     ^
//                 |     |
//                 BASE  HEAD
//
// All math is done on integers.
const BASE_COMMIT_OFFSET_RATIO: usize = 10;

#[djanco(May, 2021, subsets(Generic))]
pub fn all_projects(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .into_csv_in_dir(output, "info/javascript_projects.csv")
}

#[djanco(May, 2021, subsets(Generic))]
pub fn all_projects_extended(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .into_extended_csv_in_dir(output, "info/javascript_projects_extended.csv")
}

#[djanco(May, 2021, subsets(Generic))]
pub fn project_locs(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
    .map_into(Select!(project::URL, project::Locs))
    .into_csv_with_headers_in_dir(vec!["url", "locs"], output, "info/project_locs.csv")
}

#[macro_export]
macro_rules! one_per_selection {
    {$function:ident ($database:ident, $log:ident, $output:ident)} => {{
        for i in 0..SELECTIONS {
            $function($database, $log, $output, i)?;
        }
        Ok(())
    }}
}
 
#[djanco(May, 2021, subsets(Generic))]
pub fn select_quality_projects(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    one_per_selection!{ quality_projects(database, log, output) }
}

pub fn quality_projects(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error>  {
    database.projects()
        // Contains at least 80% JavaScript code
        .filter(|project| {
            project.language_composition().map_or(false, |languages| {
                languages.into_iter()
                    .any(|(language, propotion)| {
                        language == Language::JavaScript && propotion >= 80
                    })
            })
        })
        // Contains at least 5KLOC in the head tree.
        .filter_by(AtLeast(project::Locs, 5_000))
        // The spanm between first and last commit is at least 1 year
        .filter_by(AtLeast(project::Age, Duration::from_months(12)))
        // Contains at least 100 commits total
        .filter_by(AtLeast(Count(project::Commits), 100))        
        // Has at least 2 users
        .filter_by(AtLeast(Count(project::Users), 2))
        // Sample N projects at random (we're just going to do one selection, so take first seed)        
        .sample(Random(SELECTED_PROJECTS, Seed(SEEDS[seed_index]))) 
        // Extract: url, head commit aka to, base commit aka from
        .map_into(project::URL)
        .into_csv_with_headers_in_dir(vec!["url"], 
            output, 
            format!("selections/quality_projects_{}.csv", seed_index))
}

#[djanco(May, 2021, subsets(Generic))]
pub fn select_original_projects(_database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    // Selection from the style-analyzer paper
    vec!["https://github.com/googlechromelabs/carlo.git",
         "https://github.com/facebook/react.git",
         "https://github.com/reduxjs/redux.git",
         "https://github.com/axios/axios.git",
         "https://github.com/hakimel/reveal.js.git",
         "https://github.com/storybookjs/storybook.git",
         "https://github.com/nodejs/node.git",
         "https://github.com/jquery/jquery.git",
         "https://github.com/laravel/telescope.git",
         "https://github.com/meteor/meteor.git",
         "https://github.com/evergreen-ci/evergreen.git",
         "https://github.com/facebook/create-react-app.git",
         "https://github.com/nodejs/citgm.git",
         "https://github.com/expressjs/express.git",
         "https://github.com/facebook/react-native.git",
         "https://github.com/30-seconds/30-seconds-of-code.git",
         "https://github.com/freeCodeCamp/freeCodeCamp.git",
         "https://github.com/atom/atom.git",
         "https://github.com/webpack/webpack.git"]
        .into_iter()
        .map(|url| url.to_owned())
        .into_csv_with_headers_in_dir(
            vec!["url"], output, 
            format!("selections/original.csv"))
}

#[derive(Deserialize, Debug, PartialEq, Eq, std::hash::Hash)]
struct Url { url: String }

#[djanco(May, 2021, subsets(Generic))]
pub fn generate_project_spec_form_selections(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut selections_dir = PathBuf::from(output);
    selections_dir.push("selections");
    
    let mut project_selection_assignments: HashMap<Url, Vec<String>> = HashMap::new();

    let extension = Regex::new(".csv$").unwrap();
    let selections = std::fs::read_dir(&selections_dir)?;
    for selection in selections {
        let selection = selection?;
        let selection_path = selection.path();
        let selection_file = selection.file_name().to_str().unwrap().to_owned();            
        let selection_name = extension.replace(&selection_file, "").to_string();

        let mut csv = csv_reader::Reader::from_path(selection_path)?;
        
        for row in csv.deserialize() {
            let project_url: Url = row?;

            project_selection_assignments.entry(project_url)
                .and_modify(|vector| vector.push(selection_name.clone()))
                .or_insert(vec![selection_name.clone()]);
        }
    }

    let project_specs = database.projects().into_iter()
        .map(|p| { println!(">> {}", p.url()); p })
        .flat_map(project_spec)
        .map(|p| { println!(">> {:?}", p); p })
        .map(|(url, to, from)| (url.clone(), (url, to, from)))
        .collect::<HashMap<String, (String, String, String)>>();

    let mut selection_specs: HashMap<String, Vec<(String, String, String)>> = HashMap::new();
    for (url, selections) in project_selection_assignments {
        let project_spec = project_specs.get(&url.url);
        if project_spec.is_none() {
            continue
        }
        let project_spec = project_spec.unwrap().clone();

        for selection in selections {
            selection_specs.entry(selection)
                .and_modify(|vector| vector.push(project_spec.clone()))
                .or_insert(vec![project_spec.clone()]);
        }
    }

    for (selection, project_specs) in selection_specs {
        project_specs.into_iter().into_csv_with_headers_in_dir(
            vec!["url","to","from"], 
            &output, 
            format!("specs/{}.csv", selection))?
    }

    Ok(())
}

// Helper functions:
type ProjectURL = String;
type CommitHash = String;

pub fn is_project_spec<'a>(project: &ItemWithData<'a, Project>) -> bool {
    _project_spec(project).is_some()
}
pub fn project_spec<'a>(project: ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
    _project_spec(&project)
}
pub fn _project_spec<'a>(project: &ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
    let url = project.url();
    
    let default_branch = project.default_branch();
    if default_branch.is_none() {
        eprintln!("WARNING: Default branch not found for project {} ({:?}), skipping.", project.id(), url);
        return None;
    }
    let default_branch = default_branch.unwrap();
    let default_branch_path = format!("refs/heads/{}", default_branch);

    let heads = project.heads_with_data();
    if heads.is_none() {
        eprintln!("WARNING: Heads not found for project {} ({:?}), skipping.", project.id(), url);
        return None;
    }
    let heads = heads.unwrap();

    let default_branch_head = 
        heads.into_iter()
            .filter(|head| head.name() == default_branch_path)            
            .collect::<Vec<ItemWithData<Head>>>();

    if default_branch_head.len() == 0 {
        eprintln!("WARNING: No branch {} found in project {} ({:?}), skipping.", default_branch, project.id(), url);
        return None;
    }
    if default_branch_head.len() > 1 {
        eprintln!("WARNING: More than one ({}) branch {} found in project {} ({:?}), continuing.", 
                  default_branch_head.len(), default_branch, project.id(), url);
    }
    let default_branch_head = default_branch_head[0].clone();   
    
    let head_commit_hash = default_branch_head.hash();    
    let mut commits = default_branch_head.commits_with_data();

    // Newest first.
    commits.sort_by_key(|commit| commit.committer_timestamp());
    commits.reverse();

    let total_commits = commits.len();
    let base_commit_offset = (BASE_COMMIT_OFFSET_RATIO * total_commits) / 100;

    eprintln!("INFO: Base commit offset is {} (of {}) for project {} ({:?})", 
              base_commit_offset, total_commits, project.id(), url);

    let base_commit = commits.iter().take(base_commit_offset).last();
    if base_commit.is_none() {
        eprintln!("WARNING: Base commit unavaiable for for branch {} in project {} ({:?}), skipping.", 
                  default_branch, project.id(), url);
        return None;
    }
    let base_commit = base_commit.unwrap();


    let base_commit_hash = base_commit.hash();
    if base_commit_hash.is_none() {
        eprintln!("WARNING: Base commit hash unavaiable for base commit {} from branch {} in project {} ({:?}), skipping.", 
                  base_commit.id(), default_branch, project.id(), url);
        return None;
    }
    let base_commit_hash = base_commit_hash.unwrap();

    return Some((url, head_commit_hash, base_commit_hash))
}


#[djanco(May, 2021, subsets(Generic))]
pub fn generate_project_spec_for_original_projects(_database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    // Selection from the style-analyzer paper, with original head and base commit hashes
    vec![("https://github.com/googlechromelabs/carlo.git",    "b8ce2bca042c757b13fc82a3e059980342ddd9a8","26262aad740b7255f17950251ae344b4823572a4"),
         ("https://github.com/facebook/react.git",            "1034e26fe5e42ba07492a736da7bdf5bf2108bc6","8ced545e3df95afab6fa35bc29f9320bafbcef26"),
         ("https://github.com/reduxjs/redux.git",             "902484ed735d38aec06683c847810a7218d8dba2","b307091af4d7e846d9e5f080fb81caf4a8b4aab1"),
         ("https://github.com/axios/axios.git",               "21ae22dbd3ae3d3a55d9efd4eead3dd7fb6d8e6e","138108ee56bd689305ae505a66b48d5e9c8aa494"),
         ("https://github.com/hakimel/reveal.js.git",         "0b3e7839ebf4ed8b6c180aca0abafa28c67aee6d","247771e129f431fc751140d8da4c2fe60815a51f"),
         ("https://github.com/storybookjs/storybook.git",     "b28217f887af533a17cb1498887d6b4bd41bd643","4c46f273719427788d568c037f285907aabd17f9"),
         ("https://github.com/nodejs/node.git",               "6eda924c189e44a36fc97a7cfae41b69483d5bfb","315b1c656cee39c989015cc2b17fe8c864dbc3dd"),
         ("https://github.com/jquery/jquery.git",             "dae5f3ce3d2df27873d01f0d9682f6a91ad66b87","d9a099a58e1bb1f158ea66ec55534770be442907"),
         ("https://github.com/laravel/telescope.git",         "534030114f47696fe3f3b08ea7ca49467428f2af","6f0a10ec586cfa1a22218b6778bf9c1572b97912"),
         ("https://github.com/meteor/meteor.git",             "c3309b123a7220ac24cbe73661184ee946bca01f","62fa9927ce34cff064cc3991439553e7c52b5258"),
         ("https://github.com/evergreen-ci/evergreen.git",    "ba22d511dad83c072842e47801ef42697d142f7c","1030eca5da38dce4e5047c23a3ea7fc0c246b8ce"),
         ("https://github.com/facebook/create-react-app.git", "32106d216e4c31fda30ec475f9f03186d116c893","ffc63d55976f9cbcce7f33dc7c45b3c2190a5924"),
         ("https://github.com/nodejs/citgm.git",              "0c4c7ccdd1cad8ce9506e34ca523787ba18cafe2","d21e2a87aaa9e9f50c6175eddc54054a32c64a24"),
         ("https://github.com/30-seconds/30-seconds-of-code", "3a122c9cfcbdc091227879a06a32bc67ccd0d35d","c8c60895e80b8bc90583502accdaa339b794609c"),
         ("https://github.com/expressjs/express.git" ,        "b4eb1f59d39d801d7365c86b04500f16faeb0b1c","56e90e3c7267782febe35754806ce3f63b527485"),
         ("https://github.com/freeCodeCamp/freeCodeCamp.git", "cf65516cce60645a417e44c4fcea7418ca920572","c353c4c659c3dcb19524ba893f170805c931a44a"),
         ("https://github.com/atom/atom.git",                 "108b23210759a8c5b2f51ac99659be5dc31a7371","a3c320dd707b915da2192427bcceea166edbd6d4"),
         ("https://github.com/facebook/react-native.git",     "1850906e5e557beb2234a1708cfc5fe8e7b4f0bf","764dd511d21aa4dac815c59a7d72497267fde08a"),
         ("https://github.com/webpack/webpack.git",           "babe736cfa1ef7e8014ed32ba4a4ec38049dce14","3e74cb428af04eedac60ae13d2420d2b5bd3bde1")]
        .into_iter()
        .map(|(url, head, base)| (url.to_owned(), head.to_owned(), base.to_owned()))
        .into_csv_with_headers_in_dir(
            vec!["url", "head", "base"], output, 
            format!("selections/original.csv"))
}
