echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Closing JTQ Java"
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
if [ ! -d "java" ] && cd .. && [ ! -d "java" ] ; then
    err_msg="[ERROR] java directory does not exists."
    err_display
fi

# Check if DockerCompose exists
if  ! ( docker-compose --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command docker-compose missing"
    err_display
fi

# Move to the project directory
cd java

docker-compose down 2> /dev/null || ( err_msg="[ERROR] Could not close java."; err_display )