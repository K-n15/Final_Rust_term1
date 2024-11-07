use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader,Write};
use std::path::Path;

pub fn open_log()->Option<File>{
    if Path::new(&String::from("log.txt")).exists(){
        let new = File::open(&String::from("log.txt"));
        match new {
            Ok(k)=>return Some(k),
            Err(_e)=>None
        }
    } else{
        let new = File::create(&String::from("log.txt"));
        match new {
            Ok(k)=>return Some(k),
            Err(_e)=>None
        }
    }
}

pub fn readrecord_file(x:File)->HashMap<String,Vec<String>>{
    let temp = BufReader::new(&x);
    let mut hashing = HashMap::<String,Vec<String>>::new();
    for i in temp.lines(){
        let mut word: Vec<String> = Vec::new();
        match i {
            Ok(k)=>{
                word = k.split_whitespace().into_iter().map(|x|x.to_string()).collect();
            },
            Err(_e)=>continue
        }
        let name = word[0].clone();
        word.remove(0);
        hashing.insert(name, word);
    }
    return hashing;
}

pub fn record_log(x:HashMap<String,Vec<String>>){
    let logging = File::create(&String::from("log.txt")).expect("Inoccurace occured");
    for mut i in x{
        let mut line: Vec<String> = Vec::new();
        line.push(i.0);
        if i.1.len() >= 35 {
            line.append(&mut i.1[25..].to_vec());
        } else {
        line.append(&mut i.1);
        }
        writeln!(&logging,"{:?}",line).expect("Unexpected error occured");
    }
}