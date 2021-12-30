use std::env;
use std::process;

use minigrep::Config;
 

fn main() {
    let arg: Vec<String> = env::args().collect();
 
    let config = Config::new(&arg).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments : {}", err);
        process::exit(1);
    });

    // println!("{:?}", arg);
    println!("Searchin for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep:: run(config){
        eprintln!("Application Error : {}", e);
        process::exit(1);
    };


}







