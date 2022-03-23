cd ../results
"C:\Program Files\Intel\Power Gadget 3.6\PowerLog3.0.exe" \
    -cmd "sleep 300"

file=Idle
save_file=$file"_"$count".csv"
count=0
while [ -f $save_file ] ; do
    echo "$save_file already exists"
    let count=count+1
    save_file=$file"_"$count".csv"
done
echo "Saving as "$save_file
cp PowerLog.csv $save_file
rm PowerLog.csv