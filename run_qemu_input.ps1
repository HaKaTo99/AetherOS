# QEMU Input Test Script
# Creates a named pipe for input to AetherOS

$ErrorActionPreference = "Stop"

$kernelPath = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"
$inputPipe = "\\.\pipe\aetheros_input"
$outputLog = "D:\GitHub\AetherOS\kernel\qemu_input_test.log"

# Create named pipe server
Write-Host "Creating input pipe..."
$pipeServer = New-Object System.IO.Pipes.NamedPipeServerStream($inputPipe, [System.IO.Pipes.PipeDirection]::Out, 1, [System.IO.Pipes.PipeTransmissionMode]::Byte, [System.IO.Pipes.PipeOptions]::Asynchronous)

# Wait for connection in background
$pipeServer.WaitForConnection()

Write-Host "Pipe connected! Starting QEMU..."
Write-Host "Type your commands below and press Enter:"

# Start QEMU in background
$qemuProc = Start-Process -FilePath "qemu-system-x86_64" -ArgumentList @(
    "-kernel", $kernelPath,
    "-m", "1024M",
    "-nographic",
    "-serial", "mon:stdio"
) -NoNewWindow -PassThru -RedirectStandardInput $inputPipe

# Give it time to start
Start-Sleep -Seconds 3

# Send test commands
$commands = @("help", "exit")
foreach ($cmd in $commands) {
    Write-Host "Sending: $cmd"
    $pipeServer.Write([byte[]][char[]]$cmd, 0, $cmd.Length)
    $pipeServer.Write([byte[]][char[]]"`n", 0, 1)
    Start-Sleep -Seconds 1
}

# Wait for QEMU
$qemuProc.WaitForExit()

$pipeServer.Close()
Write-Host "Done!"
