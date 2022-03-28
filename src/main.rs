use reqwest;
use serde::{Deserialize, Serialize};
use std::io;
use std::process::exit;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    awards: i16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Posts {
    posts: Vec<Post>,
}

fn new_post(val: &serde_json::Value) -> Post {
    return Post {
        title: val["data"]["title"].to_string(),
        awards: val["data"]["total_awards_received"]
            .to_string()
            .parse()
            .unwrap(),
    };
}

fn get_posts(val: &serde_json::Value) -> Vec<Post> {
    let length: usize = val["data"]["children"].as_array().unwrap().len();
    let mut vec: Vec<Post> = Vec::new();

    for n in 0..length - 1 {
        vec.push(new_post(&val["data"]["children"][n]));
    }
    return vec;
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let response = client
        .get("https://www.reddit.com/r/WritingPrompts/top/.json?t=day")
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let posts = get_posts(&response);

    println!("{}", posts[2].title);
    // println!("{}", get_posts(&response));

    println!("How do you want to sort the prompts?");

    println!(
        "
        1. Hot
        2. Daily
        3. Weekly
        4. Monthly
        "
    );

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    if guess < 1 || guess > 4 {
        println!("you can only use 1-4");
        exit(1);
    }

    // loop to show
    let mut chosen = false;
    let mut post_num;
    let mut input;

    post_num = 0;
    while !chosen {
        print!("{}[2J", 27 as char); // clear screen
        println!("{}/{} {}\n", post_num+1, posts.len(), posts[post_num].title);
        println!("want to read this post? Next, Back, Select [n/b/s]");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line ");

        let input: char = input.trim().parse().expect("please tupe one char");

        if input == 'n' {
            post_num = post_num + 1;
            if post_num == posts.len() {
                post_num = 0;
            }
            println!("post num {}", post_num);
        } else if input == 'b' {
            if post_num == 0 as usize {
                post_num = posts.len();
            }
            post_num = post_num - 1;

        }
    }
}
