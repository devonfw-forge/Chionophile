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

function Start-NETBenchmark {
    Write-Host ("Launching .NET Benchmarks... ")
    Write-Host ("This will take more than 13 minutes!")
    Write-Host ("Launching First Benchmark... ")
    Start-Process -Wait launch_net_benchmark1.sh
    Write-Host ("First Benchmark finished")
    Write-Host ("Launching Second Benchmark... ")
    Start-Process -Wait launch_net_benchmark2.sh
    Write-Host ("Second Benchmark finished")
}

function Start-Postgres {
    Write-Host ("`r`nLaunching postgres...")
    $launch = Start-Process -PassThru launch_postgres.sh
    $port = 5432
    Wait-MyProcess -proc $launch -port $port
    Write-Host ("Postgres running`r`n")
    Write-Host ("Waiting 30 seconds...`r`n")
    Start-Sleep -s 30;
}

function Start-Backend() {
    Param($p_bash, $name)
    Start-Postgres
    Write-Host ("Launching "+$name+"...")
    $launch = Start-Process -PassThru $p_bash
    $port = 8081
    Wait-MyProcess -proc $launch -port $port
    $process = Get-Process -Id (Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue).OwningProcess
    Write-Host ($name+" running with PID: "+$process.Id+"`r`n")
    Write-Host ("Waiting 2 minutes before testing...`r`n")
    Start-Sleep -s (2*60)
    if ( $name.Contains("NET") ) {
        Start-NETBenchmark
    }
    else {
        Start-Benchmark
    }
    Write-Host ("`r`nKilling "+$name+" and Postgres...")
    Taskkill /PID $process.Id /F *> $null
    Start-Process -Wait close_postgres.sh
}

function Start-DockerBackend() {
    Param($p_bash, $c_bash, $name)
    Write-Host ("Launching "+$name+"...")
    $launch = Start-Process -PassThru $p_bash
    $port = 8081
    # It waits until process starts listening on port 8081, exit if process terminates
    Wait-MyProcess -proc $launch -port $port
    Write-Host ($name+" running`r`n")
    Write-Host ("Waiting 2 minutes before testing...`r`n")
    Start-Sleep -s (2*60)
    if ( $name.Contains("NET") ) {
        Start-NETBenchmark
    }
    else {
        Start-Benchmark
    }
    Write-Host ("`r`nStopping "+$name+" and Postgres...")
    Start-Process -Wait $c_bash
}

function Start-Processes{
    try{
        Write-Host "`r`nChecking and building benchmarks before execution...`r`n"
        # Checking benchmark before execution:
        Set-Location ../benchmarks/benchmark1; cargo build --release
        Set-Location ../benchmark2; cargo build --release 
        Set-Location ../net_benchmark1; cargo build --release 
        Set-Location ../net_benchmark2; cargo build --release 
        Set-Location ../../deployment

        Write-Host "`r`nChecking idle"
        Start-Process -Wait test_idle.sh

        Write-Host ("Waiting 1 minute before launching benchmark...`r`n")
        Start-Sleep -s (60);
        Start-DockerBackend -p_bash "launch_rust_docker.sh" -c_bash "close_rust_docker.sh" -name "JTQ Rust"

        Write-Host ("Waiting 5 minutes...`r`n")
        Start-Sleep -s (5*60);
        Start-DockerBackend -p_bash "launch_java_docker.sh" -c_bash "close_java_docker.sh" -name "JTQ Java"

        Write-Host ("Waiting 5 minutes...`r`n")
        Start-Sleep -s (5*60);
        Start-DockerBackend -p_bash "launch_node_docker.sh" -c_bash "close_node_docker.sh" -name "JTQ Node"
        
        Write-Host ("Waiting 5 minutes...`r`n")
        Start-Sleep -s (5*60);
        Start-DockerBackend -p_bash "launch_python_docker.sh" -c_bash "close_python_docker.sh" -name "JTQ Python"
        
        #Write-Host ("Waiting 5 minutes...`r`n")
        #Start-Sleep -s (5*60);
        
        #Start-Backend -p_bash "launch_net.sh" -name "JTQ .NET"
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

py ./cleaner.py

Pause