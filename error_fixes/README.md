# ERROR FIXES

## PostgreSQL "Client" libraries

This folder contains all the needed .dll, .lib and one .exe file that are required by the Rust JTQ in order to access the PostreSQL database.

To use them, either add all the files to the Rust executable folder (/target/debug/ or /target/release after being compiled with cargo) or put them in another folder and add it to your PATH variable.