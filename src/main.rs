use std::{env, fs};

pub mod parser; 
pub mod types; 
use crate::{parser::{Parser, applicative, fmap}};

fn main() {
    // take in command line args
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // let path = &args[1];
    // println!("Searching path '{}'", path);
    //
    // let contents = fs::read_to_string(path).expect("Unable to read path ");
    //
    // println!("File contains \n '{}'", contents);

    // let p = Parser::symbol("A").run(vec!["A", "B", "C", "D"]);
    // let abc = vec!["A", "B", "C", "D"];
    let abc = "ABCD".chars().map(|c| char::to_string(&c)).collect::<Vec<_>>();
    let abc = abc.iter().collect::<Vec<_>>();
    let a = String::from("A");
    let b = String::from("B");

    let p = Parser::symbol(&a);
    let q = Parser::symbol(&b);

    fn c_or_d(s: &String) -> bool {
        s == "C" || s == "D"
    }
    let s = Parser::satisfy(c_or_d);

    fn f(a: &String, b: &String, c: &String, d: &String) -> String {
        let mut res = String::from(a);
        res.push_str(b);
        res.push_str(c);
        res.push_str(d);
        res
    }
    // TODO, make macro?
    let c1 = |a| move |b| move |c| move |d| f(a,b,c,d);
    let parser = fmap(&c1, &p);
    let parser = applicative(&parser, &q);
    let parser = applicative(&parser, &s);
    let parser = applicative(&parser, &s);
    println!("{:?} ", parser.run(abc));
}
