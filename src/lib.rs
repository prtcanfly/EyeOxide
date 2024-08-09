use dotenvy::dotenv;
use ipinfo::{IpInfo, IpInfoConfig};
use reqwest::blocking::Client;
use serde_json::{json, to_string_pretty, Value};
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Write},
    process::exit,
    str::FromStr,
};

const SNUS_URL: &str = "https://api.snusbase.com/data/search";
const HASH_URL: &str = "https://api.snusbase.com/tools/hash-lookup";

#[derive(Debug)]
pub enum Tools {}

#[derive(Debug)]
enum Commands {
    Ip,
    Snus,
    SnusWrite,
    User,
    Hash,
    Help,
    Exit,
    Unknown,
}

// match string inputs to their corresponding Commands
impl FromStr for Commands {
    type Err = ();

    fn from_str(input: &str) -> Result<Commands, Self::Err> {
        match input {
            "ip" => Ok(Commands::Ip),
            "snus" => Ok(Commands::Snus),
            "snus -w" | "snus --write" => Ok(Commands::SnusWrite),
            "user" => Ok(Commands::User),
            "hash" => Ok(Commands::Hash),
            "help" => Ok(Commands::Help),
            "exit" => Ok(Commands::Exit),
            _ => Ok(Commands::Unknown),
        }
    }
}

// functions for manipulating data
impl Tools {
    // match each Command to its assigned function
    fn handle_command(command: Commands) {
        let dont_write = false;
        let write_to_file = true;

        match command {
            Commands::Ip => Commands::ip_info(),
            Commands::Snus => Commands::snusbase(dont_write).unwrap_or_default(),
            Commands::SnusWrite => Commands::snusbase(write_to_file).unwrap_or_default(),
            Commands::User => Commands::user_search().unwrap_or_default(),
            Commands::Hash => Commands::hash_lookup().unwrap_or_default(),
            Commands::Help => Commands::print_help(),
            Commands::Exit => exit(0),
            Commands::Unknown => println!(""),
        }
    }

    // user input function
    fn get_input(s: &mut String) -> &str {
        io::stdin().read_line(s).expect("Failed to read line.");

        s.trim()
    }

    // takes client, url, and body as input, prints a parsed json output, returns ()
    fn print_json(c: Client, u: &str, b: Value, o: bool) -> Result<(), Box<dyn Error>> {
        dotenv().expect("Not Found");

        let snus_api = env::var("SNUS_API").expect("No API key found.");

        let res = c
            .post(u)
            .header("Auth", snus_api)
            .header("Content-Type", "application/json")
            .json(&b)
            .send()?;

        let res_txt = res.text()?.trim().to_string();
        let json_default: Value = serde_json::from_str(&res_txt)?;
        let pretty_json = to_string_pretty(&json_default)?;

        if o {
            File::create("output.txt")?.write_all(pretty_json.as_bytes())?;
            println!("Written to output.txt");
        } else {
            println!("{}", pretty_json);
        }
        Ok(())
    }

    // run the cli on a loop, and trim command inputs
    pub fn cli() {
        loop {
            let mut input = String::new();
            print!("-> ");

            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Cannot read command.");

            let input = input.trim().to_lowercase();

            match Commands::from_str(input.as_str()) {
                Ok(command) => Tools::handle_command(command),
                Err(_) => println!("Failed to parse command."),
            }
        }
    }
}

// create the functions each Command will call
impl Commands {
    // search for an ip on ipinfo.io
    #[tokio::main]
    async fn ip_info() {
        dotenv().expect("Not Found");

        let ip_api = env::var("IP_API").expect("No API key found.");

        let config = IpInfoConfig {
            token: Some(ip_api.to_string()),
            ..Default::default()
        };

        let mut ipinfo = IpInfo::new(config).expect("Failed to connect to IpInfo.");

        println!("Enter IP:");
        let mut input = String::new();
        let ip = Tools::get_input(&mut input);

        let res = ipinfo.lookup(ip).await;
        match res {
            Ok(r) => {
                let json = to_string_pretty(&r).unwrap();
                println!("{}", json);
            }
            Err(e) => println!("error occured: {}", &e.to_string()),
        }
    }

    // similar to sherlock, finds accounts using a username
    fn user_search() -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        println!("Username:");
        let mut input = String::new();
        let username = Tools::get_input(&mut input);

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
            let res = client.get(site).send()?;
            let res = res.status();
            println!("[{}]: {}", res, site);
        }

        Ok(())
    }

    // cracks certain hashes you may find searching snusbase
    fn hash_lookup() -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        println!("Hash Value:");
        let mut input = String::new();
        let search_term = Tools::get_input(&mut input);

        let body = json!({
            "terms": [search_term],
            "types": ["hash"],
        });

        Tools::print_json(client, HASH_URL, body, false)
    }

    // search snusbase databases using a search_type and search_term
    fn snusbase(o: bool) -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        println!("Type (username, email, lastip, hash, password, name):");
        let mut input = String::new();
        let search_type = Tools::get_input(&mut input);

        println!("Search For:");
        let mut input = String::new();
        let search_term = Tools::get_input(&mut input);

        let body = json!({
            "terms": [search_term],
            "types": [search_type],
            "wildcard": true
        });

        Tools::print_json(client, SNUS_URL, body, o)
    }

    // self explanatory lol
    fn print_help() {
        println!("");
        println!("Commands:");
        println!("   ip   | Fetch data about an ip using IpInfo.\n");
        println!("   snus | Search Snusbase databases for leaked info using a type and a term.");
        println!("      snus [-w/--write] | Writes the output to a text file.\n");
        println!(
            "   user | Search social media sites for accounts that use a specific username.\n"
        );
        println!("   hash | Check Snusbase for cracked passwords using password hashes.");
        println!("");
    }
}
