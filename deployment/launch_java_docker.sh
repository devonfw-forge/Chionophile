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


# Check if java directory exists
if [ ! -d "java" ] && cd .. && [ ! -d "java" ] ; then
    err_msg="[ERROR] java directory does not exists."
    err_display
fi

# Move to the project directory
cd java

echo "Executing..."
docker-compose down
docker build . -t jtq-java
docker-compose up