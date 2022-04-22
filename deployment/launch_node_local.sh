echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching JTQ Node"
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

# Check if node directory exists
if [ ! -d "node" ] && cd .. && [ ! -d "node" ] ; then
    err_msg="[ERROR] node directory does not exists."
    err_display
fi

# Check if yarn exists
if  ! ( yarn --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command yarn missing"
    err_display
fi

# Move to the project directory
cd node

# Environment variables:
export HOST="0.0.0.0"
export PORT="8081"
export DB_HOST="localhost"
export DB_PORT="5432"
export DB_NAME="jtq_db"
export DB_USER="jtq_user"
export DB_PASSWORD="admin"
export BASE_REST_URL="jumpthequeue/services/rest"
export EXECUTION="single"#"cluster"

echo "Executing..."
yarn install || ( err_msg="[ERROR] yarn install exited" )
yarn start || ( err_msg="[ERROR] yarn start exited" )
