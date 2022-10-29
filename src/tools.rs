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

        cmd.spawn().ok().expect("Failes to write command");
    }
}

pub mod operations {
    use crate::tools;
    use crate::fileoperations;
    use std::io::{stdin,stdout,Write};
    use std::{thread, time::Duration};

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

    pub fn create_domain_config(domain_name: &str){
        let concat_vec: Vec<&str> = vec!["sudo mkdir -p ", tools::require_paths::WEB_ROOT, domain_name, "/public_html"];
        let concat_vec2: Vec<&str> = vec!["sudo mkdir -p ", tools::require_paths::WEB_ROOT, domain_name, "/log"];
        let concat_vec3: Vec<&str> = vec!["sudo chown -R $USER:$USER ", tools::require_paths::WEB_ROOT, domain_name, "/public_html"];
        let genrate_index_file: Vec<&str> = vec![tools::require_paths::WEB_ROOT, domain_name, "/public_html/index.html"];
        let index_file_content: Vec<&str> = vec!["<h1>Successfully working: ", domain_name, "</h1>"];
        let  genrate_domain_config: Vec<&str> = vec![tools::require_paths::SITES_AVAILABLE, domain_name,".conf"];
        let domain_config_content = format!( "
        <VirtualHost *:80>
            ServerName {}
            ServerAlias www.{}
            DocumentRoot /var/www/{}/public_html
            ErrorLog /var/www/{}/log/error.log
            CustomLog /var/www/{}/log/requests.log combined
        </VirtualHost>", domain_name, domain_name, domain_name, domain_name, domain_name);
        let  concat_vec4: Vec<&str> = vec!["sudo ln -s ",tools::require_paths::SITES_AVAILABLE, domain_name,".conf ", tools::require_paths::SITES_ENABLED, domain_name, ".conf"];
        let  concat_vec5: Vec<&str> = vec!["sudo ls -dZ ",tools::require_paths::WEB_ROOT, domain_name,"/log/"];
        let  concat_vec6: Vec<&str> = vec!["sudo semanage fcontext -a -t httpd_log_t '",tools::require_paths::WEB_ROOT, domain_name,"/log(/.*)?'"];
        let  concat_vec7: Vec<&str> = vec!["sudo restorecon -R -v ",tools::require_paths::WEB_ROOT, domain_name,"/log"];
        let  concat_vec8: Vec<&str> = vec!["ls -lZ ",tools::require_paths::WEB_ROOT, domain_name,"/log"];

        println!("PLease wait adding domain..");
        println!("Creating document root path for `{}` ...", domain_name);
        
        tools::system_operations::execute_command(&concat_vec.concat());
        println!("Creating document log path for `{}` ...", domain_name);
        
        tools::system_operations::execute_command(&concat_vec2.concat());
        println!("Working....");
        
        tools::system_operations::execute_command(&concat_vec3.concat());
        println!("Working....");
        
        fileoperations::filehandler::write_file(&genrate_index_file.concat(), &index_file_content.concat());
        println!("Working....");
        
        fileoperations::filehandler::write_file(&genrate_domain_config.concat(), &domain_config_content);
        println!("Working .....");
        
        tools::system_operations::execute_command(&concat_vec4.concat());
        println!("Working .....");
        
        tools::system_operations::execute_command(&concat_vec5.concat());
        println!("Working .....");
        
        tools::system_operations::execute_command(&concat_vec6.concat());
        println!("Working .....");
        
        tools::system_operations::execute_command(&concat_vec7.concat());
        println!("Working .....");
        tools::system_operations::execute_command("sudo systemctl restart httpd");
        println!("Working .....");
        tools::system_operations::execute_command(&concat_vec8.concat());

        println!("Successfully added domain");
    }

    pub fn add_domain(){
        thread::sleep(Duration::from_millis(1000));
        tools::system_operations::execute_command("clear");
        let mut domain_name=String::new();
        print!("Please enter your domain (Ex: example.com): ");
        let _=stdout().flush();
        stdin().read_line(&mut domain_name).expect("Did not enter a correct string");
        if let Some('\n')=domain_name.chars().next_back() {
            domain_name.pop();
        }
        if let Some('\r')=domain_name.chars().next_back() {
            domain_name.pop();
        }
        if domain_name != ""{
            if domain_name.contains("http://") || domain_name.contains("https://") || domain_name.contains("/") || domain_name.contains(":") || domain_name.contains(",") {
                println!("Invalid domain, please enter again.");
                add_domain();
            }else{
                tools::operations::create_domain_config(&domain_name);
            }
        }else{
            println!("Invalid domain, please enter again.");
            add_domain();
        }
    }

    pub fn get_domains(){
        let paths = fileoperations::filehandler::list_dir(tools::require_paths::SITES_AVAILABLE);
        let mut domain: Vec<&str> = vec![];
        for path in paths {
            let path_to_string = path.unwrap().path().display().to_string();
            let path_split = path_to_string.split("/");
            let splited_path: Vec<&str> = path_split.collect();
            let last_index = splited_path[splited_path.len() - 1];
            if fileoperations::filehandler::is_file(&path_to_string) {
                let split_filename = last_index.split(".");
                let splited_filename: Vec<&str> = split_filename.collect();
                if splited_filename[splited_filename.len() - 1] == "conf"  {
                    
                    // domain.push(last_index);
                    println!("File: {}", last_index);
                }
            }
        }
    }

    pub fn view_domains(){
        let paths = fileoperations::filehandler::list_dir(tools::require_paths::ETC);

        let mut i = 1;
        for path in paths {
            let path_to_string = path.unwrap().path().display().to_string();
            let path_split = path_to_string.split("/");
            let splited_path: Vec<&str> = path_split.collect();
            let last_index_value = splited_path[splited_path.len() - 1];
            if fileoperations::filehandler::is_file(&path_to_string) {
                let split_filename = last_index_value.split(".");
                let splited_filename: Vec<&str> = split_filename.collect();
                let last_index = splited_filename.len() - 1;
                if splited_filename[last_index] == "conf"  {
                    let mut domain_name = "".to_owned();
                    let mut x = 0;
                    for d_name in splited_filename {
                        if d_name != "conf" {
                            domain_name.push_str(d_name);
                        }

                        if x != (last_index-1) {
                            domain_name.push_str(".");
                        }else{
                            break;
                        }
                        x += 1;
                    }
                    // domain.push(last_index);
                    println!("{}. {}", i,domain_name);
                    i += 1;
                }
            }
        }
    }

    pub fn set_proxy_server(proxypass: &str, selected_domain: &str){
        println!("PLease wait Configuring domain..");
        let concat_vec1: Vec<&str> = vec![tools::require_paths::SITES_AVAILABLE, selected_domain, ".conf"];

        let domain_config_file = concat_vec1.concat();
        let domain_config_content = format!( "
        <VirtualHost *:80>
            ServerName {}
            ServerAlias www.{}

            ErrorLog /var/www/{}/log/error.log
            CustomLog /var/www/{}/log/requests.log combined
            ProxyPreserveHost On
            ProxyPass / {}
            ProxyPassReverse / {}
        </VirtualHost>", selected_domain, selected_domain, selected_domain, selected_domain, proxypass, proxypass);
        println!("{}", domain_config_content);
        fileoperations::filehandler::write_file(&domain_config_file, &domain_config_content);
        tools::system_operations::execute_command("sudo systemctl restart httpd");
        println!("Successfully added Proxy on: {}", selected_domain);
        println!("Proxy pass: {}", proxypass);
    }
}