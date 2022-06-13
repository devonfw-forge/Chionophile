* Add wasm32-wasi:
    ~~~
    rustup target add wasm32-wasi
    ~~~
* Clone spin repo:
    ~~~
    git clone https://github.com/fermyon/spin 
    ~~~cd spin
* Build spin framework:
    ~~~
    cargo build --release
    ~~~
* Add it to the PATH
* Build the wasm project:
    ~~~
    spin build --up --listen=127.0.0.1:8081
    ~~~
    o
    ~~~
    cargo build --target wasm32-wasi --release --manifest-path Cargo.toml
    ~~~