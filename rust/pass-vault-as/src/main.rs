mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_password_from_file;
use crate::pentry::ServiceInfo;

fn main() {
    clr();
    let ascii = r#"
    السالم عليكم     
    ____   __   ____  ____       _  _   __   _  _  __   ____ 
    (  _ \ / _\ / ___)/ ___) ___ / )( \ / _\ / )( \(  ) (_  _)
     ) __//    \\___ \\___ \(___)\ \/ //    \) \/ (/ (_/\ )(  
    (__)  \_/\_/(____/(____/      \__/ \_/\_/\____/\____/(__) 
    "#;
    println!("{ascii}");
    loop 
    {
        println!("Password manager menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim()
        {
            "1" =>
            {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: ")
                );
                println!("Entry added successfully");
                entry.write_to_file();
            }
            "2" =>
            {
                clr();
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service : {}\nUsername : {}\nPassword : {}",
                        item.service, item.username, item.password
                    );
                    println!("===============\n");
                }
            }
            "3" =>
            {
                clr();
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                let search = prompt("Enter service name:");
                for item in &services {
                    if item.service.trim() == search.trim() 
                    {
                        println!(
                            "Service : {}\nUsername : {}\nPassword : {}",
                            item.service, item.username, item.password
                        );
                        println!("===============\n");
                    }
                }

            }
            "4" =>
            {
                break;
            }
            _ =>
            {
                println!("Not a valid choice.");
            }
        }
    }
}


fn clr()
{
    println!("{}[2J", 27 as char);
}