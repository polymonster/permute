extern crate toml;
extern crate serde;

use std::env;
use std::fs;

#[derive(Clone, serde::Deserialize)]
struct Option {
    name: String,
    index: u64
}

#[derive(serde::Deserialize)]
struct Permutation {
    channels: Vec<Vec<Option>>,
    format_string: String
}

fn main() {        
    let args: Vec<String> = env::args().collect();
    let mut cnt = 0;
    for a in args {
        if cnt > 0 {
            let contents = fs::read_to_string(a).expect("error reading file!");
            print_permutations(contents);
        }
        cnt = cnt + 1;
    }
}

fn print_permutations(toml_str: String)
{
    let input: Permutation = toml::from_str(&toml_str).unwrap();
    permute(Vec::new(), input.channels, &input.format_string);
}

fn permute(result: Vec<Option>, channels: Vec<Vec<Option>>, format: &String)
{
    let mut todo_channels = channels.to_vec();
    if todo_channels.len() > 0 {
        let chan = todo_channels.pop();
        for o in chan.iter() {
            for v in o.iter() {
                let mut sub_res = result.to_vec();
                sub_res.push(v.clone());
                permute(sub_res, todo_channels.to_vec(), format);
            }
        }
    }
    else {
        // format string
        let mut output = format.clone();
        let mut cnt = 0;
        for res in result.iter() {
            let searchi = format!("%i[{}]", cnt.to_string());
            let replacei: &str = &res.index.to_string()[..]; 
            output = output.replace(searchi.as_str(), replacei);
            let searchn = format!("%n[{}]", cnt.to_string());
            let replacen: &str = &res.name.to_string()[..]; 
            output = output.replace(searchn.as_str(), replacen);
            cnt = cnt + 1;
        }
        println!("{}",output);
    }
}