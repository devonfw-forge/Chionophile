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


# Check if rust directory exists
if [ ! -d "rust" ] && cd .. && [ ! -d "rust" ] ; then
    err_msg="[ERROR] rust directory does not exists."
    err_display
fi

# Move to the project directory
cd rust

echo "Executing..."
docker-compose down
docker-compose up