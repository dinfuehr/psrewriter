use std::io::BufRead;
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

    let mut fin = BufReader::new(File::open(&args[1]).unwrap());
    let mut inside = false;

    let mut fout = BufWriter::new(File::create(Path::new(&args[2])).unwrap());
    let mut buffer = String::new();

    while fin.read_line(&mut buffer).unwrap() > 0 {
        let find = if inside { ">>" } else { "<<" };

        if buffer.contains(find) { inside = !inside; }
        if inside && buffer.ends_with("\n") { buffer.pop(); }

        fout.write(buffer.as_bytes()).unwrap();
        buffer.clear();
    }
}
