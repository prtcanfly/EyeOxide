# EyeOxide
A compilation of tools to help perform OSINT investigations.

The fast and reliable successor to [BringEmNear](https://github.com/rootprt/BringEmNear). Looking to add more functionality over time.

![EyeOxide](preview.gif)

## Installation
Copy the repo and add your API keys to a .env file:
```
git clone https://github.com/rootprt/EyeOxide.git
cd EyeOxide
$editor .env
```
Enter your keys in this format, with no quotations:
```
SNUS_API=api_key
IP_API=api_key
```
When finished, run the program with cargo:
```
cargo run
```

## Alternative Installation
Run install.sh to add the program to your ~/.cargo/bin.
```
chmod +x install.sh
./install.sh
```
Then, to run EyeOxide:
```
eyox
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
