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


# Check if benchmarks directory exists
if [ ! -d "benchmarks" ] && cd .. && [ ! -d "benchmarks" ] ; then
    err_msg="[ERROR] benchmarks directory does not exists."
    err_display
fi

# Check if Cargo exists
if  ! ( cargo --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command cargo missing"
    err_display
fi

# Move to the benchmarks directory
cd benchmarks

# Check if benchmark2 directory exists
if [ ! -d "benchmark2" ] && cd .. && [ ! -d "benchmark2" ] ; then
    err_msg="[ERROR] benchmark2 directory does not exists."
    err_display
fi

# Move to the second benchmark directory
cd benchmark2

echo "Executing First Beanchmark..."
"C:\Program Files\Intel\Power Gadget 3.6\PowerLog3.0.exe" \
    -cmd "cargo run --release" || ( err_msg="[ERROR] Test or PowerLog went wrong"; err_display )

cd ../../results
file=PowerLogB2
count=0
save_file=$file"_"$count".csv"
while [ -f $save_file ] ; do
    echo "$save_file already exists"
    let count=count+1
    save_file=$file"_"$count".csv"
done
echo "Saving as "$save_file

report_file="reportB2_"$count".html"

cp ../benchmarks/benchmark2/PowerLog.csv $save_file
rm ../benchmarks/benchmark2/PowerLog.csv

cp ../benchmarks/benchmark2/report.html $report_file
rm ../benchmarks/benchmark2/report.html
