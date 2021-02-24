use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};

const magic_size: usize = 30*30*30;
fn main() {
    let path = args().nth(1).unwrap();
    //generate_compact_index(path);
    generate_magic_file(path);
}

fn hash(key: &Vec<u8>) -> usize {
    let mut index = 0;
    
    for i in 0..key.len() {
        let val = match key[i] {
            228 => index += 27 * 30usize.pow(i as u32), //ä
            229 => index += 28 * 30usize.pow(i as u32), //å
            246 => index += 29 * 30usize.pow(i as u32), //ö
            c => index += (c - 96) as usize * 30usize.pow(i as u32)
        };
    }
    index
}

fn generate_compact_index(token_path: String) {
    let token = File::open(token_path).expect("File not found");
    let buffered = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(token),
    );
    let mut prev_word = "".to_string();
    let lines= buffered.lines().map(|l| l.unwrap());
    let mut output_buffer = String::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let word = split.next().unwrap();
        let pointer = split.next().unwrap();
        if prev_word == "" {
            prev_word = word.to_string();
            output_buffer.push_str(word);
            output_buffer.push(' ');
            output_buffer.push_str(pointer);
        } else if word == prev_word {
            output_buffer.push(' ');
            output_buffer.push_str(pointer);
        } else {
            prev_word = word.to_string();
            output_buffer.push('\n');
            output_buffer.push_str(word);
            output_buffer.push(' ');
            output_buffer.push_str(pointer);
        }
    }
    let output = WINDOWS_1252.encode(&output_buffer).0;
    std::fs::write("index.txt", output).expect("brr");
}

fn generate_magic_file(index_path: String) {
    let index = File::open(index_path).expect("File not found");
    let mut prefix: Vec<u8> = Vec::with_capacity(3);
    let mut array: [usize;magic_size]  = [0; magic_size];
    let mut is_word = true;
    for (i, res) in index.bytes().enumerate() {
        if let Ok(b) = res {
            match b {
                b' ' => {
                    if is_word {
                    is_word = false;
                    let index = hash(&prefix);
                    array[index] = i - prefix.len(); 
                    }
                },
                b'\n' => {
                    is_word = true;
                    prefix.clear()
                },
                b => {
                    if is_word && prefix.len() < 3 {
                        prefix.push(b);
                    }
                }
            }
        } else {
            break
        }
    }
    let mut output = File::create("magic-file.txt").expect("oh no");
    for i in array.iter() {
        output.write_all(i.to_string().as_bytes()).expect("frick");
        output.write(&[b'\n']).expect("ohno");

    }
    output.flush();
}

fn load_magic_file() -> [usize; magic_size] {
    let mut array = [0; magic_size];
    let mut magic_file = File::open("magic-file.txt").expect("yikes");
    let mut buffered = String::new();
    magic_file.read_to_string(&mut buffered).expect("oh gosh");
    for (i, line) in buffered.lines().enumerate() {
        if let Ok(num) = line.parse::<usize>()  {
            array[i] = num;
        }
        
    }
    return array;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn magic() {
        let magic = load_magic_file();
        print!("{:?}", magic.iter());
    }
}
