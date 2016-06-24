extern crate docbase_cli;
extern crate tempfile;
extern crate getopts;

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
use getopts::Options;

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
        }
        Err(..) => {
            println!("Please input number between {} ~ {}", min, max);
            return read_number(prompt, min, max);
        }
    }
}

fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = String::from(input_text.trim());
    return trimmed;
}

fn show_choose(items: Vec<(&str, bool)>) -> usize {
    let mut count = 0;
    for item in items {
        if item.1 {
            println!("{}: {}", count, item.0);
            count += 1;
        } else {
            println!("{}", item.0);
        }
    }
    let idx = read_number("> ", 0, count);
    return idx;
}

fn show_post<F: Fn(Client) -> ()>(client: Client, post: &Post, pager: &str, on_finish: F) {
    let tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
    let mut file: File = OpenOptions::new().write(true).open(tmpfile.path()).unwrap();
    match file.write_all(post.body.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to: {}", why);
        }
        Ok(_) => {
            match tmpfile.path().to_str() {
                Some(pth) => {
                    let mut edit = Command::new(pager);
                    edit.arg(pth);
                    let status = edit.status().unwrap_or_else(|e| {
                        panic!("Failed to open view: {}", e);
                    });
                    if !status.success() {
                        panic!("View failed!");
                    }
                    on_finish(client);
                }
                None => {
                    panic!("Failed to get temporary file path.");
                }
            }
        }
    }
}

fn browse_group(client: Client, team: &str, group: &str, pager: &str) {
    let searchResult: PostSearchResult =
        client.team(team.to_owned()).group(group.to_owned()).send();
    if searchResult.posts.len() < 1 {
        println!("No post found");
    } else {
        show_posts(client,
                   &searchResult,
                   pager,
                   "Change Group",
                   &|client| browse_team(client, team, pager),
                   &|client| search_group_posts(client, team, group, pager))
    }
}


fn search_group_posts(client: Client, team: &str, group: &str, pager: &str) {
    let q = read_string("q> ");
    let searchResult: PostSearchResult =
        client.team(team.to_owned()).group(group.to_owned()).search(&q);
    show_post_search_results(client,
                             &searchResult,
                             pager,
                             &|client| browse_group(client, team, group, pager),
                             &|client| search_group_posts(client, team, group, pager));
}


fn browse_team(client: Client, team: &str, pager: &str) {
    let groups: Vec<Group> = client.team(team.to_owned()).groups();
    if groups.len() < 1 {
        println!("No group found.");
    } else if groups.len() == 1 {
        browse_group(client, team, &groups.first().unwrap().name, pager);
    } else {
        let mut acc: Vec<(&str, bool)> = Vec::new();
        for (i, group) in groups.iter().enumerate() {
            acc.push((&group.name, true));
        }
        acc.push(("", false));
        acc.push(("Change team", true));
        acc.push(("Search Posts", true));
        let idx = show_choose(acc);
        let group_count = groups.len();
        if idx == group_count {
            browse_top(client, pager);
        } else if idx == (group_count + 1) {
            search_team_posts(client, team, pager);
        } else {
            browse_group(client, team, &groups[idx].name, pager);
        }
    }
}

