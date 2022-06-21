Write-Host "`r`n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"
Write-Host "Launching Processes"
Write-Host "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%"

function Wait-MyProcess {
    Param($proc, $port)
    do{
        Start-Sleep -s 1; $Failed = $true
        try{
            # Check if postgres is listening
            Get-Process -Id (Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue).OwningProcess *> $null
            $Failed = $false
        } 
        catch { }
        if ( $proc.HasExited ) {
            Write-Host ("`r`nProcess terminated"); Pause; Exit
        }
    } while ($Failed)
}

function Start-Benchmark {
    Write-Host ("Launching Benchmarks... ")
    Write-Host ("This will take more than 13 minutes!")
    Write-Host ("Launching First Benchmark... ")
    Start-Process -Wait launch_benchmark1.sh
    Write-Host ("First Benchmark finished")
    Write-Host ("Launching Second Benchmark... ")
    Start-Process -Wait launch_benchmark2.sh
    Write-Host ("Second Benchmark finished")
}

function Start-Backend() {
    Param($p_bash, $name)
    Write-Host ("Launching "+$name+"...")
    $launch = Start-Process -PassThru $p_bash
    $port = 8081
    Wait-MyProcess -proc $launch -port $port
    $process = Get-Process -Id (Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue).OwningProcess
    Write-Host ($name+" running with PID: "+$process.Id+"`r`n")
    Write-Host ("Waiting 2 minutes before testing...`r`n")
    Start-Sleep -s (2*60)
    Start-Benchmark
    Write-Host ("`r`nKilling "+$name+" and Postgres...")
    Start-Process -Wait close_wasm.sh
}

function Start-Processes{
    try{
        Write-Host "`r`nChecking and building benchmarks before execution...`r`n"
        # Checking benchmark before execution:
        Set-Location ../benchmarks/benchmark1; cargo build --release
        Set-Location ../benchmark2; cargo build --release
        Set-Location ../../deployment

        #Write-Host "`r`nChecking idle"
        #Start-Process -Wait test_idle.sh

        Write-Host ("Waiting 1 minute before launching benchmark...`r`n")
        Start-Sleep -s (60);
        Start-Backend -p_bash "launch_wasm.sh" -name "JTQ Wasm Rust"
    } 
    catch { 
        Write-Host ("`r`nProcess terminated"); Pause; Exit 
    }
}

Start-Processes

if (Test-Path '..\results\temp') {
    Remove-Item '..\results\temp' -Recurse
}
New-Item -Path "..\results\" -Name "temp" -ItemType "directory"
Copy-Item -Path "..\results\*.html" -Destination "..\results\temp" -Recurse

#py ./cleaner.py

Pause