param(
    [int]$TimeoutSeconds = 90,
    [string]$LogPath = "target/qemu-iso.log",
    [string]$MonitorHost = "127.0.0.1",
    [int]$MonitorPort = 4444
)

Write-Host "Waiting up to $TimeoutSeconds seconds for LFB marker in $LogPath..."
$end = (Get-Date).AddSeconds($TimeoutSeconds)

while ((Get-Date) -lt $end) {
    if (Test-Path $LogPath) {
        $content = Get-Content -Raw -Path $LogPath -ErrorAction SilentlyContinue

        if ($content -match "\[v10\.3\] LFB: desktop render complete at 0x([0-9A-Fa-f]+)\s*\[\s*([0-9]+)x([0-9]+)\s*\]") {
            $addrHex = $matches[1]
            $width = [int]$matches[2]
            $height = [int]$matches[3]
            Write-Host "Found rendered LFB at 0x$addrHex ${width}x${height}"
            $size = [int]($width * $height * 4)
            $cmd = "pmemsave 0x$addrHex $size target/framebuffer.raw`n"
        } elseif ($content -match "LFB: Visual Sovereignty Active at 0x([0-9A-Fa-f]+)\s*\[\s*([0-9]+)x([0-9]+)\s*\]") {
            $addrHex = $matches[1]
            $width = [int]$matches[2]
            $height = [int]$matches[3]
            Write-Host "Found LFB at 0x$addrHex ${width}x${height} (waiting for desktop render complete)"
            Start-Sleep -Seconds 1
            continue
        } else {
            Start-Sleep -Milliseconds 500
            continue
        }

        Write-Host ("Connecting to monitor {0}:{1} ..." -f $MonitorHost, $MonitorPort)
        try {
            $client = New-Object System.Net.Sockets.TcpClient($MonitorHost, $MonitorPort)
        } catch {
            Write-Host "Failed to connect to monitor: $_. Retrying..."
            Start-Sleep -Seconds 1
            continue
        }

        $stream = $client.GetStream()
        $writer = New-Object System.IO.StreamWriter($stream)
        $reader = New-Object System.IO.StreamReader($stream)
        $writer.AutoFlush = $true

        Start-Sleep -Milliseconds 200
        if ($stream.DataAvailable) {
            try { $banner = $reader.ReadLine(); Write-Host $banner } catch {}
        }

        Write-Host "Sending: $cmd"
        $writer.Write($cmd)
        Start-Sleep -Milliseconds 500

        # Read response lines if any (non-blocking)
        Start-Sleep -Milliseconds 500
        while ($stream.DataAvailable) {
            try { $line = $reader.ReadLine(); if ($line) { Write-Host $line } } catch { break }
        }

        $client.Close()
        Write-Host "pmemsave command issued; waiting briefly for file..."
        $waitEnd = (Get-Date).AddSeconds(10)
        while ((Get-Date) -lt $waitEnd) {
            if (Test-Path "target/framebuffer.raw") { Write-Host "framebuffer.raw created"; exit 0 }
            Start-Sleep -Milliseconds 200
        }

        Write-Warning "framebuffer.raw not found yet but pmemsave was sent. Exiting (monitor handled the dump)."
        exit 0
    }
    Start-Sleep -Milliseconds 500
}

Write-Error "Timed out waiting for LFB marker in $LogPath"
exit 2