fn show_posts<F1, F2>(client: Client,
                      searchResult: &PostSearchResult,
                      pager: &str,
                      change_msg: &str,
                      on_change: &F1,
                      on_search: &F2)
    where F1: Fn(Client) -> (),
          F2: Fn(Client) -> ()
{
    if searchResult.posts.len() < 1 {
        println!("No post found");
    } else {
        for (i, post) in searchResult.posts.iter().enumerate() {
            println!("{}: {}", i, post.title);
        }
        println!("");
        if searchResult.meta.previous_page.is_some() && searchResult.meta.next_page.is_some() {
            println!("{}: Show PrevPage", searchResult.posts.len());
            println!("{}: Show NextPage", searchResult.posts.len() + 1);
            println!("{}: Search Post", searchResult.posts.len() + 2);
            println!("{}: {}", searchResult.posts.len() + 3, change_msg);
            let idx = read_number("> ", 0, searchResult.posts.len() + 4);
            if idx == searchResult.posts.len() {
                let prevPageResult = client.load_prev_post_search_result(searchResult);
                show_posts(client, &prevPageResult, pager, change_msg, on_change, on_search);
            } else if idx == searchResult.posts.len() + 1 {
                let nextPageResult = client.load_next_post_search_result(searchResult);
                show_posts(client, &nextPageResult, pager, change_msg, on_change, on_search);
            } else if idx == searchResult.posts.len() + 2 {
                on_search(client);
            } else if idx == searchResult.posts.len() + 3 {
                on_change(client);
            } else {
                show_post(client, &searchResult.posts[idx], pager,
                          |client| {
                              show_posts(client, searchResult, pager, change_msg, on_change, on_search)
                          });
            }
        } else if searchResult.meta.previous_page.is_some() {
            println!("{}: Show PrevPage", searchResult.posts.len());
            println!("{}: Search Post", searchResult.posts.len() + 1);
            println!("{}: {}", searchResult.posts.len() + 2, change_msg);
            let idx = read_number("> ", 0, searchResult.posts.len() + 3);
            if idx == searchResult.posts.len() {
                let prevPageResult = client.load_prev_post_search_result(searchResult);
                show_posts(client, &prevPageResult, pager, change_msg, on_change, on_search);
            } else if idx == searchResult.posts.len() + 1 {
                on_search(client);
            } else if idx == searchResult.posts.len() + 2 {
                on_change(client);
            } else {
                show_post(client, &searchResult.posts[idx],
                          pager,
                          |client| {
                              show_posts(client, searchResult, pager, change_msg, on_change, on_search)
                          });
            }
        } else if searchResult.meta.next_page.is_some() {
            println!("{}: Show NextPage", searchResult.posts.len());
            println!("{}: Search Post", searchResult.posts.len() + 1);
            println!("{}: {}", searchResult.posts.len() + 2, change_msg);
            let idx = read_number("> ", 0, searchResult.posts.len() + 3);
            if idx == searchResult.posts.len() {
                let nextPageResult = client.load_next_post_search_result(searchResult);
                show_posts(client, &nextPageResult, pager, change_msg, on_change, on_search);
            } else if idx == searchResult.posts.len() + 1 {
                on_search(client);
            } else if idx == searchResult.posts.len() + 2 {
                on_change(client);
            } else {
                show_post(client, &searchResult.posts[idx],
                          pager,
                          |client| {
                              show_posts(client, searchResult, pager, change_msg, on_change, on_search)
                          });
            }
        } else {
            println!("{}: Search Post", searchResult.posts.len());
            println!("{}: {}", searchResult.posts.len() + 1, change_msg);
            let idx = read_number("> ", 0, searchResult.posts.len() + 2);
            if idx == searchResult.posts.len() {
                on_search(client);
            } else if idx == searchResult.posts.len() + 1 {
                on_change(client);
            } else {
                show_post(client, &searchResult.posts[idx],
                          pager,
                          |client| {
                              show_posts(client, searchResult, pager, change_msg, on_change, on_search)
                          });
            }
        }
    }
}

