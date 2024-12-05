use serde::{Deserialize,Serialize};
use std::io::{self, stdout, BufReader, BufRead};
use std::io::Write;
use std::fs::File;
use std::fs:: OpenOptions;

#[derive(Debug,Serialize,Deserialize)]
pub struct ServiceInfo
{
    pub service: String,
    pub username: String,
    pub password: String
}

impl ServiceInfo
{
    pub fn new(service: String, username: String, password: String) -> Self
    {
        ServiceInfo
        {
            service,
            username,
            password
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error>
    {
        serde_json::from_str(json_string)
    }

    pub fn to_json(&self) -> String
    {
        serde_json::to_string(&self).expect("Error parsing the serviceinfo")
    }

    #[allow(dead_code)]
    pub fn from_user_input() -> Self
    {
        println!("Enter Password Entry:");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("Error reading service");

        println!("Enter username:");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Error reading username");

        println!("Enter password:");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Error reading password");

        Self::new
        (
            service.trim()
                            .to_string(),
            username.trim()
                            .to_string(),
            password.trim()
                            .to_string()
        )
    }

    pub fn write_to_file(&self)
    {
        let json_output = format!("{},\n", self.to_json());

        match OpenOptions::new()
            .create(false)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) =>
            {
                if let Err(e) = file.write(json_output.as_bytes())
                {
                    println!("Error writing to file: {}", e);
                }
                else 
                {
                    println!("Successfully wrote to passwords.json");
                }
            }
            Err(e) => println!("Error opening password file: {}", e)
        }   
    }
}

pub fn read_password_from_file() -> Result<Vec<ServiceInfo>, io::Error>
{
    let file = File::open("passwords.json").expect("Unable to open passwords json");
    let reader = BufReader::new(file);

    let mut services = Vec::new();

    for line in reader.lines()
    {
        if let Ok(json_string) = line
        {
            println!("Before reading: {}", &json_string);
            match ServiceInfo::from_json(&json_string)
            {
                Ok(service_info) => services.push(service_info),
                Err(e) => eprintln!("Error in second: {}", e)
            }
        }
        else {
            eprintln!("ERROR in first");
        }
    }
    Ok(services) 
}

pub fn prompt(_prompt: &str) -> String
{
    println!("{}", _prompt);
    stdout()
        .flush()
        .unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input.trim()
        .to_string()
}