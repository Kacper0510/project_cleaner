[![forthebadge](https://forthebadge.com/images/badges/60-percent-of-the-time-works-every-time.svg)](https://forthebadge.com)

# Project Cleaner

Disk cleaning program which mainly targets output and cache directories of programming languages and build systems

## Installation

### Nix

To run on nix you can use flakes:
```bash
nix run github:Kacper0510/project_cleaner 
```

To install it permanently add flake and package it provides to you configuration.

### Build from source 

To build or run project from source you need **git** and **rust** (version 1.8.0 or higher) installed with **cargo**.
1. Clone repo 
```bash
git clone https://github.com/Kacper0510/project_cleaner.git
cd project_cleaner
```

2. Run project
```bash
cargo run 
```

3. Or build
```bash
cargo build --release
```
Executable can be found in `target/release/project_cleaner`.