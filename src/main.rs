#[macro_use]
extern crate structopt;
extern crate toml_edit;
extern crate yansi;

mod parse;

use std::io::{stdin, stdout};
use std::io::Write;
use std::process::Command;
use std::error::Error;

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
struct GitlabOpt {}

#[derive(StructOpt, PartialEq, Debug)]
/// Parse markdown and output final representation
struct ParseOpt {
  #[structopt(name = "FILE")]
  file: String,
}

fn cmd_trello(opt: TrelloOpt) -> Result<(), Box<Error>> {
  let doc = parse::parse_file(opt.file)?;

  for issue in &doc.issues {
    println!(
      "{}: {}",
      Paint::white("Title").bold(),
      Paint::yellow(&issue.title)
    );
    println!(
      "{}:\n{}",
      Paint::white("Description").bold(),
      Paint::green(&issue.description)
    );
    println!();
  }

  print!("Continue? (y/N) ");
  stdout().flush()?;
  let mut answer = String::new();
  stdin().read_line(&mut answer)?;

  if answer.trim().to_lowercase() == "y" {
    for issue in doc.issues {
      let output = Command::new("trello")
        .args(&[
          "add-card",
          "-b",
          &opt.board,
          "-l",
          &opt.list,
          "-p",
          "bottom",
          &issue.title,
          &issue.description,
        ])
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

fn cmd_gitlab(_opt: GitlabOpt) -> Result<(), Box<Error>> {
  Ok(())
}

fn cmd_parse(opt: ParseOpt) -> Result<(), Box<Error>> {
  let document = parse::parse_file(opt.file)?;
  println!("{:#?}", document);

  Ok(())
}

fn run() -> Result<(), Box<Error>> {
  let opt = Opt::from_args();
  //println!("{:?}", matches);
  //return Ok(());

  match opt {
    Opt::Trello(o) => cmd_trello(o),
    Opt::Gitlab(o) => cmd_gitlab(o),
    Opt::Parse(o) => cmd_parse(o),
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
