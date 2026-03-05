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
    let p = Parser::symbol(String::from("A"));
    let q = Parser::symbol(String::from("B"));
    fn c(s: &String) -> bool {
        s == "C"
    }
    let s = Parser::satisfy(c);
    fn f(a: &String, b: &String, c: &String) -> String {
    // fn f(a: &String, b: &String) -> String {
        let mut res = String::from(a);
        res.push_str(&b);
        res.push_str(&c);
        res
    }
    // TODO, make macro?
    let c1 = |a| { |b| { |c| f(a,b,c) }};
    let parser = fmap(&c1, &p);
    // let parser = fmap(move |a| move |b| f(&a,&b), &p);
    let parser = applicative(&parser, &q);
    let parser = applicative(&parser, &s);
    println!("{:?} ", parser.run(abc));
}
