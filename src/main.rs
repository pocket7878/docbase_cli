extern crate docbase_cli;

use docbase_cli::client::Client;
use docbase_cli::models::group::Group;
use docbase_cli::models::team::Team;
use docbase_cli::models::post_search_result::PostSearchResult;

fn main() {
    let api_token = "xxxxxxxxxxxxxxxx";
    let client = Client { api_key: api_token.to_owned() };
    let teams: Vec<Team> = client.teams();
    for team in teams {
        println!("{}", team.domain);
    }
    let groups: Vec<Group> = client.groups("azit");
    for group in groups {
        println!("{}", group.name);
    }
    let searchResult: PostSearchResult = client.get_posts("azit");
    for post in searchResult.posts {
        println!("{}", post.title);
    }
}
