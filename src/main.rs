use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use uplc::ast::{DeBruijn, Program};

fn main() {
    let markers: Vec<&str> = vec![
        "delay[(error)(force(error))]",
        "List/Tuple/Constrcontainsmoreitemsthanexpected",
        "ExpectednoitemsforList",
        "ExpectednofieldsforConstr",
        "ExpectedonincorrectBooleanvariant",
        "ExpectedonincorrectConstrvariant",
        "Constrindexdidn'tmatchatypevariant",
        "(force(builtinmkCons))])(force(builtinheadList))])(force(builtintailList))",
    ];

    let filename = format!(
        "./data/{}/scripts.csv",
        env::var("NETWORK").expect("Missing ENV var 'NETWORK'")
    );

    let mut is_first_validator = true;

    if let Ok(lines) = read_lines(filename) {
        for row in lines.into_iter().flatten() {
            let hash: String = row.chars().take(56).collect();
            let cbor: String = row.chars().skip(57).collect();
            if is_aiken(&cbor, &markers) {
                let delim = if is_first_validator { "[" } else { "," };
                println!("{delim} \"{hash}\"");
                is_first_validator = false;
            }
        }
    }

    println!("]");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_aiken(cbor: &str, markers: &[&str]) -> bool {
    let program = Program::<DeBruijn>::from_hex(cbor, &mut Vec::new(), &mut Vec::new()).unwrap();
    let program = program.to_pretty().replace(['\n', ' '], "");

    for marker in markers {
        if program.contains(marker) {
            return true;
        }
    }

    false
}
