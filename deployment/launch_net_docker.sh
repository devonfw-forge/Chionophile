echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching JTQ JAVA (Spring Boot)"
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


# Check if dotnet directory exists
if [ ! -d "dotnet" ] && cd .. && [ ! -d "dotnet" ] ; then
    err_msg="[ERROR] dotnet directory does not exists."
    err_display
fi

# Move to the project directory
cd dotnet

echo "Executing..."
docker-compose down
docker-compose up