use std::fs::{self, File};
use std::io::{self, Read, Write, SeekFrom, Seek, ErrorKind, Error};
use std::path::{absolute, Path, PathBuf};
use std::collections::BTreeMap;
use serde_json::to_string_pretty;



const NAME_KEY_STORE: &'static str = "KEY_SAVE.txt"; //find indexes

const NAME_KEY_STORE_REBUILD: &'static str = "KEY_SAVE_REBUILD.txt";
static FILE_PATH_NAME: &'static str = "keep.txt"; //keep track where we ended

pub struct Buffer<'a> {
    bytes: Vec<[&'a str; 100]>
}


impl<'a> Buffer<'a> {
    pub fn new() -> Self {
        Buffer {
            bytes: Vec::new()
        }
    }

    pub fn readwrite(&mut self, words: Option<&'a [u8]>) -> () {
        match words {
            Some(byte_slices) => {
                if let Ok(text) = std::str::from_utf8(byte_slices) {
                    let mut array: [&'a str; 100] = [""; 100];
                    for (i, word) in text.split_whitespace().take(100).enumerate(){
                        array[i] = word
                    }
                    self.bytes.push(array);

                    /*
                     for (i, bytes) in self.bytes.iter().enumerate() {
                            println!("{}\t{:#?}", i, bytes);
                        }

                     */
                } else {
                    println!("Utf-8 conversion failed");
                }
            },
            None => {
                println!("empty buffer, read the max");
            }
        }
    }
}

fn reading_by_character(file_name: &mut File, by_index: u64) -> Result<Vec<u8>, io::Error>{
    //println!("[reading_by_character] Seeking:");
    file_name.seek(SeekFrom::Start(by_index))?;
    let mut bb = vec![0u8;1];
    file_name.read_exact(&mut bb);
    //print!("{:?}", bb);
    Ok(bb)
}
fn formatting_to_json(ch: &[u8], state: &mut u64, seq_len: &mut u64, total: &mut u64, vasm: &mut Vec<String>, key: &mut u64) -> Result<(), Box<dyn std::error::Error>> {

    fn tostr(ch: &[u8]) -> Result<&str, std::io::Error> {
        std::str::from_utf8(ch).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Reading from utf8 failed"
            )
        })
    }

    // ',' 44
    // ' ' 32
    //print!("[{} {}]", tostr(ch)?, ch[0]);

    print!("{}", tostr(ch)?);

    //if ch[0] == 10 { println!("") ;}
    //println!("{}", state) ;


    match state {
            0 => if ch[0] == 10 {
                *state = 1;
            },
            1 =>  {
                *state = 2;
            },

            2 => {
                print!("(({}))-> [{}:{}]ASCII:{}\n",key,seq_len,state,ch[0]);
                *seq_len = *seq_len + 1;
                *total = *total + 1;

                if ch[0] == 10 {
                    *key = *key + 1;
                    //vasm.push(format!("{}:{}", key, seq_len));

                    vasm.push(format!("{}", key));
                    vasm.push(format!("{}",total));
                    *seq_len = 0;
                    *state = 1;
                }
            },
            _ => {}
        }
    Ok(())
}
fn read_file_by_limit(file_name: Option<&PathBuf>, buffer_limit: u64) -> Result<Vec<u8>, io::Error> {
    /*
     * reads file with set limit
     */

    match file_name {
        Some(f) => {

            println!("File Object {:#?}", file_name.unwrap().file_name());
            let mut buff: Vec<u8> = Vec::new();

            let mut file_object = File::open(f)?;

            let handle_name = Path::new(NAME_KEY_STORE);
            //let (beg, end): (u64, u64) = compute_buffer_size(Some(handle_name), 1)?;

            println!("[read_file_by_limit](seek): ");
            let mut state : u64= 2;
            let mut seq_len : u64 = 0;
            let mut total: u64 = 0;
            let mut key: u64 = 0;
            let mut vasm: Vec<String>= Vec::new();
            let metadata: u64 = file_name.expect("REASON").metadata()?.len();

            println!("METADATA: {}", metadata);
            for x in 0..metadata {

                formatting_to_json(&reading_by_character(&mut file_object, x)?, &mut state, &mut seq_len, &mut total, &mut vasm, &mut key);
                /*formatting_to_json(std::str::from_utf8(&reading_by_character(&mut file_object, x)?).map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData, "Reading from utf8 failed"
                    )
                })?);*/
            }

            let mut finEd: Vec<String> = Vec::new();
            //vasm.push(format!("{}", seq_len));
            println!("\n\n\nAssembled Tree: {:?}", vasm);

            finEd.push("{\n".to_string());
            finEd.push(format!("{:?}: {:?},\n",vasm[0], vasm[1].parse::<u64>().unwrap_or(0)));


            for slider in vasm[2..vasm.len()-2].chunks_exact(2) {
                let current = &slider[0];
                let next: u64 = slider[1].parse().unwrap_or(0);
                finEd.push(format!("{:?}: {:?},\n", current, next));
            }

            for slider in vasm[vasm.len()-2..vasm.len()].chunks_exact(2) {
                let current = &slider[0];
                let next: u64 = slider[1].parse().unwrap_or(0);
                finEd.push(format!("{:?}: {:?}\n}}", current, next));
            }
            //finEd.push("}".to_string());

            println!("\n\n\n");
            //let result: Vec<String>= finEd.into_iter().map(|w: Vec<String>| w.join("  ")).collect();
            let combined: String = finEd.join("  ");
            //print!("{:?}", finEd.join("  "));
            print!("{:?}", combined);

            let hh = File::create(NAME_KEY_STORE_REBUILD);
            hh?.write_all(combined.as_bytes());

            println!("\n\nWRITTEN COMBINED");

            println!("\n");

            (&mut file_object).take(buffer_limit).read_to_end(&mut buff)?;
            Ok(buff)
        }
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "No file provided"))
    }
}

