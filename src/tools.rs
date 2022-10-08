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
        for c in commands_vec{
            if c != first_command {
                cmd.arg(c);
            }
        }
        match cmd.output(){
            Ok(o) => {
                unsafe {
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
    use crate::tools;
    pub fn setup_apache(){
        println!("Apache server setup started....");
        tools::system_operations::execute_command("sudo yum install update");
        println!("Installing httpd......");
        tools::system_operations::execute_command("sudo yum install httpd");
        println!("Adding service to firewall ....");
        tools::system_operations::execute_command("sudo firewall-cmd --permanent --add-service=https");
        println!("Reloading firewall");
        tools::system_operations::execute_command("sudo firewall-cmd --reload");
        println!("Starting httpd....");
        tools::system_operations::execute_command("sudo systemctl start httpd");
        let concat_vec: Vec<&str> = vec!["sudo mkdir ",tools::require_paths::SITES_AVAILABLE," ", tools::require_paths::SITES_ENABLED];
        tools::system_operations::execute_command(&concat_vec.concat());
        println!("Setting success.... now run the script again to get panel and go to your ip address to access web");
    }
}