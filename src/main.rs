// use clap::Parser;
use miniscript::{Miniscript,Segwitv0};
use std::str::FromStr;
// use std::io::{self};



// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     policy: String,
// }


fn main() {
    // let args = Args::parse();

    // let stdin = io::stdin();
    for line in std::io::stdin().lines() {
        // println!("line: {}", line.unwrap());
        // }
        // for line in stdin.lock().lines() {
        // println!("Policy: {}", args.policy);
        let mut line = line.unwrap();
        line.retain(|c| !c.is_whitespace());
        // let clean = match line {
        //     Ok(line) => line.retain(|c| !c.is_whitespace()),
        //     Err(error) => panic!("Error: {}", error),
        // };

        let ms = match Miniscript::<String, Segwitv0>::from_str(&line) {
            Ok(ms) => ms,
            Err(error) => panic!("Error: {}", error),
        };
    println!("{}", ms.script_size());
    // Sipa: println!("X {%17.10f} %5i %s %s\n", ret->ScriptSize() + avgcost, (int)ret->ScriptSize(), Abbreviate(std::move(*str)).c_str(), line.c_str());kk
}

}
