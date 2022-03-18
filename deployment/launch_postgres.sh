echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching POSTGRESQL"
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


# Check if PostgreSQL directory exists
if [ ! -d "PostgreSQL" ] && cd .. && [ ! -d "PostgreSQL" ] ; then
    err_msg="[ERROR] PostgreSQL directory does not exists."
    err_display
fi

# Check if DockerCompose exists
if  ! ( docker-compose --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command docker-compose missing"
    err_display
fi

# Move to the project directory
cd PostgreSQL

docker-compose down 2> /dev/null
docker-compose up || ( err_msg="[ERROR] Could not launch postgres."; err_display )