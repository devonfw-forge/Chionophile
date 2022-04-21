echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching JTQ PYTHON (Django)"
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


# Check if java directory exists
if [ ! -d "python" ] && cd .. && [ ! -d "python" ] ; then
    err_msg="[ERROR] python directory does not exists."
    err_display
fi

# Move to the project directory
cd python
cd nginx

echo "Executing..."
docker-compose down
docker-compose up