fn compute_buffer_size(file_name: Option<&Path>, from_line: u64) -> Result<(u64,u64), io::Error> {
    match file_name {
        Some(fd) => {
            println!("[Compute_buffer_size] file_name: {:?}", file_name);
            let mut file_read = fs::read_to_string(fd)?;
            let mut file_open = walk_for_index(&file_read.into_bytes(), 1000 as usize, 10);//bp

            let mut file_open = File::open(fd)?;
            //let mut file_open = File::open(NAME_KEY_STORE_REBUILD)?;

            let mut cursor: u64 = 0;
            let mut buff = [0u8; 1];
            let _longterm: Vec<u8> = Vec::with_capacity(1000);

            let mut pattern_first = vec![32, 32, 34];
            pattern_first.extend(from_line.to_string().bytes());
            pattern_first.extend(&[34,58,32]);

            let mut pattern_second = vec![32, 32, 34];
            pattern_second.extend((from_line + 4).to_string().bytes());
            pattern_second.extend(&[34,58,32]);
            let mut number_one_R = Vec::new();
            let mut number_two_R = Vec::new();

            let mut state = 0;
            let mut pattern_idx = 0;

            loop {
                file_open.seek(SeekFrom::Start(cursor))?;

                match file_open.read_exact(&mut buff) {

                    Ok(_) => {
                        let byte = buff[0];
                            match state {
                                0 => {
                                    if byte == pattern_first[pattern_idx] {
                                        pattern_idx += 1;
                                        if pattern_idx == pattern_first.len() {
                                            state = 1;
                                            pattern_idx = 0;
                                        }
                                    } else {
                                        // Only reset if the current byte failed to match
                                        pattern_idx = if byte == pattern_first[0] { 1 } else { 0 };
                                    }
                                },
                                1 => {
                                    if byte == 44 || byte == 10 {
                                        state = 2;
                                    } else if byte.is_ascii_digit() {
                                        number_one_R.push(byte);
                                    }
                                },

                                2 => {
                                    if byte == pattern_second[pattern_idx] {
                                        pattern_idx += 1;
                                        if pattern_idx == pattern_second.len() {
                                            state = 3;
                                            pattern_idx = 0;
                                        }
                                    } else {
                                        pattern_idx = if byte == pattern_second[0] { 1 } else { 0 };
                                    }
                                }
                                3 => {
                                    if byte == 44 || byte == 10 {
                                        break;
                                    } else if byte.is_ascii_digit() {
                                        number_two_R.push(byte);
                                    }
                                }
                                _=> break,
                        }
                        cursor += 1;
                    }

                    Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                        return Err(Error::new(ErrorKind::NotFound, "Reached EOF without finding target byte"));
                    }
                    Err(e) => return Err(e),
                }
            }

            let str_one = std::str::from_utf8(&mut number_one_R).map_err(|_| {
                Error::new(ErrorKind::InvalidData, "Invalid UTF")
            })?;

            let str_two = std::str::from_utf8(&mut number_two_R).map_err(|_| {
                Error::new(ErrorKind::InvalidData, "Invalid UTF")
            })?;

            let offset_one: u64 = str_one.parse().map_err(|_| {
                Error::new(ErrorKind::InvalidData, "Can't parse to U64")
            })?;

            let offset_two: u64 = str_two.parse().map_err(|_| {
                Error::new(ErrorKind::InvalidData, "-||-")
            })?;

            let buffer_size: u64 = offset_two - offset_one;
            println!("Ranges: [{} -> {}] = {}Bytes", offset_one, offset_two, buffer_size);

            Ok((offset_one, offset_two))
        }

        None => {
            println!("Nothing to read from ..");
            Err(Error::new(ErrorKind::NotFound, "No file path provided"))
        }
    }
}

