use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;

use pulldown_cmark::{Event, Parser, Tag};

pub type Metadata = HashMap<String, String>;

#[derive(Default, Debug, Clone)]
pub struct Issue {
  pub metadata: Option<Metadata>,
  pub title: String,
  pub description: String,
}

#[derive(Default, Debug, Clone)]
pub struct Document {
  pub metadata: Option<Metadata>,
  pub issues: Vec<Issue>,
}

type StateFn<'a> = fn(&mut Machine<'a>);

pub struct Machine<'a> {
  doc: Option<Document>,
  done: bool,
  err: Option<Box<Error>>,

  parser: Peekable<Parser<'a>>,
  statefn: StateFn<'a>,
}

impl<'a> Machine<'a> {
  pub fn new(parser: Parser<'a>) -> Machine<'a> {
    Machine {
      doc: None,
      parser: parser.peekable(),
      done: false,
      err: None,
      statefn: Machine::start,
    }
  }

  fn error<S: Into<String>>(&mut self, e: S) {
    self.done = true;
    self.err = Some(From::from(e.into()));
  }

  fn start(&mut self) {
    self.doc = Some(Document::default());

    match self.parser.peek() {
      Some(&Event::End(Tag::Rule)) => {
        self.statefn = Self::metadata;
      }
      Some(&Event::Start(Tag::Header(_))) => {}
      _ => {
        self.parser.next();
      }
    };
  }

  // fn start_rule(&mut self) {
  //   self.statefn = Self::end_rule;
  // }

  fn metadata(&mut self) {
    self.parser.next();
    self.statefn = Self::end;
  }

  fn end(&mut self) {
    self.error("some error");
  }
}

impl<'a> Iterator for Machine<'a> {
  type Item = Result<(), Box<Error>>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      if self.err.is_some() {
        return Some(Err(self.err.take().unwrap()));
      }
      return None;
    }

    (self.statefn)(self);
    Some(Ok(()))
  }
}

pub fn parse_file<P: AsRef<Path>>(file_name: P) -> Result<Document, Box<Error>> {
  markdown_tokens(file_name.as_ref())?;
  let mut file = File::open(file_name)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let p = Parser::new(&contents);
  let mut m = Machine::new(p);

  for i in m.by_ref() {
    i?;
  }

  let doc = m.doc.take();
  Ok(doc.unwrap())
}

// pub fn parse_file<P: AsRef<Path>>(file_name: P) -> Result<Document, Box<Error>> {
//   let mut file = File::open(file_name)?;
//   let mut contents = String::new();
//   file.read_to_string(&mut contents)?;

//   let p = Parser::new(&contents);

//   let mut metadata = Metadata::new();
//   let mut issues = vec![];
//   let mut issue = Issue::default();
//   let mut text: String = String::new();
//   for node in p {
//     match node {
//       Event::Start(Tag::Rule) => (),
//       Event::End(Tag::Rule) => {}
//       Event::Start(Tag::Header(_)) => {
//         issue = Issue::default();
//       }
//       Event::End(Tag::Header(_)) => {
//         issue.title = text;
//         text = String::new();
//       }
//       Event::Text(t) => {
//         text.push_str(&t);
//       }
//       Event::SoftBreak => {
//         text.push('\n');
//       }
//       Event::Start(Tag::Paragraph) => {}
//       Event::End(Tag::Paragraph) => {
//         issue.description = text;
//         text = String::new();
//         issues.push(issue.clone());
//       }
//       _ => (),
//     }
//   }

//   Ok(Document { issues: issues })
// }

#[allow(dead_code)]
pub fn markdown_tokens<P: AsRef<Path>>(file_name: P) -> Result<(), Box<Error>> {
  let mut file = File::open(file_name)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let p = Parser::new(&contents);

  for node in p {
    println!("{:?}", node);
  }

  Ok(())
}
