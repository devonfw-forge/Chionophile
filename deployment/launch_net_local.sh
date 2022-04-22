echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching JTQ .NET"
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
cd dist
cd release1

echo "Executing..."
./Devon4Net.Application.WebAPI.exe || ( err_msg="[ERROR] Execution exited" )
