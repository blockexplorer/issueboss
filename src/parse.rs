use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml_edit::{Document as TDocument, Item};

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

pub fn parse_file<P: AsRef<Path>>(file_name: P) -> Result<Document, Box<Error>> {
  let mut file = File::open(file_name)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let toml = contents.parse::<TDocument>()?;

  let mut doc = Document::default();
  for entry in toml.iter() {
    match entry {
      (key, &Item::Value(ref value)) => {
        let mut h = doc.metadata.get_or_insert(HashMap::new());
        h.insert(key.to_string(), value.as_str().unwrap().to_string());
      }
      (key, &Item::Table(ref table)) => {
        let mut issue = Issue::default();
        issue.title = key.to_string();
        for tentry in table.iter() {
          if let (key, &Item::Value(ref value)) = tentry {
            if key == "description" {
              issue.description = value.as_str().unwrap().to_string();
            } else {
              let mut h = issue.metadata.get_or_insert(HashMap::new());
              h.insert(key.to_string(), value.as_str().unwrap().to_string());
            }
          }
        }
        doc.issues.push(issue);
      }
      _ => (),
    };
  }

  Ok(doc)
}
