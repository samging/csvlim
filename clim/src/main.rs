use std::fs::{self, File, OpenOptions};
use regex::Regex;
use std::io::{self, Read, Write, SeekFrom, Seek, ErrorKind, Error};
use std::path::{absolute, Path, PathBuf};
use std::collections::BTreeMap;
use serde_json::to_string_pretty;
use rustybar::ProgressBar;
use rustybar::FillStyle;
use rustybar::EmptyStyle;



const NAME_KEY_STORE: &'static str = "KEY_SAVE.txt"; //find indexes

const NAME_KEY_STORE_REBUILD: &'static str = "KEY_SAVE_REBUILD.txt";
static FILE_PATH_NAME: &'static str = "keep.txt"; //keep track where we ended

static FILE_BUBBLE_REBUILD: &'static str = "bubbles.txt"; //keep track where we ended
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
    //print!("{:?}", bb[0]);
    Ok(bb)
}
fn formatting_to_json(ch: &[u8], state: &mut u64, seq_len: &mut u64, total: &mut i64, vasm: &mut Vec<String>, key: &mut u64) -> Result<(), Box<dyn std::error::Error>> {

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

    //print!("{}", tostr(ch)?);

    //if ch[0] == 10 { println!("") ;}
    //println!("{}", state) ;


    match state {
            0 => if ch[0] == 10 {
                *total = *total + 1;
                *state = 1;
            },
            1 =>  {
                *total = *total + 1;
                *state = 2;
            },

            2 => {
                *seq_len = *seq_len + 1;
                *total = *total + 1;
                //print!("(({}))-> [{}:{}]ASCII:{} of {:?} --[S] {}\n",key,seq_len,state,ch[0],tostr(ch)?,total);
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

//bp3
fn read_file_by_limit(stream: &[u8],
                      buffer_limit: u64,
                      iteration: u64,
                      state: &mut u64,
                      seq_len: &mut u64,
                      total: &mut i64,
                      vasm: &mut Vec<String>,
                    key: &mut u64, used_vasm: &mut String, finEd: &mut Vec<String>, fs_iteration: u64, fs_size: u64) -> Result<(), io::Error> {
    /*
     * reads file with set limit
     */

    //let mut buff: Vec<u8> = Vec::new();
    //let mut file_object = File::open(f)?;

    //let handle_name = Path::new(NAME_KEY_STORE);
    //let (beg, end): (u64, u64) = compute_buffer_size(Some(handle_name), 1)?;

    //println!("[read_file_by_limit](seek): IT{} {:?} {} {} {}", iteration, stream, total, vasm.len(), key); dbg
    /*
    let mut state: u64 = 2;
    let mut seq_len: u64 = 0;
    let mut total: i64 = -1;
    let mut key: u64 = 0;*/
    //let metadata: u64 = file_name.expect("REASON").metadata()?.len();

    //println!("METADATA: {}", metadata);

    //bp2
    formatting_to_json(stream, state, seq_len, total, vasm, key);

    //let mut finEd: Vec<String> = Vec::new();
    fn rest(vasm: &mut Vec<String>, finEd: &mut Vec<String>, used: &mut String, fs_iteration: u64, fs_size: u64) -> Result<(), Box<dyn std::error::Error>>{
        //vasm.push(format!("{}", seq_len));
        //println!("\n\n\nAssembled Tree: {:?}", vasm);

        //finEd.push("{\n".to_string());
        //finEd.push(format!("{:?}: {:?},\n", vasm[0], vasm[1].parse::<u64>().unwrap_or(0)));


        /*for slider in vasm[2..vasm.len() - 2].chunks_exact(2) {
            let current = &slider[0];
            let next: u64 = slider[1].parse().unwrap_or(0);
            finEd.push(format!("{:?}: {:?},\n", current, next));
        }*/

        //println!("VASM LEN {} to {} used: {}", vasm.len() - 2, vasm.len(), used); dbg


        for slider in vasm[vasm.len() - 2..vasm.len()].chunks_exact(2) {
            let current = &slider[0];
            let next: u64 = slider[1].parse().unwrap_or(0);

            //let next: &String= &slider[1];

            if *current == *used {
                //println!("Identical!!!"); dbg
            } else {
                //*used = current.clone();
                //finEd.push(format!("{:?}: {:?},\n", current, next));
                //(*used = current.clone();


                finEd.push(format!("{:?}: {:?},\n", current, next));
                *used = current.clone();
            }
            //finEd.push(format!("{:?}: {:?}\n}}", current, next));
        }

        if finEd.len() > 0 {
            //print!("\n{:?} {:?}", finEd, vasm[vasm.len()-1]);
            //print!("\n{:?}", finEd.last().ok_or("Didn't go well"));


            //chage to interation BP
            //println!("{} ==? {}", fs_iteration, fs_size - 1);

            //    println!("INL: {:?}", finEd);
            //let mut handle = File::open(NAME_KEY_STORE_REBUILD)?;
            let mut handle = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(NAME_KEY_STORE_REBUILD)?;



            let temp_string = finEd.join("  ");
            handle.write_all("  ".as_bytes())?;
            handle.write_all(temp_string.as_bytes())?;

            //let temp_string = finEd.join("  ");
            //handle.write_all(temp_string.as_bytes());
            finEd.clear();
            vasm.clear();

            /*
            if fs_iteration == (fs_size - 1){
                let mut handle = File::create(NAME_KEY_STORE_REBUILD)?;
                let temp_string = finEd.join("  ");
                handle.write_all("{\n  ".as_bytes());
                handle.write_all(temp_string.as_bytes());
                handle.write_all("}".as_bytes());
                //let formatted : String = finEd.join(">>>>>>"); println!("{}", formatted);
                println!("END");
            } */
        }

        //finEd.push("}".to_string());

        //println!("\n\n\n");
        //let result: Vec<String>= finEd.into_iter().map(|w: Vec<String>| w.join("  ")).collect();
        //let combined: String = finEd.join("  ");
        //print!("{:?}", finEd.join("  "));
        //print!("{:?}", combined);

        //let hh = File::create(NAME_KEY_STORE_REBUILD);
        //hh?.write_all(combined.as_bytes());

        //println!("\n\nWRITTEN COMBINED");

        //println!("\n");

        //let mut file_object2 = File::open(f)?;
        //(&mut file_object2).take(buffer_limit).read_to_end(&mut buff)?;
        Ok(())
    };
    if vasm.len() > 0 {
        if vasm.len() % 2 == 0 {
            rest(vasm, finEd, used_vasm, fs_iteration, fs_size);
        }
    }
    /*if finEd == 499_998{
        //print!("{:?}", finEd.into_iter().map(|w: Vec<String>| w.join("  ")));
        let formatted: String = finEd.join("  ");
        println!("{}", formatted);
    }*/

    Ok(())
}

fn compute_buffer_size(file_name: Option<&Path>, from_line: u64, to_line: u64) -> Result<(u64, u64), io::Error> {
    match file_name {
        Some(fd) => {
            //println!("[Compute_buffer_size] file_name: {:?}", file_name);
            //let mut file_open = File::open(fd)?;
            let mut file_open = File::open(NAME_KEY_STORE_REBUILD)?;

            let mut cursor: u64 = 0;
            let mut buff = [0u8; 1];
            let _longterm: Vec<u8> = Vec::with_capacity(1000);

            let mut pattern_first = vec![32, 32, 34];
            pattern_first.extend(from_line.to_string().bytes());
            pattern_first.extend(&[34,58,32]);

            let mut pattern_second = vec![32, 32, 34];
            pattern_second.extend((to_line).to_string().bytes());
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
            //println!("Ranges: [{} -> {}] = {}Bytes", offset_one, offset_two, buffer_size);

            Ok((offset_one, offset_two))
        }

        None => {
            println!("Nothing to read from ..");
            Err(Error::new(ErrorKind::NotFound, "No file path provided"))
        }
    }
}

fn note_keeper(file_opening: Option<&PathBuf>) -> Result<(), io::Error>{

    /*let abs_path = absolute(Path::new(FILE_PATH_NAME))?;
    let mut file_reading = File::create(abs_path)?;
    let line_bytes = desired_line.unwrap_or(0).to_le_bytes();

    (&mut file_reading).write_all(&line_bytes)?;
    (&mut file_reading).write_all(file_opening.to_string_lossy().as_bytes())?;
    (&mut file_reading).write_all(content.as_bytes())?;*/

    //syncing?
    //let mut len_file = File::create(file_opening.is_some())?.metadata()?.len();
    let path = file_opening.as_deref().unwrap();
    //println!("path: {:?}", path.file_name());
    let metadata: u64 = Path::new(path).metadata()?.len();
    //println!("Size is: {}", metadata);
    let mut file_note = File::create(FILE_PATH_NAME)?;
    file_note.write_all(format!("{:?} {}",path.file_name().unwrap(), &metadata).as_bytes())?;

    Ok(())
}

fn read_file(file_p: Option<&PathBuf>) -> Result<bool, io::Error> { //bp6 needs keep.txt, so just source it...
    let content = fs::read_to_string(FILE_PATH_NAME).unwrap();
    let reg = Regex::new(r#"(\w+\.\w+)"\s(\d+)"#).unwrap();

    let Some(trycapt) = reg.captures(&content) else {
        println!("no matches for name!");
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no matches"));
    };

    //println!("Regex groups: {:?}\n read lines: {:?}", &trycapt[1], &trycapt[2]);
    let path_object = Path::new(file_p.unwrap());

    let metadata: u64 = path_object.metadata()?.len();
    let file_name: String = path_object.file_name().and_then(|name| name.to_str()).map(|s| s.to_string()).unwrap();

    let captured_bytes: u64 = trycapt[2].parse::<u64>().unwrap();
    let captured_name: String = trycapt[1].to_string();

    if captured_bytes == metadata && captured_name == file_name {
        //println!("(C) -> [Read: {} to Given: {}]\t[Read: {} to Given: {}]", captured_bytes, metadata, captured_name, file_name);
        Ok::<bool, io::Error>(false)
    } else {
        //println!("(!) -> [Read: {} to Given: {}]\t[Read: {} to Given: {}]", captured_bytes, metadata, captured_name, file_name);
        Ok::<bool, io::Error>(true)
    }
} //bpp

pub fn walk_for_index(data: &[u8], buffer_limit: usize, index_to_walk_on: u64) -> Result<Option<u64>, Box<dyn std::error::Error>> {
    let mut b = Vec::new();

    let mut file_check = match File::open(NAME_KEY_STORE) {
        Ok(file) => file,
        Err(_) => File::create(NAME_KEY_STORE)?,
    };

    let mut content = String::with_capacity(buffer_limit);
    (&mut file_check).take(buffer_limit as u64).read_to_string(&mut content)?;

    //println!("{:#?}", content);

    //println!("{:?}", data);

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
    let bulk_closure = || ->  Result<(), io::Error> {
        print!("Please provide file name: ");
        io::stdout().flush()?;
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

        //this sorta is the new update:

        /*let char_stream_closure = |formatted: &PathBuf, read_limit_TF: &mut Vec<String>| -> Result<(), io::Error>{
            let mut file_object_char = File::open(formatted)?;

            let metadata: u64 = Path::new(formatted).metadata()?.len(); //bp
            println!("METADATA: {}", metadata);


            for i in 0..metadata {
                let ch = reading_by_character(&mut file_object_char, i)?;
                println!("{}", i);
                read_file_by_limit(&ch, 1, i, read_limit_TF);
                //print!("{:?}", &ch);
                //read_file_by_limit(Some(&formatted_path), 1000);
                //formatting_to_json(&ch,  &mut state, &mut seq_len, &mut total, &mut vasm, &mut key);
            }
            Ok(())
        };*/


        pub fn create_bubble_file2(
            char_slice: &[u8],      // renamed from `char` to avoid reserving the keyword
            comma_count: u8,
            current_comma: &mut u64,
            curr: &mut u64,         // Added to track absolute character index like Python's `curr`
            file_size: u64,         // Acts as Python's `MA` (len of total input)
        ) -> Result<(), std::io::Error> {

            // Open in append mode so we don't wipe the file on every single character iteration
            let mut bubble_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("bubbled.csv")?;

            let ascii_str = std::str::from_utf8(char_slice).unwrap_or("");
            let is_comma = char_slice[0] == 44; // ',' ASCII value
            let is_newline = char_slice[0] == 10; // '\n' ASCII value

            let target_comma_count = comma_count as u64;
            let min_index: u64 = 0; // Python's MI

            // Match against the absolute character position `curr`
            match *curr {
                // case x if x == MI:
                x if x == min_index => {
                    bubble_file.write_all(ascii_str.as_bytes())?;
                    *curr += 1;
                }

                // case x if x in generated: (matches any valid index within file_size)
                x if x < file_size => {
                    if is_comma {
                        bubble_file.write_all(b",")?;

                        // if current_comma != comma_count - 1:
                        if *current_comma != (target_comma_count - 1) {
                            // print 3 spaces
                            bubble_file.write_all(b"   ")?;
                        }

                        *curr += 1;
                        *current_comma += 1;

                        // if current_comma == comma_count:
                        if *current_comma == target_comma_count {
                            *current_comma = 0;
                        }
                    } else {
                        bubble_file.write_all(ascii_str.as_bytes())?;
                        *curr += 1;
                    }
                }

                _ => panic!("Index out of bounds or file_size exceeded!"),
            }

            // if char == "\n":
            if is_newline {
                // Optional tracking equivalent to Python's print("RESTARTED")
                println!("RESTARTED");
            }

            Ok(())
        }

        pub fn char_stream_closure(
            formatted: &PathBuf, rebuffering: bool
        ) -> Result<(), io::Error> {
            let mut file_object_char = File::open(formatted)?;
            let mut vv: Vec<String> = Vec::new();
            let mut state: u64 = 2;
            let mut seq_len: u64 = 0;
            let mut total: i64 = -1;
            let mut key: u64 = 0;
            let mut used_vasm = String::new();
            let mut finEd: Vec<String> = Vec::new();

            let metadata: u64 = Path::new(formatted).metadata()?.len();
            //println!("METADATA: {}", metadata);

            if rebuffering {
                let mut file = File::create(NAME_KEY_STORE_REBUILD)?;
                file.flush()?;


                let mut bubble_file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(FILE_BUBBLE_REBUILD).expect("CRASHED: Could not create the file!");

                let mut bubble_file_csv = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open("bubbled.csv")
                    .expect("CRASHED: Could not create the file!");

                let mut bar = ProgressBar::new("[Rebuffering]", 40, metadata.try_into().unwrap());
                bar.style(FillStyle::Solid, EmptyStyle::Dash);

                let mut id_counter: u8 = 0;
                let mut should_increment: bool = true;
                let mut remember_headers: Vec<u64> = Vec::new();
                let mut current_comma: u64 = 0;
                let mut curr: u64 = 0;

                for i in 0..metadata {
                    bar.tick((i + 1).try_into().unwrap());

                    let ch = reading_by_character(&mut file_object_char, i)?;

                    //if is set to false, then it read the first line of headers
                    if should_increment == false {
                        create_bubble_file2(&ch, id_counter, &mut current_comma, &mut curr, metadata);
                    }
                    //println!("{} {}", ch[0], std::str::from_utf8(&ch).map_err(|_|{ std::io::Error::new(std::io::ErrorKind::Other, "rx -> ry boundary problem".to_string()) })?);

                    if ch[0] == 44 {
                        // println!("read , {}",i);
                        if should_increment == true{
                            id_counter = id_counter + 1;
                            remember_headers.push(i);
                        }
                        bubble_file
                            .write_all(format!("{},", i).as_bytes())
                            .expect("FAILED TO WRITE TO BUBBLE FILE");
                    }


                    if ch[0] == 10 && should_increment {
                        println!("Read NewLine: {}", id_counter);
                        should_increment = false;
                    }

                    if should_increment == false {
                        for headers in 0..id_counter as usize{
                            print!("{} ", remember_headers[headers]);
                        }
                        //index * header = value (this is how find will work)
                        println!("Wished for header index 1: [{} {}]", remember_headers[0], remember_headers[1]);
                    }



                    read_file_by_limit(&ch, 1, i, &mut state, &mut seq_len, &mut total, &mut vv, &mut key, &mut used_vasm, &mut finEd,i, metadata);
                }
                println!("");
            }
            //println!("{:?}", vv);
            let helper_func = || -> io::Result<(u64, u64)> {
                print!("Read from line: ");
                io::stdout().flush()?;

                let mut cin_from = String::with_capacity(10);
                let stdin = io::stdin();
                stdin.read_line(&mut cin_from)?;

                print!("Read to line: ");
                io::stdout().flush()?;

                let mut cin_to = String::with_capacity(10);
                let stdin = io::stdin();
                stdin.read_line(&mut cin_to)?;

                let number_from: u64 = cin_from.trim().parse().unwrap();
                let number_to: u64 = cin_to.trim().parse().unwrap();

                let (rx, ry) = compute_buffer_size(Some(Path::new(NAME_KEY_STORE_REBUILD)), number_from, number_to)?;
                //println!("{} {}", rx, ry);
                Ok((rx, ry))
            };

            //let (rx, ry) = compute_buffer_size(Some(Path::new(NAME_KEY_STORE_REBUILD)), number_from, number_to)?;
            let (rx,ry) = helper_func()?;

            for i in rx..ry {
                /* print!("{}", std::str::from_utf8(&reading_by_character(&mut file_object_char, i)?).map_err(|_|{
                    std::io::Error::new(std::io::ErrorKind::Other, "rx -> ry boundary problem".to_string())
                })?); */

                let raw_bytes = reading_by_character(&mut file_object_char, i)?;


                let char = std::str::from_utf8(&raw_bytes).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "rx -> ry boundary problem".to_string())
                })?;

                if char == "," {
                    print!(" | ");
                } else {
                    print!("{}", &char);
                }
            }

            //println!("[{rx} -> {ry}] <<< COMPUTED");
            //println!("<<<<< HERE (single vector value)");
            Ok(())
        }

        //[] read_file_by_limit(Some(&formatted_path), 1000); //now find with that file...
            //let (f, t) = compute_buffer_size(Some(Path::new(NAME_KEY_STORE_REBUILD)), 4000, 500000)?;
            /*
            println!("FINAL READINGS: ");

            for ix in f..t {
                let ch = reading_by_character(&mut file_object_char, ix)?;

                let get_slice = std::str::from_utf8(&ch).map_err(|_| {
                    Error::new(ErrorKind::InvalidData, "-||-")
                })?;
                print!("{}", get_slice);
            }
            println!("\n\n");
            Ok(())
        };
        println!("--- CALLING CHAR STREAMING: ---");
        */
        match read_file(Some(&formatted_path))?{
            true => {
                //println!("Needs to re-read");
                char_stream_closure(&formatted_path, true);
            },
            false => {
                //println!("validation is alright");
                char_stream_closure(&formatted_path, false);
            },
            _ => {}
        }

        note_keeper(Some(&formatted_path));
        //println!("{:?}",vv);
        Ok(())
    };

    bulk_closure(); // pass variables to read lim







    /* let bulk_closure = || ->  Result<(), io::Error> {
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
        //this sorta is the new update:
        let char_stream_closure = |formatted: &PathBuf| -> Result<(), io::Error>{
            let mut file_object_char = File::open(&formatted_path)?;

            let metadata: u64 = Path::new(&formatted_path).metadata()?.len(); //bp
            println!("METADATA: {}", metadata);

            /* []
            for i in 0..metadata {
                let ch = reading_by_character(&mut file_object_char, i)?;
                //print!("{:?}", &ch);
                //read_file_by_limit(Some(&formatted_path), 1000);
                //formatting_to_json(&ch,  &mut state, &mut seq_len, &mut total, &mut vasm, &mut key);
            } */

            //[] read_file_by_limit(Some(&formatted_path), 1000); //now find with that file...
            let (f, t) = compute_buffer_size(Some(Path::new(NAME_KEY_STORE_REBUILD)), 4000, 500000)?;

            println!("FINAL READINGS: ");

            for ix in f..t {
                let ch = reading_by_character(&mut file_object_char, ix)?;

                let get_slice = std::str::from_utf8(&ch).map_err(|_| {
                    Error::new(ErrorKind::InvalidData, "-||-")
                })?;
                print!("{}", get_slice);
            }
            println!("\n\n");
            Ok(())
        };
        println!("--- CALLING CHAR STREAMING: ---");

        char_stream_closure(&formatted_path);
        Ok(())
    };

    println!("callling bluk closure");
    bulk_closure();*/


    /*let get_user_run_save = || -> Result<String, io::Error> {
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
        println!("Reading by characters: ");


        //BYTE READING (impl):
        match note_keeper(None, &formatted_path, String::from("something that should be a String type")) {
                Ok(_) => match read_file_by_limit(Some(&formatted_path), 1000) {

                    Ok(limit_buff) => {
                        //BYTE READING (impl) :

                        let res: u64 = walk_for_index(&limit_buff, 1000, 9)
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

    };*/

    //get_user_run_save().unwrap();
}
