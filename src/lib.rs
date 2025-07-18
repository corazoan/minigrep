use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        for line in search_sensitive(&config.query, &contents, config.ignore_case) {
            println!("{line}");
        }
    } else {
        for line in search_sensitive(&config.query, &contents, config.ignore_case) {
            println!("{line}");
        }
    }

    // println!("With text:\n{contents}");
    Ok(contents)
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough argument received");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        // let key = env::var("PATH").expect("HELO");
        // let vec:Vec<&str> = key.split(":").collect();
        // for i in vec {
        //     println!("{}", i);
        // }
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn search_sensitive<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for x in content.lines() {
        if ignore_case {
            if x.to_lowercase().contains(&query.to_lowercase()) {
                result.push(x.trim());
            }
        } else {
            if x.trim().contains(query) {
                result.push(x.trim());
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::search_sensitive;

    #[test]
    fn one_result() {
        let query = "Rust";
        let content = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        let ignore_case = true;
        assert_eq!(vec!["Rust:"], search_sensitive(query, content, ignore_case));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_sensitive(query, contents, true)
        );
    }
}
