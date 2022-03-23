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

echo "Executing..."
yarn install || ( err_msg="[ERROR] yarn install exited" )
yarn start || ( err_msg="[ERROR] yarn start exited" )
