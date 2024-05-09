use csv::{Reader, ReaderBuilder, Writer};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, usize};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please enter a valid path to a .vcf");
    let genotypes = env::args()
        .nth(1)
        .expect("Please enter a .csv that contains the dersired accessions");
    get_hms(path);
}

fn get_hms(csv_path: String) {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(csv_path)
        .unwrap();
    let result = rdr.records().next();
    println!("{:?}", result);
}
