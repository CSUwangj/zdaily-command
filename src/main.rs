#![feature(fs_try_exists)]
use std::{fs, collections::HashMap};
use chrono::{prelude::{Local, SecondsFormat}};
use dotenv::dotenv;
use log::info;
use reqwest::Client;
use structopt::StructOpt;

mod opts;
mod model;

use crate::model::{DailyData, QuestionData};

fn new_client() -> Client {
    match std::env::var("HTTPS_PROXY") {
        Ok(proxy) => reqwest::Client::builder()
          .proxy(reqwest::Proxy::all(proxy).expect("Pass Proxy Failed"))
          .build()
          .expect("Client Build Failed."),
        Err(_) => reqwest::Client::new()
    }
}

async fn fetch_daily() -> DailyData {
    let mut map = HashMap::new();
    map.insert("query", "query questionOfToday {
  activeDailyCodingChallengeQuestion {
    link
    question {
      title
      titleSlug
    }
  }
}");

    
    let client = new_client();
    let res = client.post("https://leetcode.com/graphql/")
        .json(&map)
        .send()
        .await
        .expect("Fetch Failed.");
    return res
        .json::<DailyData>()
        .await
        .expect("Wrong Response for daily metadata.");
}

async fn fetch_question(title_slug: String) -> QuestionData {
    let mut map = HashMap::new();
    map.insert("query", format!("query questionData {{
  question(titleSlug: \"{}\") {{
    content
  }}
}}", title_slug));

    let client = new_client();
    let res = client.post("https://leetcode.com/graphql/")
        .json(&map)
        .send()
        .await
        .expect("Fetch Failed.");
    return res
        .json::<QuestionData>()
        .await
        .expect("Wrong Response for daily metadata.");
}
  

#[tokio::main]
async fn main() {
    dotenv().ok();
    let opt = opts::Opt::from_args();
    std::env::set_var("RUST_LOG", "trace");
    match opt.log_level {
        0 => std::env::set_var("RUST_LOG", "error"),
        1 => std::env::set_var("RUST_LOG", "warn"),
        2 => std::env::set_var("RUST_LOG", "info"),
        3 => std::env::set_var("RUST_LOG", "debug"),
        _ => std::env::set_var("RUST_LOG", "trace"),
    }
    pretty_env_logger::init();

    info!("{:?}", opt);

    let dir = match fs::try_exists(&opt.content_dir) {
        Ok(true) => opt.content_dir,
        Ok(false) => "./".into(),
        Err(e) => panic!("{}", e)
    };
    let local = Local::now();
    let time = local.to_rfc3339_opts(SecondsFormat::Secs, false);
    let file_name = local.format("%Y-%m-%d").to_string() + "-Daily-Challenge";
    let title = local.format("%Y-%m-%d").to_string() + " Daily Challenge";
    let month = local.format("%B").to_string();
    let day_binding = local.format("%e").to_string();
    let day = day_binding.trim();

    let daily = fetch_daily().await;
    let content = fetch_question(daily.data.active_daily_question.question.title_slug).await;

    let problem_description = content.data.question.content;
    let problem_title = daily.data.active_daily_question.question.title;
    let url = daily.data.active_daily_question.link;
    let document = format!("+++
updated = {time}
title = \"{title}\"
path = \"{file_name}\"
date = {time}

[taxonomies]
tags = [\"Algorithm\"]
categories = [\"DailyChallenge\"]
archives = [\"archive\"]
+++

Today I have done leetcode's [{month} LeetCoding Challenge](https://leetcode.com{url}) with `cpp`.

<!-- more -->

# {month} LeetCoding Challenge {day}

## Description

**{problem_title}**

{problem_description}

## Solution

``` cpp

```
", time = time, file_name = file_name, title = title, problem_title = problem_title, problem_description = problem_description, url = url);

    info!("{}", document);

    let folder_name = dir.join(format!("{}-Daily-Challenge", local.format("%Y-%m-%d")));
    let file_name = folder_name.join("index.md");

    fs::create_dir_all(folder_name).expect("Folder already exist.");
    fs::write(file_name, document).expect("Write template failed.");
}
