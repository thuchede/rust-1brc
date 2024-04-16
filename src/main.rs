use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::time::Instant;

fn main() {
    println!("Opening sample file");
    let timer = Instant::now();
    let buf = open("data/sample.csv"); // using a sample
    // let buf = open("data/measurements.txt"); //
    let mut hash: HashMap<String, Vec<f64>> = HashMap::new();

    buf.unwrap().lines().for_each(|rline| {
        let line = rline.unwrap().clone();
        let (city, temp_str) = line.split_once(";").unwrap();
        // println!("{city}, {temp_str}");
        let temp = temp_str.parse::<f64>().unwrap_or_else(|t| {
            // println!("ERR {:?}", t);
            0f64
        });
        hash.entry(city.to_string()).and_modify(|i| i.push(temp)).or_insert(vec![temp]);
    });

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut w = io::BufWriter::new(lock);
    write!(&mut w, "{{");

    let mut res: Vec<(String, f64, f64, f64)> = hash.iter().map(|(a, b)| {
        let mean: f64 =  b.iter().sum::<f64>() / (b.len() as f64);
        let min: f64 =  b.iter().fold(f64::INFINITY, |a, &b| a.min(b)); //.min_by(|&a, &b| { a.total_cmp(b) }).unwrap();
        let max: f64 =  b.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        (a.clone(), min, mean, max)
    })
    .collect::<Vec<(String, f64, f64, f64)>>();

    res.sort_by(|(a, _, _, _), (b, _, _, _)|{
        a.cmp(b)
    });

    let next_c = "";

    res.iter().enumerate().for_each(|(i, (a,b,c,d))| {
        let prefix = match i {
            0 => "",
            _ => ", "
        };
        let text = format!("{}{}={:.2}/{:.2}/{:.2}", prefix, a, b, c, d);
        write!(&mut w, "{}", text);
    });

    write!(&mut w, "}}\n");
    w.flush();

    println!("Done in {}ms", timer.elapsed().as_millis());
}

fn open(filepath: &str) -> Result<BufReader<File>, Error> {
    let file = match File::open(filepath) {
        Ok(f) => f,
        Err(_) => return Err(Error::new(ErrorKind::NotFound,format!("file {} does not exist!", filepath)))
    };
    Ok(BufReader::new(file))
}
