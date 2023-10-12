use solidhunter_lib::linter::SolidLinter;
use solidhunter_lib::types::{LintDiag, Position};
use std::{fs, path::PathBuf};

struct Finding {
    start: Position,
    end: Position,
    id: String,
}

fn test_directory(base_name: &str) {
    let mut source = String::new();
    let mut config = String::new();
    let mut expected_findings: Vec<Finding> = Vec::new();

    for path in fs::read_dir(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("testdata")
            .join(base_name),
    )
    .unwrap()
    {
        let path = path.unwrap().path();

        if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
            if filename == "file.sol" || filename == "file.t.sol" {
                source = path.to_str().unwrap().to_string();
            } else if filename == ".solidhunter.json" {
                config = path.to_str().unwrap().to_string();
            } else if filename == "findings.csv" {
                for line in fs::read_to_string(path).unwrap().lines() {
                    let splitted_line: Vec<&str> = line.split(':').collect();
                    expected_findings.push(Finding {
                        start: Position {
                            line: splitted_line[1].parse::<usize>().unwrap(),
                            character: splitted_line[2].parse::<usize>().unwrap(),
                        },
                        end: Position {
                            line: splitted_line[3].parse::<usize>().unwrap(),
                            character: splitted_line[4].parse::<usize>().unwrap(),
                        },
                        id: splitted_line[0].to_string(),
                    });
                }
            }
        }
    }

    test_linter(&config, &source, &expected_findings);
}

fn test_linter(config: &str, source: &str, expected_findings: &Vec<Finding>) {
    let mut linter: SolidLinter = SolidLinter::new();
    let _ = linter.initialize_rules(&String::from(config));

    let result = linter.parse_file(String::from(source));
    let mut found_findings: Vec<&Finding> = Vec::new();
    let mut not_found_findings: Vec<&Finding> = Vec::new();
    let mut not_needed_findings: Vec<&LintDiag> = Vec::new();
    let mut not_needed_found = false;
    let mut not_found = false;

    match result {
        Ok(diags) => {
            let mut found;
            for (_, diag) in diags.iter().enumerate() {
                found = false;
                for (_, expected_finding) in expected_findings.iter().enumerate() {
                    if (diag.range.start == expected_finding.start)
                        && (diag.range.end == expected_finding.end)
                        && (diag.id == expected_finding.id)
                    {
                        found_findings.push(expected_finding.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    not_needed_findings.push(diag);
                }
            }
            for (_, expected_finding) in expected_findings.iter().enumerate() {
                found = false;
                for (_, found_finding) in found_findings.iter().enumerate() {
                    if (expected_finding.start == found_finding.start)
                        && (expected_finding.end == found_finding.end)
                        && (expected_finding.id == found_finding.id)
                    {
                        found = true;
                        break;
                    }
                }
                if found == false {
                    not_found_findings.push(expected_finding.clone());
                }
            }
            if not_needed_findings.len() > 0 {
                println!("Diagnostics not expected:");
                for (_, finding) in not_needed_findings.iter().enumerate() {
                    println!(
                        "{}:{}:{}:{}:{}",
                        finding.id,
                        finding.range.start.line,
                        finding.range.start.character,
                        finding.range.end.line,
                        finding.range.end.character
                    );
                }
                not_needed_found = true;
            }
            if not_found_findings.len() > 0 {
                println!("\nMissing diagnostics:");
                for (_, finding) in not_found_findings.iter().enumerate() {
                    println!(
                        "{}:{}:{}:{}:{}",
                        finding.id,
                        finding.start.line,
                        finding.start.character,
                        finding.end.line,
                        finding.end.character
                    );
                }
                not_found = true;
            }
            assert_eq!(not_needed_found == true || not_found == true, false, "There are some missing or not needed diagnostics:\n Not needed found: {}\n Not found: {}", not_needed_findings.len(), not_found_findings.len());
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

macro_rules! test_directories {
    ($($dir:ident),+ $(,)?) => {$(
        #[allow(non_snake_case)]
        #[test]
        fn $dir() {
            test_directory(stringify!($dir));
        }
    )+};
}

test_directories! {
    ContractNameCamelCase,
    FunctionMaxLines,
    ImportOnTop,
    MaxLineLength,
    MaxStatesCount,
    FunctionNameMixedCase,
    FunctionParamNameMixedCase,
    UseForbiddenName,
    ReasonString,
    NoInlineAssembly,
    FunctionVisibility,
    OneContractPerFile,
    CustomErrors,
    EventNameCamelCase,
    ConstNameSnakeCase,
    StateVisibility,
    NoEmptyBlock,
    NoConsole,
    ExplicitTypes,
    ImplicitTypes,
    PayableFallback,
    VisibilityModifierOrder,
    VarNameMixedCase,
    ModifierNameMixedcase,
    NoGlobalImport,
    NotRelyOnTime,
    NamedParametersMapping,
    Ordering,
    PrivateVarsLeadingUnderscore,
    FoundryTestFunctions,
    AvoidTxOrigin,
}
