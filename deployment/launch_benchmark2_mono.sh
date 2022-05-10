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


# Check if actions directory exists
if [ ! -d "actions" ] && cd .. && [ ! -d "actions" ] ; then
    err_msg="[ERROR] benchmarks directory does not exists."
    err_display
fi

# Check if Cargo exists
if  ! ( cargo --version &> /dev/null ) ; then 
    err_msg="[ERROR] Command cargo missing"
    err_display
fi

# Move to the benchmarks directory
cd actions

# Check if benchmark1 directory exists
if [ ! -d "benchmark" ] && cd .. && [ ! -d "benchmark" ] ; then
    err_msg="[ERROR] benchmark1 directory does not exists."
    err_display
fi

# Move to the first benchmark directory
cd benchmark

echo "Executing Second Beanchmark..."
cargo run --release -- -i ../config2_mono.yaml || ( err_msg="[ERROR] Test or PowerLog went wrong"; err_display )

cd ../../results/central_monolitic

report_file="reportB2_Mono.html"

echo "Saving as "$report_file

cp ../../actions/benchmark/report2.html $report_file
rm ../../actions/benchmark/report2.html
