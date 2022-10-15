#![feature(fs_try_exists)]
use std::{fs};
use chrono::{prelude::{Local, SecondsFormat}};

fn main() {
    let dir = match fs::try_exists("./content") {
        Ok(true) => "./content/",
        Ok(false) => "./",
        Err(e) => panic!("{}", e)
    };
    let local = Local::now();
    let time = local.to_rfc3339_opts(SecondsFormat::Secs, false);
    let file_name = local.format("%Y-%m-%d").to_string() + "-Daily-Challenge";
    let title = local.format("%Y-%m-%d").to_string() + " Daily Challenge";
    let month = local.format("%B").to_string();
    let day_binding = local.format("%e").to_string();
    let day = day_binding.trim();
    let problem_description = "";
    let problem_title = "";
    let url = "";
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

Today I have done leetcode's [{month} LeetCoding Challenge]({url}) with `cpp`.

<!-- more -->

# {month} LeetCoding Challenge {day}

## Description

**{problem_title}**

{problem_description}

## Solution

```

```
", time = time, file_name = file_name, title = title, problem_title = problem_title, problem_description = problem_description, url = url);

    println!("{}", document);
}
