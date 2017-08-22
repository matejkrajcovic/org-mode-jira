extern crate regex;
extern crate chrono;

use errors::*;
use self::regex::Regex;
use self::chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Worklog {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

#[derive(Debug)]
pub struct Issue {
    id: String,
    description: String,
    worklogs: Vec<Worklog>,
}

#[derive(Debug)]
pub struct ParsedData {
    jira_url: String,
    username: String,
    issues: Vec<Issue>,
}

pub fn parse_file(path: &str) -> Result<ParsedData> {
    let file = File::open(path)
        .chain_err(|| "unable to open file")?;
    let mut file = BufReader::new(file);

    let mut first_line = String::new();
    file.read_line(&mut first_line)
        .chain_err(|| "unable to read first line")?;
    let jira_url = parse_keyword("JIRA_URL", &first_line)
        .chain_err(|| "unable to parse JIRA_URL")?;

    let mut second_line = String::new();
    file.read_line(&mut second_line)
        .chain_err(|| "unable to read second line")?;
    let username = parse_keyword("USERNAME", &second_line)
        .chain_err(|| "unable to parse USERNAME")?;

    let mut issues: Vec<Issue> = Vec::new();
    let mut worklogs: Vec<Worklog> = Vec::new();
    let mut last_id = String::from("");
    let mut last_description = String::from("");
    for line in file.lines() {
        let line = line.chain_err(|| "unable to read line")?;
        match parse_line(&line).chain_err(|| "unable to parse line")? {
            LineType::Issue{ id, description } => {
                if !worklogs.is_empty() {
                    issues.push(Issue { id: last_id, description: last_description, worklogs });
                    worklogs = Vec::new();
                }
                last_id = id;
                last_description = description;
            },
            LineType::Worklog{ start, end } => {
                worklogs.push(Worklog { start, end });
            },
            LineType::Nothing => {},
        }
    }
    if !worklogs.is_empty() {
        issues.push(Issue { id: last_id, description: last_description, worklogs });
    }

    Ok(ParsedData {
        jira_url,
        username,
        issues,
    })
}

fn parse_keyword(key: &str, line: &str) -> Result<String> {
    let re = Regex::new(&format!(r"#\+{}: (.*)", key)).unwrap();
    let captures = re.captures(line);
    match captures {
        Some(captures) => {
            match captures.get(1) {
                Some(capture) => {
                    Ok(String::from(capture.as_str()))
                }
                None => {
                    bail!("unable to parse keyword 1")
                }
            }
        }
        None => {
            bail!("unable to parse keyword 2")
        }
    }
}

#[test]
fn test_parse_keyword() {
    assert_eq!(parse_keyword("JIRA_URL", "#+JIRA_URL: https://my_project.atlassian.net/").unwrap(), String::from("https://my_project.atlassian.net/"));
    assert_eq!(parse_keyword("USERNAME", "#+USERNAME: username").unwrap(), String::from("username"));
    assert!(parse_keyword("USERNAME", "#+JIRA_URL: https://my_project.atlassian.net/").is_err());
    assert!(parse_keyword("USERNAME", "#+USERNAME:").is_err());
}

#[derive(Debug, Clone, PartialEq)]
enum LineType {
    Nothing,
    Issue {
        id: String,
        description: String,
    },
    Worklog {
        start: DateTime<Local>,
        end: DateTime<Local>,
    },
}

fn parse_line(line: &str) -> Result<LineType> {
    let re_issue = Regex::new(r"\* (\w*-\d*) \((.*)\)").unwrap();
    let re_worklog = Regex::new(r"CLOCK: \[(.*)\]--\[(.*)\] =>").unwrap();

    if let Some(captures) = re_issue.captures(line) {
        match (captures.get(1), captures.get(2)) {
            (Some(id), Some(description)) => {
                let id = String::from(id.as_str());
                let description = String::from(description.as_str());
                Ok(LineType::Issue { id, description })
            }
            _ => {
                bail!("error parsing line")
            }
        }
    } else if let Some(captures) = re_worklog.captures(line) {
        match (captures.get(1), captures.get(2)) {
            (Some(start_str), Some(end_str)) => {
                match (Local.datetime_from_str(start_str.as_str(), "%Y-%m-%d %a %H:%M"),
                       Local.datetime_from_str(end_str.as_str(), "%Y-%m-%d %a %H:%M")) {
                    (Ok(start), Ok(end)) => {
                        Ok(LineType::Worklog { start, end })
                    }
                    _ => {
                        bail!("error parsing line")
                    }
                }
            }
            _ => {
                bail!("error parsing line")
            }
        }
    } else {
        Ok(LineType::Nothing)
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("").unwrap(), LineType::Nothing);
    assert_eq!(parse_line("* ABC-1 (Take a nap)").unwrap(), LineType::Issue {
        id: String::from("ABC-1"),
        description: String::from("Take a nap"),
    });
    assert_eq!(parse_line("CLOCK: [2017-07-28 Fri 15:00]--[2017-07-28 Fri 17:30] =>  2:30").unwrap(), LineType::Worklog {
        start: Local.ymd(2017, 7, 28).and_hms(15, 0, 0),
        end: Local.ymd(2017, 7, 28).and_hms(17, 30, 0),
    });
    assert_eq!(parse_line("* ABC-1").unwrap(), LineType::Nothing);
    assert_eq!(parse_line("CLOCK: [2017-07-28 Fri 15:00]").unwrap(), LineType::Nothing);
}
