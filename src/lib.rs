use std::io::{File, Open, Write, IoResult, BufferedReader};
use std::collections::HashMap;

fn main() {
  let mut word_counts = HashMap::new();
  let mut f = File::open(&Path::new("10.txt"));
  let mut reader = BufferedReader::new(f);

  for line in reader.lines() {
    let line_str = line.to_string();
    let mut words = line_str.split_str(" ");
    for word in words {
      let word_str = word.to_string();
      let mut count = 0;
      if word_counts.contains_key(word) {
        let value:int = *(word_counts.get(word).unwrap());
        count = value + 1i;
      }else{
        count = 1i
      }
      word_counts.insert(word_str, count);
    }
  }
  let fname = "out.txt";
  let p = Path::new(fname);
  let mut f = File::open_mode(&p, Open, Write);
  f.write_line("word \t\t count:");
  for (word, count) in word_counts.iter() {
    let a = format!("{} \t\t {}", word, count);
    f.write_line(a.as_slice());
  }
}

#[test]
fn is_one_equal_to_one() {
    assert_eq!(1i, 1i);
}

#[test]
fn it_works() {
}
