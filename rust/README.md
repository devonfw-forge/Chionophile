# Rust JTQ

## Windows Installation

Follow these steps to install and run this application

### 1. Install Rust and the libraries needed to compile 

The steps needed for your operating system are documented  here in the [Rust docs](https://doc.rust-lang.org/book/ch01-01-installation.html)

### 2. Clone this repository

### 3. Add the necessary binaries to use PostgreSQL

Go to the [error fixes folder](https://github.com/devonfw-forge/Chionophile/tree/Develop/error_fixes) in the root of this repository and follow the instructions under the "PostgreSQL CLient Libraries" section.

### 4. Install diesel_cli

Run this command from the root of the Rust project 

```
cargo install diesel_cli --no-default-features --features postgres
```

### 5. Create the Database

**Follow the steps to create the database in the [Postgres folder's README.md](https://github.com/devonfw-forge/Chionophile/tree/Develop/postgres).**

### 6. Run the application

Run the application with 
```
cargo run
```
or 
```
cargo run --release
```
