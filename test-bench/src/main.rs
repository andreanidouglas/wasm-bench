#![feature(exclusive_range_pattern)]

use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

#[derive(Deserialize, Serialize, Default)]
enum Ttype {
    #[default]
    ENUM_1,
    ENUM_2,
}

#[derive(Serialize, Deserialize, Default)]
struct BenchStruct {
    id: i32,
    value: String,
    ttype: Ttype,
}

fn make_request(counts: Arc<Mutex<Vec<u128>>>, i: i32) {
    let mut inner_counts = Vec::<u128>::new();
    for j in 0..100 {
 //       println!("Executing call: {} -> {}", i, j);
        let status: bool;
        let now = Instant::now();
        {
            let url = std::env::var("WASM_ENDPOINT").unwrap();//"https://wasm-bench-lp6uomye.fermyon.app/";
            let mut bench = BenchStruct::default();
            bench.id = i;
            let body = serde_json::to_string(&bench).expect("should not happen");

            let client = reqwest::blocking::Client::new();
            let resp = client.post(url).body(body).send();
            status = match resp {
                Ok(r) => r.status() == StatusCode::OK,
                Err(e) => {
                    println!("could not post to endpoint {:?}", e);
                    false
                }
            };
        }
        if status {
            let elapsed = now.elapsed().as_millis();
            println!("Elapsed: {} {} -> {} ms", i, j, elapsed);
            inner_counts.push(elapsed);
        }
    }

    counts
        .lock()
        .expect("could not lock thread")
        .append(&mut inner_counts);
}

#[derive(Eq, PartialEq)]
enum ResultBox {
    Ok0_300,
    Ok300_500,
    Ok500_600,
    Ok600_700,
    Ok700_800,
    Ok800_900,
    Ok900_3000,
    Ok3000,
}

fn main() -> Result<()> {
    let counts: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(Vec::new()));
    {
        let counts = counts.clone();
        let mut threads = Vec::new();
        for i in 0..10 {
            let counts = counts.clone();
            threads.push(thread::spawn(move || {
                make_request(counts, i);
            }));
        }

        for thread in threads {
            if let Err(res) = thread.join() {
                println!("Error joining thread: {:?}", res)
            }
        }
    }

    let mut results = Vec::<ResultBox>::new();
    let counts = counts.lock().expect("could not lock on main");
    for i in counts.iter() {
        let result_box = match i {
            0..300 => ResultBox::Ok0_300,
            300..500 => ResultBox::Ok300_500,
            300..500 => ResultBox::Ok300_500,
            500..600 => ResultBox::Ok500_600,
            600..700 => ResultBox::Ok600_700,
            700..800 => ResultBox::Ok700_800,
            800..900 => ResultBox::Ok800_900,
            900..3000 => ResultBox::Ok900_3000,
            _ => ResultBox::Ok3000,
        };
        results.push(result_box);
    }

    let n_requests: f32 = results.len() as f32;

    println!(
        "0-300 -> {} [{}%]",
        results.iter().filter(|x| **x == ResultBox::Ok0_300).count(),

        results.iter().filter(|x| **x == ResultBox::Ok0_300).count() as f32 / n_requests * 100f32
    );
    println!(
        "300-500 -> {} [{}%]",
        results
            .iter()
            .filter(|x| **x == ResultBox::Ok300_500)
            .count(),

        results.iter().filter(|x| **x == ResultBox::Ok300_500).count() as f32 / n_requests * 100f32
    );
    println!(
        "500-600 -> {} [{}%]",
        results
            .iter()
            .filter(|x| **x == ResultBox::Ok500_600)
            .count(),

        results.iter().filter(|x| **x == ResultBox::Ok500_600).count() as f32 / n_requests * 100f32
    );
    println!(
        "600-700 -> {} [{}%]",
        results
            .iter()
            .filter(|x| **x == ResultBox::Ok600_700)
            .count(),

        results.iter().filter(|x| **x == ResultBox::Ok600_700).count() as f32 / n_requests * 100f32
    );
     println!(
        "+900-3000 -> {} [{}%]",
        results.iter().filter(|x| **x == ResultBox::Ok900_3000).count(),
        results.iter().filter(|x| **x == ResultBox::Ok900_3000).count() as f32 / n_requests * 100f32
    );

  println!(
        "+3000 -> {} [{}%]",
        results.iter().filter(|x| **x == ResultBox::Ok3000).count(),
        results.iter().filter(|x| **x == ResultBox::Ok3000).count() as f32 / n_requests * 100f32
    );

    Ok(())
}