fn show_post_search_results<F1, F2>(client: Client,
                                    searchResult: &PostSearchResult,
                                    pager: &str,
                                    on_quit_search: &F1,
                                    on_change_word: &F2)
    where F1: Fn(Client) -> (),
          F2: Fn(Client) -> ()
{
    if searchResult.posts.len() < 1 {
        println!("No post found");
        println!("0: Quit Search");
        println!("1: Change word");
        let idx = read_number("> ", 0, 2);
        match idx {
            0 => on_quit_search(client),
            1 => on_change_word(client),
            _ => panic!("Illigal"),
        }
    } else {
        for (i, post) in searchResult.posts.iter().enumerate() {
            println!("{}: {}", i, post.title);
        }
        println!("");
        if searchResult.meta.previous_page.is_some() && searchResult.meta.next_page.is_some() {
            println!("{}: Show PrevPage", searchResult.posts.len());
            println!("{}: Show NextPage", searchResult.posts.len() + 1);
            println!("{}: Quit Search", searchResult.posts.len() + 2);
            println!("{}: Change word", searchResult.posts.len() + 3);
            let idx = read_number("> ", 0, searchResult.posts.len() + 4);
            if idx == searchResult.posts.len() {
                let prevPageResult = client.load_prev_post_search_result(searchResult);
                show_post_search_results(client, &prevPageResult, pager, on_quit_search, on_change_word);
            } else if idx == searchResult.posts.len() + 1 {
                let nextPageResult = client.load_next_post_search_result(searchResult);
                show_post_search_results(client, &nextPageResult, pager, on_quit_search, on_change_word);
            } else if idx == searchResult.posts.len() + 2 {
                on_quit_search(client);
            } else if idx == searchResult.posts.len() + 3 {
                on_change_word(client);
            } else {
                show_post(client, &searchResult.posts[idx],
                          pager,
                          |client| {
                              show_post_search_results(client, searchResult, pager, on_quit_search, on_change_word)
                          });
            }
        } else if searchResult.meta.previous_page.is_some() {
            println!("{}: Show PrevPage", searchResult.posts.len());
            println!("{}: Quit Search", searchResult.posts.len() + 1);
            println!("{}: Change word", searchResult.posts.len() + 2);
            let idx = read_number("> ", 0, searchResult.posts.len() + 3);
            if idx == searchResult.posts.len() {
                let prevPageResult = client.load_prev_post_search_result(searchResult);
                show_post_search_results(client, &prevPageResult, pager, on_quit_search, on_change_word);
            } else if idx == searchResult.posts.len() + 1 {
                on_quit_search(client);
            } else if idx == searchResult.posts.len() + 2 {
                on_change_word(client);
            } else {
                show_post(client, &searchResult.posts[idx],
                          pager,
                          |client| {
                              show_post_search_results(client, searchResult, pager, on_quit_search, on_change_word)
                          });
            }
        } else if searchResult.meta.next_page.is_some() {
            println!("{}: Show NextPage", searchResult.posts.len());
            println!("{}: Quit Search", searchResult.posts.len() + 1);
            println!("{}: Change word", searchResult.posts.len() + 2);
            let idx = read_number("> ", 0, searchResult.posts.len() + 3);
            if idx == searchResult.posts.len() {
                let nextPageResult = client.load_next_post_search_result(searchResult);
                show_post_search_results(client, &nextPageResult, pager, on_quit_search, on_change_word);
            } else if idx == searchResult.posts.len() + 1 {
                on_quit_search(client);
            } else if idx == searchResult.posts.len() + 2 {
                on_change_word(client);
            } else {
                show_post(client, &searchResult.posts[idx],
                          pager,
                          |client| {
                              show_post_search_results(client, searchResult, pager, on_quit_search, on_change_word)
                });
            }
        } else {
            println!("{}: Quit Search", searchResult.posts.len());
            println!("{}: Change word", searchResult.posts.len() + 1);
            let idx = read_number("> ", 0, searchResult.posts.len() + 2);
            if idx == searchResult.posts.len() {
                on_quit_search(client);
            } else if idx == searchResult.posts.len() + 1 {
                on_change_word(client);
            } else {
                show_post(client, &searchResult.posts[idx],
                          pager,
                          |client| {
                              show_post_search_results(client, searchResult, pager, on_quit_search, on_change_word)
                          });
            }
        }
    }
}

fn search_team_posts(client: Client, team: &str, pager: &str) {
    let q = read_string("q> ");
    let searchResult: PostSearchResult = client.team(team.to_owned()).search(&q);
    show_post_search_results(client,
                             &searchResult,
                             pager,
                             &|client| browse_team(client, team, pager),
                             &|client| search_team_posts(client, team, pager));
}

fn browse_top(client: Client, pager: &str) {
    let teams: Vec<Team> = client.teams();
    if teams.len() < 1 {
        println!("No team found");
    } else if teams.len() == 1 {
        browse_team(client, &teams.first().unwrap().name, pager);
    } else {
        for (i, team) in teams.iter().enumerate() {
            println!("{}: {}", i, team.name);
        }
        let idx = read_number("> ", 0, teams.len());
        browse_team(client, &teams[idx].name, pager);
    }
}

fn main() {
    let key = "DOCBASE_TOKEN";
    let api_token = match env::var(key) {
        Ok(v) => v,
        Err(e) => panic!("environment variable `DOCBASE_TOKEN` not found"),
    };
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("t", "team-domain", "set team domain name", "TEAM_DOMAIN");
    opts.optopt("g", "group", "set group name", "GROUP_NAME");
    opts.optopt("p", "pager", "set pager program", "PAGER");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let tdomain = matches.opt_str("t");
    let gname = matches.opt_str("g");
    let pager = match matches.opt_str("p") {
        Some(p) => {p},
        None => String::from("less")
    };
    let client = Client { api_key: api_token.to_owned() };
    if tdomain.is_some() && gname.is_some() {
        browse_group(client, &tdomain.unwrap(), &gname.unwrap(), &pager);
    } else if tdomain.is_some() {
        browse_team(client, &tdomain.unwrap(), &pager);
    } else {
        browse_top(client, &pager);
    }
}
