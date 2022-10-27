use std::io;
//use std::io::Read;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
fn main() {
  let mut count: i32 = 0;
  let mut number: Vec<String> = Vec::new();
  let mut numb: Vec<String> = Vec::new();
  println!("Choose what year to search in");
  let mut sea = String::new();
  io::stdin().read_line(&mut sea).expect("failed input");
  let mut sea: String = sea.trim().parse().expect("failed parse");
  println!("What do you want to know in the {} elections", sea);
  let mut userinput = String::new();
  io::stdin().read_line(&mut userinput).expect("failed input");
  let userinput: String = userinput
    .to_ascii_lowercase()
    .trim()
    .parse()
    .expect("failed parse");
  sea.push_str(" parties.txt");

  if Path::new(&sea).exists() {
    let file = File::open(sea).unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
      let line = line.unwrap();
      number.push(line);
    }

    for i in number {
      if count == 0 {
      } else {
        for j in i.split("|") {
          numb.push(j.to_string());
        }
        if userinput == "Party".to_ascii_lowercase() {
          println!("{}", numb[0]);
          numb = vec![];
        } else if userinput == "Candidate".to_ascii_lowercase() {
          println!("{}", numb[2]);
          numb = vec![];
        } else if userinput == "Winner".to_ascii_lowercase() {
          println!("{}", numb[4]);
          numb = vec![];
        }
      }
      count += 1;
    }  }
}
