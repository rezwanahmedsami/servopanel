mod tools;
mod fileoperations;
// use std::process::Command;
fn main() {
    
    // println!("systempath: {}", tools::require_paths::SITES_AVAILABLE);
    // let filepath: &str = "./test.txt";
    // let filecontent: &str = "This is file content just for test"; 
    // fileoperations::filehandler::write_file(filepath, filecontent)
    // Command::new("ls")
    //     .arg("-l")
    //     .arg("-a")
    //     .spawn()
    //     .expect("ls command failed to start");
    // let command: &str = "sudo su";
    // tools::system_operations::execute_command(command)

    // tools::operations::add_domain();
    // let paths = fileoperations::filehandler::list_dir("/etc");
    // for path in paths {
    //     let path_to_string = path.unwrap().path().display().to_string();
    //     let path_split = path_to_string.split("/");
    //     let splited_path: Vec<&str> = path_split.collect();
    //     // p.split("/");
    //     println!("{:?}", splited_path[splited_path.len() - 1]);
    // }

    
    // println!("is dir: {}", fileoperations::filehandler::is_dir("./LICENSE"));
    // println!("is file: {}", fileoperations::filehandler::is_file("./LICENSE"));
    // println!("path exist: {}", fileoperations::filehandler::path_exists("./LICENSEs"));

    tools::operations::view_domains();
}
