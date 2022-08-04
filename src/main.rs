use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Post {
    text: String,
}

#[derive(Deserialize, Debug)]
struct LastTweets {
    data: Vec<Post>,
}

fn main() {
    let tweeter_id = match env::args().nth(1) {
        Some(id) => id,
        None => panic!("No tweeter id has been provided."),
    };

    let bearer = match env::args().nth(2) {
        Some(path) => path,
        None => panic!("Twitter bearer token has not been provided."),
    };

    let output_filepath = match env::args().nth(3) {
        Some(path) => path,
        None => panic!("Output filepath has not been provided."),
    };

    let tweeter_timeline_url = format!("https://api.twitter.com/2/users/{}/tweets", tweeter_id);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(tweeter_timeline_url)
        .header(AUTHORIZATION, format!("Bearer {}", bearer))
        .send()
        .unwrap();

    let str: String = resp.text().unwrap();

    let tweets: LastTweets = serde_json::from_str(str.as_str()).unwrap();

    println!("{:?}", tweets);
}
