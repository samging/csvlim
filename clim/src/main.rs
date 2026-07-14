use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{absolute, Path, PathBuf};
use std::iter::IntoIterator;
use std::collections::BTreeMap;
use serde_json::to_string_pretty;

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

fn read_file_by_limit(file_name: Option<&PathBuf>, buffer_limit: u64) -> Result<Vec<u8>, io::Error> {
    /*
     * reads file with set limit
     */

    match file_name {
        Some(f) => {
            println!("File Object {:#?}", file_name.unwrap().file_name());
            let mut buff: Vec<u8> = Vec::new();

            let mut file_object = File::open(f)?;
            (&mut file_object).take(buffer_limit).read_to_end(&mut buff)?;
            Ok(buff)
        }
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "No file provided"))
    }
}

fn note_keeper(desired_line: Option<u64>, file_opening: &PathBuf, content: String) -> Result<(), io::Error>{
    static FILE_PATH_NAME: &'static str = "keep.txt";

    let abs_path = absolute(Path::new(FILE_PATH_NAME))?;
    let mut file_reading = File::create(abs_path)?;
    let line_bytes = desired_line.unwrap_or(0).to_le_bytes();

    (&mut file_reading).write_all(&line_bytes)?;
    (&mut file_reading).write_all(file_opening.to_string_lossy().as_bytes())?;
    (&mut file_reading).write_all(content.as_bytes())?;
    Ok(())
}

pub fn walk_for_index(data: &[u8], buffer_limit: usize, index_to_walk_on: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut b = Vec::new();

    const NAME_KEY_STORE: &'static str = "KEY_SAVE.txt";
    let mut file_check = match File::open(NAME_KEY_STORE) {
        Ok(file) => file,
        Err(_) => File::create(NAME_KEY_STORE)?,
    };

    let mut content = String::with_capacity(buffer_limit);
    (&mut file_check).take(buffer_limit as u64).read_to_string(&mut content)?;

    println!("HERE {:#?}", content);

    let ordered_map: BTreeMap<u64, u64> = serde_json::from_str(&content).unwrap_or_else(|_| BTreeMap::new()); //where it stopped

    println!("Parsed Map (serde): {:#?}", ordered_map); //this does not print
    println!("index_to_walk_on: {}", index_to_walk_on); //this does not print

    match ordered_map.get(&index_to_walk_on){
        Some(v) => {
            println!("walked on:'{}' found: {}",index_to_walk_on, v);
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

    Ok(())
}

fn main() {
    let mut get_user_run_save = || -> Result<String, io::Error> {
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

        match note_keeper(None, &formatted_path, String::from("something that should be a String type")) {
                Ok(_) => match read_file_by_limit(Some(&formatted_path), 1000) {

                    Ok(limit_buff) => {
                        walk_for_index(&limit_buff, 500, 3);

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

    let res = get_user_run_save().unwrap();
}
