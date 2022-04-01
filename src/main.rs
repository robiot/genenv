use colored::*;
use path_clean::PathClean;
use std::fs;
use std::path;
use structopt::StructOpt;

mod input;

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        eprintln!("{} {}", "error:".red().bold(), format_args!($($arg)*));
    }};
}

fn absolute_path(path: impl AsRef<path::Path>) -> std::io::Result<path::PathBuf> {
    let path = path.as_ref();
    Ok(if path.is_absolute() {
        path.to_path_buf().clean()
    } else {
        std::env::current_dir()?.join(path).clean()
    })
}

fn find_env(
    path: &str,
    opts: &input::Opt,
    found_before: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut found = found_before;

    if opts.path_exclude.len() > 0 {
        let path_exclude: Vec<String> = opts.path_exclude.iter().map(|i| i.replace("/", "")).collect();

        if path_exclude.contains(&path::Path::new(path).file_name().unwrap().to_str().unwrap().to_string()) {
            return Ok(found);
        }
    }

    for entry in match fs::read_dir(path) {
        Ok(f) => f,
        Err(err) => {
            return Err(Box::new(err));
        }
    } {
        let file = match entry {
            Ok(f) => f,
            Err(err) => {
                log_error!("Unexpected error : {}", err);
                continue;
            }
        };

        if match file.metadata() {
            Ok(m) => m,
            Err(err) => {
                log_error!(
                    "Could not read the metadata of '{}' : {}",
                    file.path().display(),
                    err
                );
                continue;
            }
        }
        .is_dir()
        {
            if opts.recursive {
                found = find_env(
                    &absolute_path(file.path())?.display().to_string(),
                    opts,
                    found,
                )?;
            }
            continue;
        }

        // The slash to prevent other files ending with .env
        if file.path().display().to_string().ends_with("/.env") {
            found = true;
            let iter = match dotenvy::from_path_iter(file.path()) {
                Ok(v) => v,
                Err(err) => {
                    return Err(Box::new(err));
                }
            };

            // Grab all keys
            let mut keys: Vec<String> = vec![];
            for object in iter {
                keys.push(
                    match object {
                        Ok(v) => v,
                        Err(err) => {
                            return Err(Box::new(err));
                        }
                    }
                    .0,
                );
            }

            let output = path::Path::new(&absolute_path(path)?)
                .join(format!("{}.example", file.file_name().to_str().unwrap()))
                .clean()
                .display()
                .to_string();

            println!("{} to '{}'", "Writing".purple().bold(), output);

            let value = format!("={}\n", opts.value);
            fs::write(output, keys.join(&value) + &value)?;
        }
    }

    Ok(found)
}

fn main() {
    let opts = input::Opt::from_args();

    match find_env(&opts.path, &opts, false) {
        Ok(found) => {
            if found {
                println!("{}", "Success".green().bold());
            } else {
                log_error!("No file was found");
            }
        }
        Err(err) => {
            log_error!("{}", err);
        }
    }
}
