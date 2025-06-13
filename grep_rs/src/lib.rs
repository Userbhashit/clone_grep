use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead},
    path::Path
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    pub query: String,
    pub file_path: String,

    #[arg(short = 'r', long, default_value_t = false)]
    pub directory: bool,

    #[arg(short = 'i', long, default_value_t = false)]
    pub case_insensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    if !config.directory {
        let contents = fs::read_to_string(config.file_path)?;

        if config.case_insensitive {
            search_case_insensitive(&config.query, &contents);
        } else {
            search(&config.query, &contents);
        }
    } else {
        let dir_str = config.file_path;
        let dir = Path::new(&dir_str);

        let _ = visit_dirs(config.query.as_str(), dir, config.case_insensitive);
    }

    Ok(())
}

pub fn search(query: &str, contents: &str) {
    let mut count = 0;

    for line in contents.lines() {
        if line.contains(query) {
            println!("{}", line);
            count += 1;
        }
    }

    println!("\nFound {} in {} lines.", query, count);
}

pub fn search_case_insensitive(query: &str, contents: &str) {
    let query = query.to_lowercase();
    let mut count = 0;

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            println!("{}", line);
            count += 1;
        }
    }

    println!("\nFound {} in {} lines.", query, count);
}

pub fn visit_dirs(query: &str, dir: &Path, case_insesitive: bool) -> io::Result<()> {
    if dir.is_dir() {
        for file in dir.read_dir()? {
            let file = file?;
            let path = file.path();

            if path.is_dir() {
                visit_dirs(query, &path, case_insesitive)?;
            } else if path.is_file() {
                println!("Opeaning file: {:?}",path);

                let file = File::open(&path)?;
                let reader = io::BufReader::new(file);


                for line in reader.lines() {
                    let line = line?;
                    if case_insesitive {
                        if line.to_lowercase().contains(query) {
                            println!("{}", line);
                        }
                    } else {
                        if line.contains(query) {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
