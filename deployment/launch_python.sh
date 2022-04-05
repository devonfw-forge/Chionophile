echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching JTQ Python"
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

# Check if python directory exists
if [ ! -d "python" ] && cd .. && [ ! -d "python" ] ; then
    err_msg="[ERROR] node directory does not exists."
    err_display
fi

# Check if python exists
if  ! ( python --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command python missing"
    err_display
fi

# Move to the project directory
cd python
cd jtq

echo "Executing..."
python .\manage.py runserver || ( err_msg="[ERROR] django server exited" )
