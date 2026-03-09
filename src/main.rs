use std::{env, fs};

pub mod parser; 
pub mod types; 
use crate::{parser::{Parser, applicative, fmap, many}};

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
    let abc = "ABCDEFGHIGJK".chars().map(|c| char::to_string(&c)).collect::<Vec<_>>();
    let abc = abc.iter().collect::<Vec<_>>();

    let a = String::from("A");
    let b = String::from("B");
    let e = String::from("E");

    let p = Parser::symbol(&a);
    let q = Parser::symbol(&b);
    let r = Parser::symbol(&e);

    fn c_or_d(s: &String) -> bool {
        s == "C" || s == "D"
    }
    let s = Parser::satisfy(c_or_d);
    let ch = parser::choice(p.clone(), r);

    // still need to decide what type i want the res to be
    fn f(a: &String, b: &String, c: &String, d: &String, e: &String) -> Vec<String> {
        vec![a.clone(),b.clone(),c.clone(),d.clone(),e.clone()]
    }
    // TODO, make macro?
    let c1 = |a| move |b| move |c| move |d| move |e| f(a,b,c,d,e);
    let parser = fmap(c1, p.clone());
    let parser = applicative(parser, q);
    let parser = applicative(parser, s.clone());
    let parser = applicative(parser, s.clone());
    let parser = applicative(parser, ch);

    println!("{:?} ", parser.run(&abc));

    let parser = many(p.clone());

    println!("{:?} ", parser.run(&abc));
}
