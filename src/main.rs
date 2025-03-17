// use clap::Parser;
use miniscript::BareCtx;
use miniscript::Error;
use miniscript::Legacy;
use miniscript::{policy, Miniscript, Segwitv0};
use policy::Liftable;
use std::str::FromStr;
// use std::io::{self};

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     policy: String,
// }

fn main() {
    // let args = Args::parse();

    type Script = Miniscript<String, Legacy>;
    type Policy = policy::Concrete<String>;

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
        let pol = match Policy::from_str(&line) {
            Ok(pol) => pol,
            Err(error) => panic!("Error: {}", error),
        };
        if let Ok(desc) = pol.compile::<Legacy>() {
            // Lift
            // assert_eq!(desc.lift().unwrap().sorted(), pol.lift().unwrap().sorted());
            // // Try to roundtrip the output of the compiler
            // print!("{}", desc.script_size());
            let output = desc.to_string();
            if let Ok(desc) = Script::from_str(&output) {
                let _ = desc.to_string();
                println!("{} {} {}", desc.script_size(), desc, pol);
                // assert_eq!(output.to_lowercase(), rtt.to_lowercase());
            } else {
                panic!("compiler output something unparseable: {}", output)
            }
        }
        // println!("{}", pol);

        // let ms = match Miniscript::<String, Legacy>::from_str(&line) {
        //     Ok(ms) => ms,
        //     Err(error) => panic!("Error: {}", error),
        // };
        // println!("{}", ms.script_size());
        // Sipa: println!("X {%17.10f} %5i %s %s\n", ret->ScriptSize() + avgcost, (int)ret->ScriptSize(), Abbreviate(std::move(*str)).c_str(), line.c_str());kk
    }
}
