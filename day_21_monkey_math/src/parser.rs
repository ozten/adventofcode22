use std::fs::read_to_string;
use std::collections::HashMap;

pub enum Formula {
    N(isize),
    F(String)
}

pub fn parse(filename: &str, cache: &mut HashMap<String, Formula>) {
    
    let data = read_to_string(filename).unwrap();
    for line in data.lines() {
        if ! line.is_empty() {
            
            let tmp = line.split(": ");
            let mut parts = tmp.into_iter();
            
            let node = parts.next().unwrap().to_owned();
            let rest = parts.next().unwrap();
            println!("node={:?} rest={:?}", node, rest);
            match rest.parse::<isize>() {
                Ok(num) => {
                    cache.insert(node, Formula::N(num.clone()));
                },
                Err(error) => {
                    cache.insert(node, Formula::F(rest.clone().to_owned()));
                }
            }
        }
    }
}
