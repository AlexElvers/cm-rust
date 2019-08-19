use std::env;
use std::path::Path;
use std::process::exit;

mod high_score_table;


fn high_scores(level: Option<&str>) {
    match level {
        Some(level) => {
            let path = Path::new(".").join("MINING").join(level).join("high.dat");
            if !path.exists() {
                eprintln!("high scores file of {} does not exist", level);
                exit(1);
            }
            let high_scores = high_score_table::load(&path).unwrap();
            for (i, high_score) in high_scores.iter().enumerate() {
                println!("{}. {:?}", i + 1, high_score);
            }
        }
        None => panic!("not implemented yet"),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match args[1].as_ref() {
            "high-scores" => {
                let level =
                    if args.len() == 3 {
                        Some(args[2].as_ref())
                    } else {
                        None
                    };
                high_scores(level);
                return;
            }
            _ => {}
        }
    }
    eprintln!("unknown usage");
    exit(1);
}
