#![feature(slice_split_once)]

use std::collections::{BTreeMap};
use std::fs::File;
use std::io;
use fxhash::{FxHashMap};
use std::io::{BufRead, BufReader, Write};
use std::ops::Add;
use std::time::Instant;
use fast_float::parse;
use lazy_static::lazy_static;
use rayon::prelude::*;
use memmap2::MmapOptions;

struct Record {
    min: f32,
    sum: f32,
    max: f32,
    count: i32,
}

impl Record {
    fn new(value: f32) -> Self {
        Record {
            min: value,
            sum: value,
            max: value,
            count: 0,
        }
    }
    fn add(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.sum = self.sum.add(value);
        self.max = self.max.max(value);
        self.count = self.count.add(1);
    }
    fn mean(&self) -> f32 {
        self.sum / (self.count as f32)
    }
}

lazy_static! {
    static ref TEMP_VALUES: FxHashMap<Vec<u8>, f32> = {
        let mut map = FxHashMap::default();
        for int in -1000..=1000 {
            for dec in -9..=9 {
                if dec == 0 {
                    let key = format!("{}", int);
                    map.insert(key.as_bytes().to_vec(), int as f32);
                } else {
                    let val = int as f32 + 0.1 * (dec as f32);
                    let key = format!("{}", val);
                    map.insert(key.as_bytes().to_vec(), val);
                }
            }
        }
        map.insert("-0".as_bytes().to_vec(), 0.0);
        map
    };
}

fn main() {
    let timer = Instant::now();

    let mut hash: FxHashMap<String, Record> = FxHashMap::default();
    let file = File::open("data/measurements.txt").unwrap();

    let cores: usize = std::thread::available_parallelism().unwrap().into();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    let chunk_size = mmap.len() / cores;
    let mut chunks: Vec<(usize, usize)> = vec![];
    let mut start = 0;
    for _ in 0..cores {
        let end = (start + chunk_size).min(mmap.len());
        let next_new_line = match memchr::memchr(b'\n', &mmap[end..]) {
            Some(v) => v,
            None => {
                assert_eq!(end, mmap.len());
                0
            }
        };
        let end = end + next_new_line;
        chunks.push((start, end));
        start = end + 1;
    }


    let mut buf = Vec::with_capacity(4096);
    let mut reader = BufReader::new(file);
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n == 0 {
            break;
        }
        let line: &[u8] = &buf[..n - 1];
        if let Some((city, value)) = line.split_once(|&b| b == b';') {
            let value = *TEMP_VALUES
                .get(value)
                .unwrap();
            let city = unsafe {
                String::from_utf8_unchecked(city.to_vec())
            };
            hash.entry(city).and_modify(|record| record.add(value)).or_insert(Record::new(value));
        }
        // Clear buffer
        buf.clear();
    }

    let reading = timer.elapsed().as_millis();
    println!("Done reading in {}ms", reading);

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut w = io::BufWriter::new(lock);

    let res: BTreeMap<String, Record> = hash.into_iter().collect();

    let sorting = timer.elapsed().as_millis() - reading;
    println!("Done sorting in {}ms", sorting);

    write!(&mut w, "{{");
    res.iter().enumerate().for_each(|(i, (a,b))| {
        let prefix = match i {
            0 => "",
            _ => ", "
        };
        let info = format!("{}{}={:.2}/{:.2}/{:.2}", prefix, a, b.min, b.mean(), b.max);
        write!(&mut w, "{}", info);
    });

    write!(&mut w, "}}\n");

    let _ = w.flush();

    let printing = timer.elapsed().as_millis() - reading - sorting;
    println!("Done printing in {}ms", printing);

    println!("Done in {}ms", timer.elapsed().as_millis());
}