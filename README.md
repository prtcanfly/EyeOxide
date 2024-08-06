# EyeOxide
A compilation of tools to help perform OSINT investigations.

The fast and reliable successor to [BringEmNear](https://github.com/rootprt/BringEmNear). Looking to add more functionality over time.
![EyeOxide](eyox.gif)
## Installation
Copy the repo and edit main.rs to enter your API keys. When finished, run the program with cargo.
```
git clone https://github.com/rootprt/EyeOxide.git
cd EyeOxide
$editor src/enums.rs
cargo run
```

## Usage
#### The current commands are as follows:

#### Ip
```
ip 
```
asks for an Ip address, and prints the corresponding IpInfo data

#### Snus
```
snus
```
searches snusbase databases using a search type and a search term

#### User
```
user
```
searches for social media accounts using a username

#### Hash
```
hash 
```
similar to snus, it searches snusbase for cracked passwords using their corresponding hashes, can save the headache of dehashing them yourself
