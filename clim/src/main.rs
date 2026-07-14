use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{absolute, Path, PathBuf};

pub struct Buffer<'a> {
    bytes: Vec<[&'a str; 100]>
}


impl<'a> Buffer<'a> {
    fn new() -> Buffer {
        Buffer {
            bytes: vec::new()
        }
    }

    fn readwrite(&mut self, words: Option<&'a [u8]>) -> () {
        match words {
            Some(byte_slices) => {
                if let Ok(text) = std::str::from_utf8(byte_slices) {
                    let mut array: [&'a str; 100] = [""; 100];
                    for (i, word) in text.split_whitespace().take(100).enumerate(){
                        array[i] = word
                    }
                    self.bytes.push(array);
                        for (i, bytes) in self.bytes.iter().enumerate() {
                            println!("{}\t{:#?}", i, bytes);
                        }
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

fn read_file_by_limit(file_name: &Option<File>, buffer_limit: u64) -> Result<Vec<u8>, io::Error> {
    /*
     * reads file with set limit
     */
    match file_name.as_ref() {
        Some(f) => {
            let mut buff: Vec<u8> = Vec::new();
                f.try_clone()?.take(buffer_limit)
                .read_to_end(&mut buff)?;
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

fn main() {
    let mut get_user_run_save = || -> Result<String, io::Error> {
        let mut cin = String::with_capacity(100);
        let stdin = io::stdin();
        stdin.read_line(&mut cin)?;

        let cleaned_input = cin.trim_end().to_string();

        let abs_formatter = |c: &str| -> Result<PathBuf, io::Error> {
            let file_object = Path::new(c);
            if file_object.is_absolute() {
                Ok(file_object.to_path_buf())
            } else {
                absolute(file_object)
            }
        };
        let formatted_path = abs_formatter(&cleaned_input)?;

        match note_keeper(None, &formatted_path, String::from("something that should be a String type")) {
            Ok(_) => match(read_file_by_limit(Some(&formatted_path), 100)) {
                Ok(limit_buff) => {
                   if (Buffer.bytes.len() != 0) {
                       let mut instance = Buffer::new();
                   }
                   instance::readwrite(Some(limit_buff));
               } Err(e) => panic!("read_file_by_limit call has panicked")
            }
            Err(e) => {
                eprintln!("note_keeper failed: {}", e);
                panic!("note_keeper failed!");
            }
        }

    };

    let res = get_user_run_save().unwrap();
}
