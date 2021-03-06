use fst::SetBuilder;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use xz2::read::XzDecoder;

fn main() {
    let input_path =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("data/rockyou.txt.xz");
    let input = io::BufReader::new(XzDecoder::new(File::open(input_path).unwrap()));

    let output_path = Path::new(&env::var("OUT_DIR").unwrap()).join("password_hash.fst");
    let output = io::BufWriter::new(File::create(output_path).unwrap());

    let mut passwords = input
        .split(b'\n')
        .filter_map(|line| {
            let line = line.unwrap();
            match String::from_utf8(line) {
                Ok(line) => Some(line),
                Err(e) => {
                    eprintln!(
                        "Error processing {}: {:?}",
                        String::from_utf8_lossy(&e.as_bytes()),
                        e
                    );
                    None
                }
            }
        })
        .take(1_000_000)
        .collect::<Vec<_>>();
    passwords.sort();

    let mut gen = SetBuilder::new(output).unwrap();
    for password in passwords.iter() {
        gen.insert(password).unwrap();
    }
    gen.finish().unwrap();
}
