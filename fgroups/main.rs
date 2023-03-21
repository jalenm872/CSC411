use std::collections::HashMap;
use std::io;
//use std::io::BufRead;
use std::str;

//Helper Function from notes
fn ws(c: char) -> bool {
    c == ' ' || c == '\t'
}

//Main Function
fn main() {

    //Create a hash map
    let mut map: HashMap<String, Vec<String>> = std::collections::HashMap::new();

    //Create a vector to store the lines
    let mut lines: Vec<String> = Vec::new();

    loop{
        //Create a string to store the input
        let mut input_text = String::new();
        //Read the input from stdin
        io::stdin().read_line(&mut input_text).unwrap();
        //If the input is empty, break the loop
        if input_text.is_empty() {
            break;
        }
        //Push the input into the lines vector
        lines.push(input_text);
    }

    //Iterate over the lines vector
    lines.iter().for_each(|line| {
        //Split each element by the first space and store in a vector
        let line_words: Vec<&str> = line.trim().splitn(2, ws).collect();

        //If the vector has two elements, push the both elements into the words vector
        if line_words.len() != 2 {
        }
        else{
            //If the value repeats in the hash map, do not push it into the vector
            if map.contains_key(line_words[0].trim()) {
                let mut flag = false;
                for value in map.get(line_words[0].trim()).unwrap() {
                    if value == line_words[1].trim() {
                        flag = true;
                    }
                }
                if flag == false {
                    map.entry(line_words[0].trim().to_string()).or_insert(Vec::new()).push(line_words[1].trim().to_string());
                }
            }
            else{
                map.entry(line_words[0].trim().to_string()).or_insert(Vec::new()).push(line_words[1].trim().to_string());
            }
        }
    });

    //Remove keys with only one value
    map.retain(|_, v| v.len() != 1);

    //Print each value for each key in the HashMap
    //Iterate over each key in the HashMap
    for key in map.keys() {
        //Print each value for the current key
        for value in map.get(key).unwrap() {
            //println!("{}", value);
            print!("{}\n", value);
        }
        //If the last key was printed, do not print a newline
        if key != map.keys().last().unwrap() {
            println!();
        }
    }
}