extern crate docbase_cli;
extern crate tempfile;

use docbase_cli::client::Client;
use docbase_cli::models::group::Group;
use docbase_cli::models::team::Team;
use docbase_cli::models::post_search_result::PostSearchResult;
use docbase_cli::models::post::Post;
use std::env;
use std::io;
use std::io::Write;
use std::process::Command;
use std::fs::File;
use std::fs::OpenOptions;

fn show_post(post: &Post) {
    let tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
    let mut file: File = OpenOptions::new().write(true).open(tmpfile.path()).unwrap();
    match file.write_all(post.body.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to: {}", why);
        },
        Ok(_) => {
            match tmpfile.path().to_str() {
                Some(pth) => {
                    let mut edit = Command::new("less");
                    edit.arg(pth);
                    let status = edit.status().unwrap_or_else(|e| {
                        panic!("Failed to open view: {}", e);
                    });
                    if !status.success() {
                        panic!("View failed!");
                    }
                },
                None => {
                    panic!("Failed to get temporary file path.");
                }
            }
        }
    }
}

fn browse_group(client: Client, team: &Team, group: &Group) {
    let searchResult: PostSearchResult = client.group_posts(&team.name, &group.name);
    if searchResult.posts.len() < 1 {
        println!("No post found");
    } else if(searchResult.posts.len() == 1) {
        show_post(&searchResult.posts.first().unwrap());
    } else {
        for (i, post) in searchResult.posts.iter().enumerate() {
            println!("{}: {}", i, post.title);
        }
        let idx = read_number("> ", 0, searchResult.posts.len());
        show_post(&searchResult.posts[idx]);
    }
}

fn read_number(prompt: &str, min: usize, max: usize) -> usize {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => {
            if min <= i && i < max {
                return i;
            } else {
                println!("Please input number between {} ~ {}", min, max);
                return read_number(prompt, min, max);
            }
        },
        Err(..) => {
            println!("Please input number between {} ~ {}", min, max);
            return read_number(prompt, min, max);
        }
    }
}

fn browse_team(client: Client, team: &Team) {
    let groups: Vec<Group> = client.groups(&team.name);
    if (groups.len() < 1) {
        println!("No group found.");
    } else if(groups.len() == 1) {
        browse_group(client, team, groups.first().unwrap());
    } else {
        for (i,group) in groups.iter().enumerate() {
            println!("{}: {}", i, group.name);
        }
        let idx = read_number("> ", 0, groups.len());
        browse_group(client, team, &groups[idx]);
    }
}

fn main() {
    let key = "DOCBASE_TOKEN";
    let api_token = match env::var(key) {
        Ok(v) => v,
        Err(e) => panic!("environment variable `DOCBASE_TOKEN` not found")
    };
    let client = Client { api_key: api_token.to_owned() };
    let teams: Vec<Team> = client.teams();
    if (teams.len() < 1) {
        println!("No team found");
    } else if(teams.len() == 1) {
        browse_team(client, teams.first().unwrap());
    } else {
        for (i, team) in teams.iter().enumerate() {
            println!("{}: {}", i, team.name);
        }
        let idx = read_number("> ", 0, teams.len());
        browse_team(client, &teams[idx]);
    }
}
