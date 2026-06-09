[CmdletBinding()]
param(
    [string]$Version = $env:KITKIT_VERSION,
    [string]$InstallDir = $env:KITKIT_INSTALL_DIR,
    [string]$Target = $env:KITKIT_TARGET,
    [string]$Repository = $env:KITKIT_REPO,
    [switch]$NoPathUpdate
)

Set-StrictMode -Version 2.0
$ErrorActionPreference = "Stop"

$AppName = "kitkit-cli"
$BinName = "kitkit-cli.exe"

if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = "latest"
}
if ([string]::IsNullOrWhiteSpace($Repository)) {
    $Repository = "stello-agent/kitkit-cli"
}

if ($PSVersionTable.PSVersion.Major -lt 6) {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
}

function Fail {
    param([string]$Message)
    throw "kitkit-cli install: $Message"
}

function Resolve-Tag {
    param([string]$RequestedVersion)

    if ($RequestedVersion -eq "latest") {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repository/releases/latest"
        if ([string]::IsNullOrWhiteSpace($release.tag_name)) {
            Fail "could not resolve latest release tag for $Repository"
        }
        return $release.tag_name
    }

    if ($RequestedVersion.StartsWith("v")) {
        return $RequestedVersion
    }

    return "v$RequestedVersion"
}

function Resolve-Target {
    if (-not [string]::IsNullOrWhiteSpace($Target)) {
        return $Target
    }

    $archName = $null
    try {
        $archName = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString()
    } catch {
        $archName = $env:PROCESSOR_ARCHITECTURE
    }

    switch -Regex ($archName.ToLowerInvariant()) {
        "^(x64|amd64)$" { return "x86_64-pc-windows-msvc" }
        "^(arm64|aarch64)$" { return "aarch64-pc-windows-msvc" }
        default { Fail "unsupported CPU architecture: $archName" }
    }
}

function Invoke-DownloadFile {
    param(
        [string]$Uri,
        [string]$OutFile
    )

    $params = @{
        Uri = $Uri
        OutFile = $OutFile
    }

    if ($PSVersionTable.PSVersion.Major -lt 6) {
        $params.UseBasicParsing = $true
    }

    Invoke-WebRequest @params
}

function Get-DefaultInstallDir {
    if (-not [string]::IsNullOrWhiteSpace($InstallDir)) {
        return $InstallDir
    }

    if (-not [string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
        return (Join-Path $env:LOCALAPPDATA "Programs\kitkit-cli\bin")
    }

    if (-not [string]::IsNullOrWhiteSpace($HOME)) {
        return (Join-Path $HOME ".kitkit-cli\bin")
    }

    Fail "LOCALAPPDATA and HOME are not set; pass -InstallDir"
}

function Test-PathListContains {
    param(
        [string]$PathList,
        [string]$Directory
    )

    if ([string]::IsNullOrWhiteSpace($PathList)) {
        return $false
    }

    $targetPath = [IO.Path]::GetFullPath($Directory).TrimEnd([char[]]@("\", "/"))
    foreach ($entry in $PathList.Split([IO.Path]::PathSeparator)) {
        if ([string]::IsNullOrWhiteSpace($entry)) {
            continue
        }
        try {
            $entryPath = [IO.Path]::GetFullPath($entry).TrimEnd([char[]]@("\", "/"))
        } catch {
            continue
        }
        if ([string]::Equals($entryPath, $targetPath, [StringComparison]::OrdinalIgnoreCase)) {
            return $true
        }
    }

    return $false
}

function Format-PowerShellStringLiteral {
    param([string]$Value)
    return "'" + $Value.Replace("'", "''") + "'"
}

$Tag = Resolve-Tag $Version
$ResolvedTarget = Resolve-Target
$Archive = "$AppName-$Tag-$ResolvedTarget.zip"
$BaseUrl = "https://github.com/$Repository/releases/download/$Tag"

$TempDir = Join-Path ([IO.Path]::GetTempPath()) ("kitkit-cli-install-" + [Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $TempDir | Out-Null

try {
    $ArchivePath = Join-Path $TempDir $Archive
    $ExtractDir = Join-Path $TempDir "extract"
    New-Item -ItemType Directory -Path $ExtractDir | Out-Null

    Write-Host "Installing $AppName $Tag for $ResolvedTarget"
    Invoke-DownloadFile -Uri "$BaseUrl/$Archive" -OutFile $ArchivePath

    Expand-Archive -Path $ArchivePath -DestinationPath $ExtractDir -Force
    $Binary = Get-ChildItem -Path $ExtractDir -Recurse -File -Filter $BinName | Select-Object -First 1
    if ($null -eq $Binary) {
        Fail "$BinName was not found in $Archive"
    }

    $FinalInstallDir = Get-DefaultInstallDir
    New-Item -ItemType Directory -Path $FinalInstallDir -Force | Out-Null
    $Destination = Join-Path $FinalInstallDir $BinName
    Copy-Item -Path $Binary.FullName -Destination $Destination -Force
    if (Get-Command Unblock-File -ErrorAction SilentlyContinue) {
        Unblock-File -Path $Destination -ErrorAction SilentlyContinue
    }

    $processPath = [Environment]::GetEnvironmentVariable("Path", "Process")
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $pathContainsInstallDir = (Test-PathListContains -PathList $processPath -Directory $FinalInstallDir) -or
        (Test-PathListContains -PathList $userPath -Directory $FinalInstallDir)

    if (-not $NoPathUpdate -and -not $pathContainsInstallDir) {
        if ([string]::IsNullOrWhiteSpace($userPath)) {
            $newUserPath = $FinalInstallDir
        } else {
            $newUserPath = "$userPath$([IO.Path]::PathSeparator)$FinalInstallDir"
        }
        [Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")
        $env:Path = "$env:Path$([IO.Path]::PathSeparator)$FinalInstallDir"
        Write-Host "Added $FinalInstallDir to the user PATH. Open a new terminal if this shell cannot find $AppName."
    } elseif (-not $pathContainsInstallDir) {
        Write-Host "Add $FinalInstallDir to PATH to run $AppName without the full path."
    }

    try {
        $installedVersion = & $Destination --version
        Write-Host "Installed $installedVersion to $Destination"
    } catch {
        Write-Host "Installed $AppName to $Destination"
    }

    $quotedDestination = Format-PowerShellStringLiteral $Destination
    Write-Host ""
    Write-Host "To uninstall:"
    Write-Host "  & $quotedDestination auth logout  # optional: remove cached tokens first"
    Write-Host "  Remove-Item -LiteralPath $quotedDestination -Force"
    Write-Host "  Remove $FinalInstallDir from the user PATH if it is no longer needed."
} finally {
    Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
}
