Write-Host "`r`n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
Write-Host "Launching Cloud benchmarks"
Write-Host "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"


function Start-Benchmark {
    Write-Host "`r`nChecking and building benchmarks before execution...`r`n"
    # Checking benchmark before execution:
    Set-Location ../actions/benchmark; cargo build --release
    Set-Location ../../deployment
    # Launching benchmarks
    Write-Host ("Launching Benchmarks... ")
    Write-Host ("This will take more than 10 minutes!")
    Write-Host ("Launching First Benchmark... ")
    Start-Process -Wait launch_benchmark_cloud1.sh
    Write-Host ("First Benchmark finished")
    # Write-Host ("Launching Second Benchmark... ")
    # Start-Process -Wait launch_benchmark_cloud2.sh
    # Write-Host ("Second Benchmark finished")
}

Start-Benchmark

if (Test-Path '..\results\temp') {
    Remove-Item '..\results\temp' -Recurse
}
New-Item -Path "..\results\" -Name "temp" -ItemType "directory"
Copy-Item -Path "..\results\*.html" -Destination "..\results\temp" -Recurse

# py ./cleaner.py

Pause