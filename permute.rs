//extern crate toml;
extern crate serde;
extern crate serde_json;

use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Clone, serde::Deserialize)]
struct Option {
    name: String,
    index: u64
}

#[derive(serde::Deserialize)]
struct Output {
    channels: String,
    format_string: String,
    ignore_duplicates: bool,
    include_least_index: u32
}

#[derive(serde::Deserialize)]
struct Permutation {
    channel_map: HashMap<String, Vec<Vec<Option>>>,
    outputs: Vec<Output>
}

fn main() {        
    let args: Vec<String> = env::args().collect();
    let mut cnt = 0;
    // files
    for a in args {
        if cnt > 0 {
            let contents = fs::read_to_string(a).expect("error reading file!");
            print_permutations(contents);
        }
        cnt = cnt + 1;
    }
}

fn print_permutations(toml_str: String) {
    let input: Permutation = serde_json::from_str(&toml_str).unwrap();
    for output in &input.outputs {
        println!("// output {}", output.channels);
        let channels = input.channel_map[&output.channels].to_vec();
        permute(Vec::new(), channels, &output.format_string, output.ignore_duplicates, output.include_least_index);
    }
}

fn permute(result: Vec<Option>, channels: Vec<Vec<Option>>, format: &String, ignore_duplicate_elements: bool, include_least_index: u32) {
    let mut todo_channels = channels.to_vec();
    if todo_channels.len() > 0 {
        let chan = todo_channels.pop();
        for o in chan.iter() {
            for v in o.iter() {
                let mut sub_res = result.to_vec();
                sub_res.push(v.clone());
                permute(sub_res, todo_channels.to_vec(), format, ignore_duplicate_elements, include_least_index);
            }
        }
    }
    else {
        // format string
        let mut output = format.clone();
        let mut cnt = 0;
        let mut duplicate = 0;
        let mut include = 0;
        for n in 0..result.len() {
            if result[n].index >= include_least_index as u64 {
                include = 1;
            }
        }
        for n in 0..result.len() {
            for l in 0..result.len() {
                if n != l && result[n].index == result[l].index {
                    duplicate = 1;
                }
            }
        }
        // ignore duplicate elements only allows permutations where each element is different
        if ignore_duplicate_elements {
            if duplicate > 0 {
                return;
            }
        }
        // restrict which entries we care about by only accepting permuations with at least index
        if include == 0 {
            return;
        }
        output = output.replace("%d", &duplicate.to_string());
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