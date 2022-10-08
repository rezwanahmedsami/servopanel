pub mod require_paths{
    pub static ETC: &str = "/etc/";
    pub static APACHE_SERVER: &str = "/etc/httpd";
    pub static SITES_AVAILABLE: &str = "/etc/httpd/sites-available/";
    pub static SITES_ENABLED: &str = "/etc/httpd/sites-enabled/";
    pub static WEB_ROOT: &str = "/var/www/";
}

pub mod system_operations {
    use std::process::Command;
    pub fn execute_command(command: &str){
        let split_command = command.split(" ");
        let commands_vec: Vec<&str> = split_command.collect();
        let first_command = commands_vec[0];
        let mut cmd = Command::new(first_command);
        // println!("{:?}", commands_vec[0]);
        for c in commands_vec{
            if c != first_command {
                cmd.arg(c);
            }
        }
        match cmd.output(){
            Ok(o) => {
                unsafe {
                    // String::from_utf8_unchecked(o.stdout);
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("There was an error: {}", e);
            }
        }
    }
}

pub mod operations {
    pub fn setup_apache(){
        
    }
}