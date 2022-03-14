# ERROR FIXES

## libintl-9.dll

The current libintl-9.dll, present in the necessary files to use PostgreSQL as a client, has a critical error that stops the Rust JTQ when trying to connect to the DB.

This file is from the library getttext. Funnily enough, libintl-8.dll contains more up-to-date code than libintl-9.dll, as addressed by @hut8 in this thread https://github.com/diesel-rs/diesel/discussions/2947

The libintl-9.dll present in this folder is just a renamed libintl-8.dll, and it needs to be in the same folder as the executable for the Rust JTQ. 
It was extracted from the gettext 0.21 64bit shared library that can be downloaded from here http://mlocati.github.io/articles/gettext-iconv-windows.html
