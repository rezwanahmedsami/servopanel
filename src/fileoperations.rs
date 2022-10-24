pub mod filehandler {
    use std::fs::{self, ReadDir};
    use std::io::prelude::*;
    use std::fs::metadata;

    pub fn write_file(filepath: &str, filecontent: &str){
        let mut file = fs::File::create(filepath).expect("Can't create file");
        file.write_all(filecontent.as_bytes())
        .expect("Can't write to the file");
    }

    pub fn list_dir(dir_path: &str) -> ReadDir{
        let paths = fs::read_dir(dir_path).unwrap();
        return paths;
    }

    pub fn is_dir(path: &str) -> bool{
        let md = metadata(path).unwrap();
        return md.is_dir();
    }

    pub fn is_file(path: &str) -> bool {
        let md = metadata(path).unwrap();
        return md.is_file();
    }

    pub fn path_exists(path: &str) -> bool {
        return fs::metadata(path).is_ok()
    }
}