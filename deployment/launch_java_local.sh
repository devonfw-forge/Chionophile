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

# Check if Maven exists
if  ! ( mvn --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command mvn missing"
    err_display
fi

# Move to the project directory
cd java

# Environment variables:
export SPRING_DATASOURCE_URL="jdbc:postgresql://localhost/jtq_db"
export SPRING_DATASOURCE_USERNAME="jtq_user"
export SPRING_DATASOURCE_PASSWORD="admin"

echo "Executing..."
#mvn clean
#mvn install
cd core || ( err_msg="[ERROR] core directory does not exists."; err_display )
mvn spring-boot:run