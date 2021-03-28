use boss_time_estimate::BossPhase;
use csv::Reader;
use rayon::prelude::*;
use std::env;
use std::error::Error;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => {
            println!("How can you do this?");
            return;
        }
        1 => {
            println!("Usage: cargo run <boss>");
            return;
        }
        _ => (),
    }
    let dir_path = "data/";
    let filename = &args[1];
    let suffix = ".csv";
    let file_path = dir_path.to_string() + filename + suffix;
    let boss = read_boss(&file_path);
    match boss {
        Ok(phases) => {
            println!(
                "{:<10}{:>10}{:>10}{:>10}",
                "Phase", "Power", "Semi", "Condi"
            );
            let arr: Vec<_> = phases
                .par_iter()
                .map(|p| (p.phase(), p.power_time(), p.semi_time(), p.condi_time()))
                .collect();
            arr.iter()
                .for_each(|a| println!("{:<10}{:>10.2}{:>10.2}{:>10.2}", a.0, a.1, a.2, a.3,));
            let sums: (f64, f64, f64) = arr
                .iter()
                .map(|(_, a, b, c)| (a, b, c))
                .fold((0.0, 0.0, 0.0), |acc, x| {
                    (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2)
                });
            println!(
                "{:<10}{:>10.2}{:>10.2}{:>10.2}",
                "Total", sums.0, sums.1, sums.2,
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    };
}

fn read_boss(filename: &str) -> Result<Vec<BossPhase>, Box<dyn Error>> {
    let mut reader = Reader::from_path(filename)?;
    let mut phases: Vec<BossPhase> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let hp: f64 = record[1].parse()?;
        let c: f64 = record[2].parse()?;
        let pc: f64 = record[3].parse()?;
        let n: f64 = record[4].parse()?;

        phases.push(BossPhase::init(record[0].to_string(), hp, c, pc, n));
    }

    Ok(phases)
}
