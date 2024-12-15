use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug, PartialEq)]
pub struct Clientrecord { //attributes of a client
    pub id: String,
    pub gender: String,
    pub property: String,
    pub children: String,
    pub income: u32,
    pub married: String,
}

pub fn load(path: &str) -> Result<Vec<Clientrecord>, Box<dyn Error>> { //loads csv file from path and returns Clientrecord instances
    let data = fs::read_to_string(path)?;
    let mut records = Vec::new();
    let lines = data.lines().skip(1);
    for line in lines {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 9 {
            let income = fields[5].parse().unwrap_or(0);
            records.push(Clientrecord {
                id: fields[0].to_string(),
                gender: fields[1].to_string(),
                property: fields[3].to_string(),
                children: childrencategories(fields[4].parse().unwrap_or(0)),
                income,
                married: fields[8].to_string(),
            });
        }
    }
    Ok(records)
}

pub fn getinput(prompt: &str) -> String { //takes a prompt and prints response for string response
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn numberinput(prompt: &str) -> u32 { //takes a prompt and prints response for u32 integer response
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Not a valid number!")
}

pub fn childrencategories(children: u32) -> String { //makes children easier by limiting the bins they can be in
    match children {
        0 => "No children".to_string(),
        1..=2 => "1-2 children".to_string(),
        3..=4 => "3-4 children".to_string(),
        _ => "5+ children".to_string(),
    }
}

