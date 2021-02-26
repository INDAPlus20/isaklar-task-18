use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{env::args, fs::File, io::{BufRead, BufReader, Read, Seek, Write}};
use std::io::SeekFrom;

const magic_size: usize = 30*30*30;
fn main() {
    let path = args().nth(1).unwrap();
    //generate_compact_index(path);
    //generate_magic_file(path);
    let input = WINDOWS_1252.encode(&path).0;
    find(&input[0..]);
}
// assumes length 3
fn hash(key: &Vec<u8>) -> usize {
    let mut index = 0;
    
    for i in 0..key.len().min(3) {
        match key[i] {
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
    let mut index = File::open(index_path).expect("File not found");
    let mut buf: Vec<u8> = Vec::with_capacity(150_000_000);
    index.read_to_end(&mut buf);
    let mut prefix: Vec<u8> = Vec::with_capacity(40);
    let mut array: [u64;magic_size]  = [0; magic_size];
    let mut is_word = true;
    for i in 0..buf.len() {
        let b = buf[i];
            match b {
                b' ' => {
                    if is_word {
                    is_word = false;
                    
                    let index = hash(&prefix);
                    if array[index] == 0 {
                        array[index] = i as u64 - prefix.len() as u64; 
                    }
                    
                    }
                },
                b'\n' => {
                    is_word = true;
                    prefix.clear()
                },
                b => {
                    prefix.push(b);
                }
            }
    }
    let mut output = File::create("magic-file.txt").expect("oh no");
    for i in array.iter() {
        output.write_all(i.to_string().as_bytes()).expect("frick");
        output.write(&[b'\n']).expect("ohno");

    }
    output.flush().expect("maybe this?");
}

fn load_magic_file() -> [u64; magic_size] {
    let mut array = [0; magic_size];
    let mut magic_file = File::open("magic-file.txt").expect("yikes");
    let mut buffered = String::new();
    magic_file.read_to_string(&mut buffered).expect("oh gosh");
    for (i, line) in buffered.lines().enumerate() {
        if let Ok(num) = line.parse::<u64>()  {
            array[i] = num;
        }
        
    }
    return array;
}

// Assumes encoded byte-array
fn find(word: &[u8]) {
    let magic_file = load_magic_file();
    let mut prefix: Vec<u8> = Vec::with_capacity(3);
    for b in word {
        if prefix.len() == 3 {
            break;
        }
        prefix.push(*b);
    }
    let mut index_offset = magic_file[hash(&prefix)];
    println!("Index offset: {:?}", index_offset);
    let mut index_file = File::open("index.txt").expect("No index file found!");
    let mut index_file = BufReader::new(index_file);
    let mut read_word: Vec<u8> = Vec::with_capacity(40);
    let mut current = index_file.seek(SeekFrom::Start(index_offset as u64)).expect("nono");
    let mut indices: Vec<u64> = Vec::with_capacity(100000);
    // check if the word matches anything
    println!("Finding word in index file");
    loop {
        index_file.read_until(b' ', &mut read_word);
        read_word.pop();
        //println!("Looking for: {:?} Currently at: {:?}", word, read_word);
        
        if word.to_vec() == read_word {
            // get all indices
            let mut buf: String = String::with_capacity(1000);
            index_file.read_line(&mut buf);
            indices = buf.split_whitespace().map(|i| i.parse::<u64>().unwrap()).collect();
            break;
        } else {
            // go to next line
            index_file.read_until(b'\n', &mut read_word);
            read_word.clear();
            index_file.seek(SeekFrom::Current(0));
        }
    }
    println!("Done!");
    let mut korpus = File::open("korpus/korpus").expect("Korpus where?");
    let bytes = 30;
    let mut buf = [0; 60];
    // print at indices
    println!("Found {:?} occurances", indices.len());
    for index in indices {
        korpus.seek(SeekFrom::Start(index - 30));
        korpus.read_exact(&mut buf);
        for b in buf.iter() {

        }
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;
    #[test]
    fn magic_time() {
        let timer = Instant::now();
        let magic = load_magic_file();
        println!("Done! {:?}ms", timer.elapsed().as_millis());
        //print!("{:?}", magic.iter());
    }
}
