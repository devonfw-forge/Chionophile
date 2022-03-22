echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching JTQ RUST (Actix)"
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo



err_display(){
    echo $err_msg
    echo
    echo "Press any key to exit"
    while [ true ] ; do
        read -t 3 -n 1
        if [ $? = 0 ] ; then
            exit ;
        fi
    done
    exit
}


# Check if Rust directory exists
if [ ! -d "Rust" ] && cd .. && [ ! -d "Rust" ] ; then
    err_msg="[ERROR] Rust directory does not exists."
    err_display
fi

# Check if Cargo exists
if  ! ( cargo --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command cargo missing"
    err_display
fi

# Move to the project directory
cd RUST

echo "Executing..."
if ! ( cargo run --release ) ; then
    cargo clean 2> /dev/null
    mkdir target ; mkdir target/release
    echo "Copying dlls"
    cp -r ../error_fixes/PostgresClientLibraries/* target/release/ || ( err_msg="[ERROR] Missing dlls"; err_display )
    echo
    echo "Executing with dlls"
    cargo run --release || ( err_msg="[ERROR] Cargo release exited"; err_display )
fi