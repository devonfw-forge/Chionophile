# ERROR FIXES

## PostgreSQL "Client" libraries

This folder contains all the needed binaries that are required by the Rust JTQ in order to access the PostreSQL database.

To use them, either add all the files to the Rust executable folder (/target/debug/ or /target/release after being compiled with cargo) or put them in another folder and add it to your PATH variable.
This are needed in 2 situations

1. To execute the application:

    In this case, adding all the files to the Rust executable folder (/target/debug/ or /target/release after being compiled with cargo) is enough.

2. To run diesel migrations

    To be able to do this, you must add the folder that contains the binaries to your variable PATH. Then it won't be necessary to include them in the same folder as the executable.
