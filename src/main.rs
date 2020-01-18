use std::{env, fmt::Display, process};

use ansi_term::Colour;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};

macro_rules! error {
   ($($arg:tt)*) => ({
        eprintln!("{} {}", Colour::Red.bold().paint("cargo ci:"), format!($($arg)*));
        process::exit(255);
    })
}

macro_rules! info {
   ($($arg:tt)*) => ({
        println!("{} {}", Colour::Cyan.bold().paint("cargo ci:"), format!($($arg)*));
    })
}

fn bail(message: &str, error: impl Display) {
    error!("{}\n            due to: {}", message, error);
}

fn is_cargo_subcommand(subcommand: &str) -> bool {
    let output = process::Command::new("cargo")
        .arg("--list")
        .output()
        .map_err(|e| bail("failed to run `cargo --list`.", e))
        .unwrap();

    String::from_utf8_lossy(&output.stdout)
        .split('\n')
        .skip(1)
        .filter_map(|line| line.split_whitespace().next())
        .any(|c| c == subcommand)
}

fn should_run(only: Option<&str>, skip: Option<Vec<&str>>) -> bool {
    let version = env::var("TRAVIS_RUST_VERSION").unwrap_or_else(|_| {
        env::var("RUSTUP_TOOLCHAIN").unwrap_or_else(|_| {
            let output = process::Command::new("rustup")
                .arg("show")
                .arg("active-toolchain")
                .output()
                .map_err(|e| bail("failed to run `rustup show active-toolchain`.", e))
                .unwrap();
            String::from_utf8_lossy(&output.stdout).to_string()
        })
    });

    if let Some(only) = only {
        version.starts_with(only)
    } else if let Some(skip) = skip {
        for v in skip {
            if version.starts_with(v) {
                return false;
            }
        }
        true
    } else {
        true
    }
}

fn main() {
    let mut args: Vec<_> = env::args().collect();

    if args.len() > 1 && args[1] == "ci" {
        args[0] = args[0].replace("cargo-ci", "cargo ci");
        args.remove(1);
    }

    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::AllowExternalSubcommands)
        .help_message("Show this message and exit.")
        .version(crate_version!())
        .version_short("v")
        .version_message("Show the version and exit.")
        .arg(
            Arg::with_name("only")
                .long("only")
                .takes_value(true)
                .value_name("version")
                .help("Only run the command if we are using this version."),
        )
        .arg(
            Arg::with_name("skip")
                .long("skip")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .value_name("version")
                .conflicts_with("only")
                .help("Skip running the command if we are NOT using this version."),
        )
        .get_matches_from(&args);

    match matches.subcommand() {
        (subcommand, Some(submatches)) => {
            if should_run(
                matches.value_of("only"),
                matches.values_of("skip").map(|i| i.collect()),
            ) {
                let mut args: Vec<_> = submatches.values_of_lossy("").unwrap_or_else(|| vec![]);
                let (program, args) = if is_cargo_subcommand(subcommand) {
                    args.insert(0, subcommand.to_string());
                    ("cargo", args)
                } else {
                    (subcommand, args)
                };

                // Construct the command for display purposes only.
                let mut cmd = String::from(program);
                for arg in &args {
                    cmd.push(' ');
                    cmd.push_str(&arg);
                }
                info!("running `{}`.", cmd);

                // Run the subcommand.
                match process::Command::new(program)
                    .args(&args)
                    .stdout(process::Stdio::inherit())
                    .stderr(process::Stdio::inherit())
                    .output()
                    .map_err(|e| bail(&format!("failed to run `{}`.", cmd), e))
                    .unwrap()
                    .status
                    .code()
                {
                    Some(code) => process::exit(code),
                    None => error!("command terminated by signal."),
                }
            }
        }
        _ => error!("a subcommand is required."),
    }
}
