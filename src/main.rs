use encoding_rs::WINDOWS_1252;
use std::{env::args, fs::File, io::{BufRead, BufReader, Read, Seek, Write}, time::Instant};
use std::io::SeekFrom;

#[allow(non_upper_case_globals)]
const magic_size: usize = 30*30*30;
fn main() {
    let path = args().nth(1).unwrap();

    // This is for generating index file
    // generate_compact_index(path);

    // This is for generating magic file
    // generate_magic_file(path);

    // These are for lookup
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

#[allow(dead_code)]
fn generate_compact_index(token_path: String) {
    let time = Instant::now();
    let token = File::open(token_path).expect("File not found");
    let mut token = BufReader::new(token);
    let mut word: Vec<u8> = Vec::with_capacity(40);
    let mut prev_word: Vec<u8> = Vec::with_capacity(40);
    let mut byte_offset: Vec<u8> = Vec::with_capacity(40);
    //let mut output_buffer: Vec<u8> = Vec::with_capacity(150_000_000);
    let mut output = File::create("index.txt").expect("oh no");
    // check if the word matches anything
    //println!("Finding word in index file");
    loop {
        if let Ok(some) =  token.read_until(b' ', &mut word) {
            if some == 0 {
                break;
            }

        
        word.pop();

        if word == prev_word{
            output.write(&[b' ']).expect("byte write fail");
            
        } else {
            output.write(&[b'\n']).expect("byte write fail");
            output.write_all(&word).expect("byte write fail");
            output.write(&[b' ']).expect("byte write fail");
            prev_word = word.clone();
        }

        token.read_until(b'\n', &mut byte_offset).expect("byte write fail");
        byte_offset.pop();
        output.write_all(&byte_offset).expect("byte write fail");
        word.clear();
        byte_offset.clear();
    } else {
        break;
    }
    }
    output.flush().expect("flushingggggg");
    println!("Took: {:?}s", time.elapsed().as_secs());

}
#[allow(dead_code)]
fn generate_magic_file(index_path: String) {
    let mut index = File::open(index_path).expect("File not found");
    let mut buf: Vec<u8> = Vec::with_capacity(150_000_000);
    index.read_to_end(&mut buf).expect("brr");
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
#[allow(dead_code)]
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

#[allow(dead_code)]
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
    let index_offset = magic_file[hash(&prefix)];
    if index_offset != 0 {
        let index_file = File::open("index.txt").expect("No index file found!");
        let mut index_file = BufReader::new(index_file);
        let mut read_word: Vec<u8> = Vec::with_capacity(40);
        let mut _current = index_file.seek(SeekFrom::Start(index_offset as u64)).expect("nono");
        let mut indices: Vec<u64> = Vec::with_capacity(100000);
        // check if the word matches anything
        loop {
            index_file.read_until(b' ', &mut read_word).expect("brr");
            read_word.pop();
            //println!("Looking for: {:?} Currently at: {:?}", word, read_word);
            
            if word.to_vec() == read_word {
                // get all indices
                let mut buf: String = String::with_capacity(1000);
                index_file.read_line(&mut buf).expect("brr");
                indices = buf.split_whitespace().map(|i| i.parse::<u64>().unwrap()).collect();
                break;
            } else {
                if hash(&read_word) != hash(&prefix) {
                    break;
                }
                // go to next line
                index_file.read_until(b'\n', &mut read_word).expect("brr");
                read_word.clear();
                index_file.seek(SeekFrom::Current(0)).expect("brr");
            }
        }
        let mut korpus = File::open("korpus").expect("Korpus need to be in the same directory");
        let mut buf = [0; 60];
        // print at indices
        println!("Found {:?} occurances", indices.len());
        for i in 0..indices.len() {
            if i%30 == 0 {
                println!("Show next 30? y/n");
                let mut response: String = String::new();
                std::io::stdin().read_line(&mut response).expect("Wrong input");
                let _trim = response.trim();
                if response.contains("y") || response.contains("Y") {
                } else {
                    println!("Aborting ...");
                    break;
                }
            }
            korpus.seek(SeekFrom::Start(indices[i] - 30)).expect("msg");
            korpus.read_exact(&mut buf).expect("msg");
            let decoded = WINDOWS_1252.decode(&buf).0.replace('\n',  " ");
            println!("...{}...", decoded);
        }
        
    } else {
        println!("No word found");
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;
    #[test]
    fn magic_time() {
        let timer = Instant::now();
        let _magic = load_magic_file();
        println!("Done! {:?}ms", timer.elapsed().as_millis());
        //print!("{:?}", magic.iter());
    }
}
