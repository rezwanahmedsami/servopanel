pub mod filehandler {
    use std::fs::File;
    use std::io::prelude::*;
    pub fn write_file(filepath: &str, filecontent: &str){
        let mut file = File::create(filepath).expect("Can't create file");
        file.write_all(filecontent.as_bytes())
        .expect("Can't write to the file");
    }
}