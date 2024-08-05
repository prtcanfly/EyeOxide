# EyeOxide
EyeOxide is equipped with a couple things to help perform OSINT investigations.

This was meant to be a successor to [BringEmNear](https://github.com/rootprt/BringEmNear), but it deserves its own title...

## Installation
Copy the repo and edit main.rs to enter your API keys. When finished, run the program with cargo.
```
git clone https://github.com/rootprt/EyeOxide.git
cd EyeOxide
$editor src/main.rs
cargo run
```

## Usage
#### There is currently only a couple commands to choose from, they are as follows:

##### Ip
```
ip 
```
asks for an Ip address, and prints the corresponding IpInfo data

##### Snus
```
snus
```
searches snusbase databases using a search type and a search term

##### Hash
```
hash 
```
similar to snus, it searches snusbase for cracked passwords using their corresponding hashes, can save the headache of dehashing them yourself
