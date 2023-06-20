use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::parse_config(&args) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    if let Err(e) =  minigrep::run(config){
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
    
}


