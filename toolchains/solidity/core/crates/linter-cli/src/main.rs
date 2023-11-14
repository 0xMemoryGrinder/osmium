use clap::{arg, Parser};
use solidhunter_lib::errors::SolidHunterError;
use solidhunter_lib::linter::SolidLinter;
use solidhunter_lib::rules::rule_impl::create_rules_file;
use solidhunter_lib::types::LintResult;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        required = false,
        default_value = ".",
        help = "Path to the project to lint"
    )]
    path: String,
    #[arg(
        short = 'e',
        long = "exclude",
        help = "Exclude part of the project path"
    )]
    ignore_path: Vec<String>,

    #[arg(
        short = 'r',
        long = "rules",
        default_value = ".solidhunter.json",
        help = "Specify rules file"
    )]
    rules_file: String,

    #[arg(
        short = 'j',
        long = "json",
        default_value = "false",
        help = "Outputs a json format instead"
    )]
    to_json: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        default_value = "false",
        help = "Verbose output"
    )]
    verbose: bool,

    #[arg(
        short = 'i',
        long = "init",
        default_value = "false",
        help = "Initialize rules file"
    )]
    init: bool,
}

fn print_result(results: Vec<LintResult>) {
    for result in results {
        match result {
            Ok(diags) => {
                println!("{}", &diags);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn main() -> Result<(), SolidHunterError> {
    let args = Args::parse();

    if !args.to_json {
        println!();
        println!("SolidHunter: Fast and efficient Solidity linter");
        println!(
            "By {} - v{} - GNU GPL v3",
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_VERSION")
        );
        println!();
    }

    if args.verbose {
        println!("Verbose output enabled");
        println!("Project path: {:?}", args.path);
        println!("Exclude path: {:?}", args.ignore_path);
        println!("Using rules file: {}", args.rules_file);
        println!("Verbose output: {}", args.verbose);
    }

    if args.init {
        println!("Initializing rules file...");
        create_rules_file(".solidhunter.json");
        println!("Done!");
        return Ok(());
    }

    if !args.path.is_empty() {
        let mut linter: SolidLinter = SolidLinter::new();
        let mut path: String = args.rules_file;
        if args.path != "." && path == ".solidhunter.json" {
            path = args.path.as_str().to_owned() + "/" +  path.as_str();
        }
        linter.initialize_rules(&path)?;

        let result = linter.parse_path(&args.path);
        if !args.to_json {
            print_result(result);
        } else {
            for res in result {
                match res {
                    Ok(diags) => {
                        let json = serde_json::to_string_pretty(&diags);
                        match json {
                            Ok(j) => {
                                println!("{}", j);
                            }
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
    }
    Ok(())
}
