#![feature(io)]
#![feature(unicode)]

use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::path::Path;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        panic!("psrewriter <in> <out>");
    }

    let fin = BufReader::new(File::open(&args[1]).unwrap());
    let mut last = ' ';
    let mut inside = false;

    let mut fout = BufWriter::new(File::create(Path::new(&args[2])).unwrap());
    let mut buffer: Vec<u8> = vec![0; 4];

    for ch in fin.chars() {
        let ch = ch.unwrap();

        if inside {
            if ch == '\n' {
                // ignore
                continue;
            } else if ch == '>' && last == '>' {
                inside = false;
            }
        } else if ch == '<' && last == '<' {
            inside = true;
        }

        ch.encode_utf8(&mut buffer).unwrap();
        fout.write(&buffer).unwrap();
        last = ch;
    }
}
