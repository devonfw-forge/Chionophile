# JTQ en Rust

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

**If you have followed the steps to create the database in the [Postgres folder's README.md](https://github.com/devonfw-forge/Chionophile/tree/Develop/postgres), you should already have a completely ready database and you can skip this step.**

-----

Create a PostgreSQL Database called "jtq" with the user "postgres" and password "123". Of course, this can be changed to your liking by adjusting the command below and the .cargo/config file.

Then run this command. Make sure the you have the necessary binaries in your PATH. If not, return to step 3.

```
diesel migration run --database-url postgres://postgres:123@localhost/jtq
```

To revert the database to its default state run 

```
diesel migration redo --database-url postgres://postgres:123@localhost/jtq
```

### 6. Run the application

Run the application with 
```
cargo run
```
or 
```
cargo run --release
```