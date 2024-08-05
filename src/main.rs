use ipinfo::{IpInfo, IpInfoConfig};
use reqwest::blocking::Client;
use serde_json::{
    json,
    Value,
    to_string_pretty
};
use std::{
    error::Error,
    process::exit,
    str::FromStr,
    io::{
        self,
        Write
    },
};

const IP_API: &str = "<IpInfo API Key>";
const SNUS_API: &str = "<Snusbase API Key>";
const SNUS_URL: &str = "https://api.snusbase.com/data/search";
const HASH_URL: &str = "https://api.snusbase.com/tools/hash-lookup";

enum Commands {
    Ip,
    Snus,
    User,
    Hash,
    Exit,
    Unknown,
}

enum Functions{}

// match string inputs to their corresponding Commands
impl FromStr for Commands {
    type Err = ();

    fn from_str(input: &str) -> Result<Commands, Self::Err> {
        match input {
            "ip" => Ok(Commands::Ip),
            "snus" => Ok(Commands::Snus),
            "user" => Ok(Commands::User),
            "hash" => Ok(Commands::Hash),
            "exit" => Ok(Commands::Exit),
            _ => Ok(Commands::Unknown),
        }
    }
}

// functions for manipulating data
impl Functions {
    // match each Command to its assigned function
    fn handle_command(command: Commands) {
        match command {
            Commands::Ip => Commands::ip_info(),
            Commands::Snus => Commands::snusbase().unwrap_or_default(),
            Commands::User => Commands::user_search().unwrap_or_default(),
            Commands::Hash => Commands::hash_lookup().unwrap_or_default(),
            Commands::Exit => exit(0),
            Commands::Unknown => println!(""),
        }
    }
    
    // user input function
    fn get_input(s: &mut String) -> &str { 
        io::stdin()
            .read_line(s)
            .expect("Failed to read line.");

        s.trim()
    }
   
    // takes client, url, and body as input, prints a parsed json output, returns ()
    fn print_json(c: Client, u: &str, b: Value) -> Result<(), Box<dyn Error>>  {
        let res = c.post(u)
            .header("Auth", SNUS_API)
            .header("Content-Type", "application/json")
            .json(&b)
            .send()?;

        let res_txt = res.text()?.trim().to_string();  
        let json_default: Value = serde_json::from_str(&res_txt)?;
        let pretty_json = to_string_pretty(&json_default)?;
        
        println!("{}", pretty_json);

        Ok(())
    }

}

// create and assign the functions each Command will call
impl Commands {
    // search for an ip on ipinfo.io
    #[tokio::main]
    async fn ip_info() {
        let config = IpInfoConfig {
            token: Some(IP_API.to_string()),
            ..Default::default()
        };

        let mut ipinfo = IpInfo::new(config)
            .expect("Failed to connect to IpInfo.");

        println!("Enter IP:");
        let mut input = String::new();
        let ip = Functions::get_input(&mut input);

        let res = ipinfo.lookup(ip).await;
        match res {
            Ok(r) => {
                let json = to_string_pretty(&r).unwrap();
                println!("{}", json);
            },
            Err(e) => println!("error occured: {}", &e.to_string()), 
        }
    }
    
    // similar to sherlock, finds accounts using a username
    fn user_search() -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        println!("Username:");
        let mut input = String::new();
        let username = Functions::get_input(&mut input);
        
        let sites = vec![
            format!("https://instagram.com/{}", username),
            format!("https://github.com/{}", username),
            format!("https://x.com/{}", username),
            format!("https://reddit.com/u/{}", username),
            format!("https://tiktok.com/@{}", username),           
            format!("https://imgur.com/user/{}", username),
            format!("https://facebook.com/{}", username),
            format!("https://pinterest.com/{}", username),
            format!("https://t.me/{}", username),
            format!("https://www.tumblr.com/{}", username),
        ];

        for site in &sites {
            let res = client
                .get(site)
                .send()?;
            let res = res.status();
            println!("[{}]: {}", res, site);
        }

        Ok(())
    }

    // cracks certain hashes you may find searching snusbase
    fn hash_lookup() -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        println!("Search Params:");
        let mut input = String::new();
        let search_term = Functions::get_input(&mut input); 
        
        let body = json!({
            "terms": [search_term],
            "types": ["hash"],
        });

        Functions::print_json(client, HASH_URL, body)
    }
    
    // search snusbase databases using a search_type and search_term
    fn snusbase() -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        println!("Search Type:");
        let mut input = String::new();
        let search_type = Functions::get_input(&mut input);

        println!("Search Params:");
        let mut input = String::new();
        let search_term = Functions::get_input(&mut input);

        let body = json!({
            "terms": [search_term],
            "types": [search_type],
            "wildcard": true
        });

        Functions::print_json(client, SNUS_URL, body)
    }
}


// run the cli on a loop, and trim command inputs
fn cli() {
    loop {
        let mut input = String::new();
        print!("-> ");
        
        io::stdout()
            .flush()
            .unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read command.");
        
        let input = input 
            .trim()
            .to_lowercase(); 
        
        match Commands::from_str(input.as_str()) {
            Ok(command) => Functions::handle_command(command), 
            Err(_) => println!("Failed to parse command."), 
        }
    }
}

// print the ascii art, then start the cli loop
fn main() {
    println!(r#"MM""""""""`M                   MMP"""""YMM          oo       dP          "#);
    println!(r#"MM  mmmmmmmM                   M' .mmm. `M                   88          "#);
    println!(r#"M`      MMMM dP    dP .d8888b. M  MMMMM  M dP.  .dP dP .d888b88 .d8888b. "#);
    println!(r#"MM  MMMMMMMM 88    88 88ooood8 M  MMMMM  M  `8bd8'  88 88'  `88 88ooood8 "#);
    println!(r#"MM  MMMMMMMM 88.  .88 88.  ... M. `MMM' .M  .d88b.  88 88.  .88 88.  ... "#);
    println!(r#"MM        .M `8888P88 `88888P' MMb     dMM dP'  `dP dP `88888P8 `88888P' "#);
    println!(r#"MMMMMMMMMMMM      .88          MMMMMMMMMMM                               "#);
    println!(r#"              d8888P                                                     "#);
    
    cli();
}
