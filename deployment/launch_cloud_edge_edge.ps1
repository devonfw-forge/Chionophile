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
    Write-Host ("This will take more than 5 minutes!")
    # Launch powertop on aws central and edge server
    $body_central_1 = @{
        time = '300'
        csv = 'test_consumption_B1_edge_central.csv'
    }
    $body_central_2 = @{
        time = '300'
        csv = 'test_consumption_B2_edge_central.csv'
    }
    $body_edge_1 = @{
        time = '300'
        csv = 'test_consumption_B1_edge_edge.csv'
    }
    $body_edge_2 = @{
        time = '300'
        csv = 'test_consumption_B2_edge_edge.csv'
    }
    $url_edge = 'http://52.211.194.30:8083/powertop/start'
    $url_central = 'http://54.165.40.147:8083/powertop/start'
    Invoke-WebRequest -Uri $url_edge -Body ($body_edge_1|ConvertTo-Json) -ContentType "application/json" -Method 'POST'
    Invoke-WebRequest -Uri $url_central -Body ($body_central_1|ConvertTo-Json) -ContentType "application/json" -Method 'POST'
    # Launch benchmark
    Write-Host ("Launching First Benchmark... ")
    Start-Process -Wait launch_benchmark1_edge_edge.sh
    Write-Host ("First Benchmark finished")
    Invoke-WebRequest -Uri $url_edge -Body ($body_edge_2|ConvertTo-Json) -ContentType "application/json" -Method 'POST'
    Invoke-WebRequest -Uri $url_central -Body ($body_central_2|ConvertTo-Json) -ContentType "application/json" -Method 'POST'
    Write-Host ("Launching Second Benchmark... ")
    Start-Process -Wait launch_benchmark2_edge_edge.sh
    Write-Host ("Second Benchmark finished")
}

Start-Benchmark

if (Test-Path '..\results\temp') {
    Remove-Item '..\results\temp' -Recurse
}
New-Item -Path "..\results\" -Name "temp" -ItemType "directory"
Copy-Item -Path "..\results\*.html" -Destination "..\results\temp" -Recurse

# py ./cleaner.py

Pause