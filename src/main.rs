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
    let mut buffer = Vec::new();

    while fin.read_until(b'\n', &mut buffer).unwrap() != 0 {
        let len = buffer.len();

        if len > 1 {
            for i in 1..len {
                let ch = buffer[i];
                let last = buffer[i-1];

                if (inside && ch == b'>' && last == b'>') || (!inside && ch == b'<' && last == b'<') {
                    inside = !inside;
                }
            }
        }

        if inside && buffer[len-1] == b'\n' {
            buffer.pop();
        }

        fout.write(&buffer).unwrap();
        buffer.clear();
    }
}
