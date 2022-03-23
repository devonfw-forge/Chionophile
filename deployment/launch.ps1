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

function Start-Testing {
    Write-Host ("Launching Test... ")
    Start-Process -Wait launch_benchmark.sh
    Write-Host ("Test finished")
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
    Start-Testing
    Write-Host ("`r`nKilling "+$name+" and Postgres...")
    Taskkill /PID $process.Id /F *> $null
    Start-Process -Wait close_postgres.sh
}

# Checking benchmark before execution:
Set-Location ../benchmark; cargo build --release *> $null
Set-Location ../deployment

Write-Host "`r`nChecking Idle"
Start-Process -Wait test_idle.sh

Write-Host ("Waiting 1 minute before launching benchmark...`r`n")
Start-Sleep -s (60);

Start-Backend -p_bash "launch_rust.sh" -name "JTQ Rust"

Write-Host ("Waiting 5 minutes...`r`n")
Start-Sleep -s (5*60);

Start-Backend -p_bash "launch_java.sh" -name "JTQ Java"

Write-Host ("Waiting 5 minutes...`r`n")
Start-Sleep -s (5*60);

Start-Backend -p_bash "launch_node.sh" -name "JTQ Node"

Pause