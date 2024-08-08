# EyeOxide
A compilation of tools to help perform OSINT investigations.

The fast and reliable successor to [BringEmNear](https://github.com/rootprt/BringEmNear). Looking to add more functionality over time.

![EyeOxide](preview.gif)

## Installation
1.  Copy the repo and cd into the directory:
    
    > ```sh
    > git clone https://github.com/rootprt/EyeOxide.git
    >  
    > cd EyeOxide
    > ``` 
    
2.  Add your API keys, and then run the install script.

    > Make and open an .env file in EyeOxide's main folder:  
    > ```sh
    > $editor .env
    > ```
    > 
    > In your .env file, add the following lines, replacing api_keys as necessary:
    > ```sh
    > SNUS_API=snusbase_api_key
    > IP_API=ipinfo_api_key
    > ```
    > 
    > Run the install script:
    > ```sh
    > chmod +x install.sh
    > 
    > ./install.sh
    > ```
    
## Commands
All of these commands can be viewed easily by typing:
```
help
```

#### Ip
Asks for an Ip address, and prints the corresponding IpInfo data:
```
ip 
```

#### Snus
Searches snusbase databases using a search type and a search term:
```
snus
```

#### User
Searches for social media accounts using a username:
```
user
```

#### Hash
Similar to snus, it searches snusbase for cracked passwords using their corresponding hashes:
```
hash 
```
