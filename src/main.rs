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
    let command: &str = "mongodrun";
    tools::system_operations::execute_command(command)
}
