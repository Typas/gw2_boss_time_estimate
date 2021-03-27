use csv::Reader;
use rayon::prelude::*;
use boss_time_estimate::BossPhase;
use std::env;
use std::error::Error;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => {
            println!("How can you do this?");
            return;
        },
        1 => {
            println!("Usage: cargo run <boss>");
            return;
        },
        _ => (),
    }
    let dir_path = "data/";
    let filename = &args[1];
    let suffix = ".csv";
    let file_path = dir_path.to_string() + filename + suffix;
    let boss = read_boss(&file_path);
    match boss {
        Ok(phases) => {
            println!("{:<6}{:>8}{:>8}{:>8}", "Phase", "Power", "Semi", "Condi");
            phases.iter().for_each(|p| {
                println!(
                    "{:<6}{:>8.2}{:>8.2}{:>8.2}",
                    p.phase(),
                    p.power_time(),
                    p.semi_time(),
                    p.condi_time()
                )
            });
            println!(
                "{:<6}{:>8.2}{:>8.2}{:>8.2}",
                "Total",
                phases.par_iter().map(|p| p.power_time()).sum::<f64>(),
                phases.par_iter().map(|p| p.semi_time()).sum::<f64>(),
                phases.par_iter().map(|p| p.condi_time()).sum::<f64>()
            );
        }
        Err(e) => println!("Error: {}", e),
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
