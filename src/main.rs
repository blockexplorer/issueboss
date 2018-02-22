extern crate pulldown_cmark;
extern crate yansi;

use std::fs::File;
use std::io::{Read, stdin, stdout};
use std::env;
use std::io::Write;
use std::process::Command;
use std::error::Error;

use pulldown_cmark::{Event, Parser, Tag};
use yansi::Paint;

#[derive(Default, Debug, Clone)]
struct Issue {
  title: String,
  description: String,
}

fn usage() {
  println!("issuebose <markdown file> <board> <list>");
  println!("Make sure to set up trello-cli: https://github.com/mheap/trello-cli");
}

fn run() -> Result<(), Box<Error>> {
  let markdown_file = env::args().nth(1).ok_or_else(|| "missing markdown file")?;
  let board = env::args().nth(2).ok_or_else(|| "missing board")?;
  let list = env::args().nth(3).ok_or_else(|| "missing list")?;

  let mut file = File::open(markdown_file)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let p = Parser::new(&contents);

  let mut issues = vec![];
  let mut issue = Issue::default();
  let mut text: String = String::new();
  for node in p {
    match node {
      Event::Start(Tag::Header(_)) => {
        issue = Issue::default();
      }
      Event::End(Tag::Header(_)) => {
        issue.title = text;
        text = String::new();
      }
      Event::Text(t) => {
        text.push_str(&t);
      }
      Event::SoftBreak => {
        text.push('\n');
      }
      Event::Start(Tag::Paragraph) => {}
      Event::End(Tag::Paragraph) => {
        issue.description = text;
        text = String::new();
        issues.push(issue.clone());
      }
      _ => (),
    }
  }

  for issue in &issues {
    println!("{}: {}", Paint::white("Title").bold(), Paint::yellow(&issue.title));
    println!("{}:\n{}", Paint::white("Description").bold(), Paint::green(&issue.description));
    println!();
  }

  print!("Continue? (y/N) ");
  stdout().flush()?;
  let mut answer = String::new();
  stdin().read_line(&mut answer)?;

  if answer.trim().to_lowercase() == "y" {
    for issue in issues {
      let output = Command::new("trello")
      .args(&[
        "add-card", 
        "-b", &board, 
        "-l", &list,
        "-p", "bottom",
        &issue.title,
        &issue.description])
      .output()
      .expect("failed to execute process");

      if output.stderr.len() > 0 {
        println!("{}: {}", Paint::red("error"), issue.title);
        println!("{}", String::from_utf8_lossy(&output.stderr));
      }
    }
  }

  Ok(())
}

fn main() {
  match run() {
    Ok(_) => (),
    Err(e) => {
      println!("{}", e);
      usage();
    }
  }
}
