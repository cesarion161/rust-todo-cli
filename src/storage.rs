use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write};

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(file_url: &str, truncate: bool) -> io::Result<FileWriter> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(truncate)
            .create(true)
            .open(file_url)?;
        Ok(FileWriter {
            file,
        })
    }

    pub fn write_to_file(&mut self, content: String) -> io::Result<()> {
        self.file.write_all(content.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    pub fn read_file(&mut self) -> io::Result<String> {
        let mut buf_reader = BufReader::new(&self.file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
