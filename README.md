# EyeOxide
A compilation of tools to help perform OSINT investigations.

The fast and reliable successor to [BringEmNear](https://github.com/rootprt/BringEmNear). Looking to add more functionality over time.

![EyeOxide](preview.gif)

## Installation
1.  Copy the repo and run the installer script:
    
    ```
    git clone https://github.com/rootprt/EyeOxide.git

    cd EyeOxide

    chmod +x install.sh
    ./install.sh
    ```
    Then, to run EyeOxide:
    ```
    eyox
    ```

2.  Set your API keys as environment variables using your shell:

    <details>
    <summary>Bash</summary>

    ```
    $editor ~/.bashrc
    ```
    
    In your config file, add the following lines, replacing api_keys as necessary:
    ```sh
    export SNUS_API="snusbase_api_key"
    export IP_API="ipinfo_api_key"
    ```
    </details>

    <details>
    <summary>Zsh</summary>

    ```
    $editor ~/.zshrc
    ```
    
    In your config file, add the following lines, replacing api_keys as necessary:
    ```sh
    export SNUS_API="snusbase_api_key"
    export IP_API="ipinfo_api_key"
    ```
    </details>

    <details>
    <summary>Fish</summary>

    ```
    $editor ~/.config/fish/config.fish
    ```
    
    In your config file, add the following lines, replacing api_keys as necessary:
    ```sh
    set -x SNUS_API 'snusbase_api_key' 
    set -x IP_API 'ipinfo_api_key'
    ```
    </details>

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