fn note_keeper(desired_line: Option<u64>, file_opening: &PathBuf, content: String) -> Result<(), io::Error>{

    let abs_path = absolute(Path::new(FILE_PATH_NAME))?;
    let mut file_reading = File::create(abs_path)?;
    let line_bytes = desired_line.unwrap_or(0).to_le_bytes();

    (&mut file_reading).write_all(&line_bytes)?;
    (&mut file_reading).write_all(file_opening.to_string_lossy().as_bytes())?;
    (&mut file_reading).write_all(content.as_bytes())?;
    Ok(())
}

pub fn walk_for_index(data: &[u8], buffer_limit: usize, index_to_walk_on: u64) -> Result<Option<u64>, Box<dyn std::error::Error>> {
    let mut b = Vec::new();

    let mut file_check = match File::open(NAME_KEY_STORE) {
        Ok(file) => file,
        Err(_) => File::create(NAME_KEY_STORE)?,
    };

    let mut content = String::with_capacity(buffer_limit);
    (&mut file_check).take(buffer_limit as u64).read_to_string(&mut content)?;

    println!("{:#?}", content);
    println!("<<<<<<<<<<<<<<<<<");

    let ordered_map: BTreeMap<u64, u64> = serde_json::from_str(&content).unwrap_or_else(|_| BTreeMap::new()); //where it stopped

    println!("Parsed Map (serde): {:#?}", ordered_map); //this does not print
    println!("index_to_walk_on: {}", index_to_walk_on); //this does not print

    match ordered_map.get(&index_to_walk_on){
        Some(&v) => {
            //println!("walked on:'{}' found: {}",index_to_walk_on, v);
            return Ok(Some(v))
        },
        None => {
            println!("CRPLE");
            println!("{:#?}", data);
            for (i, munch_byte) in data.iter().enumerate() {
                if *munch_byte == 10 {
                    b.push(i);
                } //if buffer is too small for reading to the '\n' then throw panic
            }
            let mut keys_hashed = BTreeMap::new();

            for (i, bt) in b.iter().enumerate() {
                keys_hashed.insert((i+1) as u64, bt);
            }

            let serialized = to_string_pretty(&keys_hashed)?;
            let mut key_store = File::create(NAME_KEY_STORE)?;
            key_store.write_all(&serialized.as_bytes())?;
            println!("Written");
        }
    }

    Ok(None)
}

fn main() {
    let get_user_run_save = || -> Result<String, io::Error> {
        let mut cin = String::with_capacity(100);
        let stdin = io::stdin();
        stdin.read_line(&mut cin)?;

        let cleaned_input = cin.trim_end().to_string();

        let abs_formatter = |c: String| -> Result<PathBuf, io::Error> {
            let file_object = Path::new(&c);
            if file_object.is_absolute() {
                Ok(file_object.to_path_buf())
            } else {
                absolute(file_object)
            }
        };

        let formatted_path = abs_formatter(cleaned_input)?;

        //BYTE READING (impl):
        match note_keeper(None, &formatted_path, String::from("something that should be a String type")) {
                Ok(_) => match read_file_by_limit(Some(&formatted_path), 1000) {

                    Ok(limit_buff) => {
                        //BYTE READING (impl):
                        let _res: u64 = walk_for_index(&limit_buff, 500, 3)
                            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
                            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Walk_for_index [ERR]"))?;


                        println!("COMPUTE BUFFER SIZE BEFORE");
                        let (sSeek, eSeek) = compute_buffer_size(Some(Path::new(NAME_KEY_STORE)), 3)?;
                        println!("COMPUTE BUFFER SIZE AFTER");
                        println!("Seeking from {} to {}", sSeek, eSeek);
                        //println!("{:?}",&limit_buff[res as usize..=res as usize +100].to_string());
                        let mut f = File::open(&formatted_path)?;

                        for S in sSeek..eSeek {
                            f.seek(SeekFrom::Start(S))?;
                            let mut buf= [0u8;1];
                            f.read_exact(&mut buf)?;

                            let utf_character = std::str::from_utf8(&buf).map_err(|_|{
                                std::io::Error::new(
                                        std::io::ErrorKind::InvalidData,
                                        format!("Seeking conversion failed at byte position {}", S),
                                )
                            })?;
                            print!("{}", utf_character);
                        }

                        //let slice: &[u8] = &limit_buff[res as usize..=res as usize + 100];

                        //let stringish = std::str::from_utf8(slice)
                            //.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

                        //println!("{:?}",stringish);
                        let mut Inst: Buffer = Buffer::new();

                       Inst.readwrite(Some(&limit_buff));
                        Ok("Ran alright!".to_string())
                   } Err(e) => {
                        eprintln!("Ran into: e{}", &e);
                        Err(e)
                    }
                }
                Err(e) => {
                    eprintln!("note_keeper failed: {}", e);
                    panic!("note_keeper failed!");
                }
            }

    };

    get_user_run_save().unwrap();
}
