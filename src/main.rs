extern crate pulldown_cmark;
extern crate yansi;
#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::{Read, stdin, stdout};
use std::env;
use std::io::Write;
use std::process::Command;
use std::error::Error;

use pulldown_cmark::{Event, Parser, Tag};
use yansi::Paint;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "issueboss")]
/// Helps in posting issues to multiple sources.
enum Opt {
  #[structopt(name = "trello")]
  Trello(TrelloOpt),
  #[structopt(name = "gitlab")]
  Gitlab(GitlabOpt),
  #[structopt(name = "ast")]
  Ast(AstOpt),
  #[structopt(name = "parse")]
  Parse(ParseOpt),
}

#[derive(StructOpt, PartialEq, Debug)]
struct TrelloOpt {
  #[structopt(short = "l", long = "list", help = "Name of the trello list")]
  list: String,
  #[structopt(short = "b", long = "board", help = "Name of the trello board")]
  board: String,
  #[structopt(name = "FILE")]
  file: String,
}

#[derive(StructOpt, PartialEq, Debug)]
struct GitlabOpt {

}

#[derive(StructOpt, PartialEq, Debug)]
/// Parse markdown and output final representation
struct ParseOpt {
  #[structopt(name = "FILE")]
  file: String,
}

#[derive(StructOpt, PartialEq, Debug)]
/// Parse markdown and output AST
struct AstOpt {
  #[structopt(name = "FILE")]
  file: String,
}

#[derive(Default, Debug, Clone)]
struct Issue {
  title: String,
  description: String,
}

fn cmd_trello(opt: TrelloOpt) -> Result<(), Box<Error>> {
  let mut file = File::open(opt.file)?;
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
        "-b", &opt.board, 
        "-l", &opt.list,
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

fn cmd_gitlab(opt: GitlabOpt) -> Result<(), Box<Error>> {
  Ok(())
}

fn cmd_parse(opt: ParseOpt) -> Result<(), Box<Error>> {
  let mut file = File::open(opt.file)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let p = Parser::new(&contents);

  for node in p {
    println!("{:?}", node);
  }

  Ok(())
}

fn cmd_ast(opt: AstOpt) -> Result<(), Box<Error>> {
  let mut file = File::open(opt.file)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let p = Parser::new(&contents);

  for node in p {
    println!("{:?}", node);
  }

  Ok(())
}

fn run() -> Result<(), Box<Error>> {
  let opt = Opt::from_args();
  //println!("{:?}", matches);
  //return Ok(());

  match opt {
    Opt::Trello(o) => {
      cmd_trello(o)
    }
    Opt::Gitlab(o) => {
      cmd_gitlab(o)
    }
    Opt::Ast(o) => {
      cmd_ast(o)
    }
    Opt::Parse(o) => {
      cmd_parse(o)
    }
  }
}

fn main() {
  match run() {
    Ok(_) => (),
    Err(e) => {
      println!("{}", e);
    }
  }
}
