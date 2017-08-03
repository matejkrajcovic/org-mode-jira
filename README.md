# org-mode-jira

[![Build Status](https://travis-ci.org/matejkrajcovic/org-mode-jira.svg?branch=master)](https://travis-ci.org/matejkrajcovic/org-mode-jira)

`org-mode-jira` allows you to track time spent on Jira issues in an Org mode file and then simply push them to Jira.

## Usage
Create file `jira.org` with following content:
```org
#+JIRA_URL: https://my_project.atlassian.net/
#+USERNAME: me

* ABC-1 (Take a nap)
    CLOCK: [2017-07-28 Fri 15:00]--[2017-07-28 Fri 17:30] =>  2:30
* ABC-2 (Write README)
    CLOCK: [2017-07-28 Fri 17:30]--[2017-07-28 Fri 19:50] =>  2:20
```

Executing
```sh
$ org-mode-jira jira.org
```
will ask you for password and push two new worklogs to Jira. Texts in parentheses will be used as worklog descriptions.

## Installation
1. [Install Rust](https://www.rust-lang.org/en-US/install.html)
2. Clone the source:
```sh
$ git clone https://github.com/matejkrajcovic/org-mode-jira.git
$ cd org-mode-jira
```
3. Install:
```sh
$ cargo install
```

## License
Licensed under [GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.en.html).
