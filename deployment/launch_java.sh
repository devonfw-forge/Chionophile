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

echo "Executing..."
if ! ( mvn install ) ; then
    cd ..
    mvn clean || ( err_msg="[ERROR] Executing mvn clean."; err_display )
    mvn install || ( err_msg="[ERROR] Executing mvn clean."; err_display )
    cd core || ( err_msg="[ERROR] core directory does not exists."; err_display )
    mvn spring-boot:run || ( err_msg="[ERROR] Spring Boot error." )
else
    cd core || ( err_msg="[ERROR] core directory does not exists."; err_display )
    mvn spring-boot:run || ( err_msg="[ERROR] Spring Boot error." )
fi
