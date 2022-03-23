echo
echo "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
echo "Launching Benchmark"
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


# Check if benchmark directory exists
if [ ! -d "benchmark" ] && cd .. && [ ! -d "benchmark" ] ; then
    err_msg="[ERROR] benchmark directory does not exists."
    err_display
fi

# Check if Cargo exists
if  ! ( cargo --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command cargo missing"
    err_display
fi

# Move to the project directory
cd benchmark

echo "Executing..."
"C:\Program Files\Intel\Power Gadget 3.6\PowerLog3.0.exe" \
    -cmd "cargo run --release" || ( err_msg="[ERROR] Test or PowerLog went wrong"; err_display )

cd ../results
file=PowerLog
count=0
save_file=$file"_"$count".csv"
while [ -f $save_file ] ; do
    echo "$save_file already exists"
    let count=count+1
    save_file=$file"_"$count".csv"
done
echo "Saving as "$save_file

report_file="report_"$count".html"

cp ../benchmark/PowerLog.csv $save_file
rm ../benchmark/PowerLog.csv

cp ../benchmark/report.html $report_file
rm ../benchmark/report.html