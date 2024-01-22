use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut cmd_args: impl Iterator<Item = String>) -> Result<Self, String> {
        let usage_string = "./rgrep <string-to-search> <file-path>";
        /*
         * Handling insufficient cmd line args. In Rust, Suppose if user calls binary with ./out_binary arg1 arg2 then
         * rust receives 3 args. Rust includes binary name as first arg so even if no args are given then there will be
         * 1 arg always just like in c. So user should only know the number of args they give not the implicit arg rust takes.
         */
        // ignoring file name
        cmd_args.next();

        let query = match cmd_args.next() {
            Some(arg) => arg,
            None => return Err(format!("Required parameter query string not found\n{usage_string}")),
        };

        let file_path = match cmd_args.next() {
            Some(arg) => arg,
            None => return Err(format!("Required parameter file path not found \n{usage_string}")),
        };

        let ignore_case: bool = env::var("RGREP_IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let search_results = if config.ignore_case {
        println!(
            "Performing case insensitive search for {} in {} as RGREP_IGNORE_CASE is set\n",
            config.query, config.file_path
        );
        search_case_insensitive(&config.query, &contents)
    } else {
        println!(
            "Performing case sensitive search for {} in {} as RGREP_IGNORE_CASE is not set\n",
            config.query, config.file_path
        );
        search(&config.query, &contents)
    };
    for line in search_results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // This variable lifetime will only be until end of this function. Shadowing !!!
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        );
    }
}
