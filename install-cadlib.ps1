<#
.SYNOPSIS
    Installs the native cadlib library for the cadlib Rust bindings v1.0.0.

.DESCRIPTION
    Downloads cadlib-windows-x86_64.tar.gz from the v1.0.0 release, checks it against the
    SHA-256 digest embedded below (generated with this release), and installs cadlib.dll and its
    import library cadlib.lib into <Prefix>\lib, cadlib.h into <Prefix>\include, and the
    LICENSE and third-party notices into <Prefix>\share\doc\cadlib.

    CADLIB_DOWNLOAD_BASE points the script at a mirror of the release assets; the embedded
    digest still applies.

    Generated for v1.0.0. DO NOT EDIT.

.EXAMPLE
    irm https://raw.githubusercontent.com/SimScaleGmbH/simscale-rust-cadlib/v1.0.0/install-cadlib.ps1 | iex

.EXAMPLE
    iex "& { $(irm https://raw.githubusercontent.com/SimScaleGmbH/simscale-rust-cadlib/v1.0.0/install-cadlib.ps1) } -Prefix C:\cadlib"
#>
[CmdletBinding()]
param(
    # Install into <Prefix>\lib and <Prefix>\include (default: %LOCALAPPDATA%\cadlib).
    [string]$Prefix = '',
    # Print what would be installed, change nothing.
    [switch]$DryRun
)

# Errors throw rather than exit: under `irm | iex`, exit would close the caller's session.
$ErrorActionPreference = 'Stop'
# Invoke-WebRequest's progress bar slows downloads to a crawl in Windows PowerShell 5.1.
$ProgressPreference = 'SilentlyContinue'
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

$version = '1.0.0'
$repo = 'SimScaleGmbH/simscale-rust-cadlib'
$binding = 'rust'
$sha256 = '7fd60f401a94882c11b15bac25ae111730a524d93af0c98a4c835687ae73f8b1'
$downloadBase = if ($env:CADLIB_DOWNLOAD_BASE) { $env:CADLIB_DOWNLOAD_BASE } else { "https://github.com/$repo/releases/download" }

if ($PSVersionTable.PSEdition -eq 'Core' -and -not $IsWindows) {
    throw 'install-cadlib: install-cadlib.ps1 is for Windows; on Linux and macOS use install-cadlib.sh from the same release.'
}
$arch = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
if ($arch -ne 'AMD64') {
    throw "install-cadlib: unsupported architecture: $arch (cadlib is built for Windows x86_64 only)"
}

$url = "$downloadBase/v$version/cadlib-windows-x86_64.tar.gz"
if (-not $Prefix) { $Prefix = Join-Path $env:LOCALAPPDATA 'cadlib' }
$Prefix = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Prefix)
$libDir = Join-Path $Prefix 'lib'
$includeDir = Join-Path $Prefix 'include'
$docDir = Join-Path $Prefix 'share\doc\cadlib'

function Write-FollowUp {
    Write-Host "Next: point your build and your programs at $libDir, e.g. in your PowerShell profile:"
    switch ($binding) {
        'rust' { Write-Host "  `$env:CADLIB_LIB_DIR = '$libDir'" }
        'go' { Write-Host "  `$env:CGO_LDFLAGS = '-L$($libDir -replace '\\', '/')'" }
    }
    Write-Host "  `$env:PATH = '$libDir;' + `$env:PATH    # so cadlib.dll loads at run time"
    Write-Host "To persist one, use [Environment]::SetEnvironmentVariable('<name>', '<value>', 'User')."
}

if ($DryRun) {
    Write-Host "cadlib v$version for windows-x86_64 (dry run: nothing is downloaded or installed)"
    Write-Host "  download: $url"
    Write-Host "  sha256:   $sha256"
    Write-Host "  install:  $libDir\cadlib.dll"
    Write-Host "            $libDir\cadlib.lib"
    Write-Host "            $includeDir\cadlib.h"
    Write-FollowUp
    return
}

# The bsdtar shipped with Windows: a GNU tar earlier on PATH (Git for Windows) reads C:\ as a host.
$tar = Join-Path $env:SystemRoot 'System32\tar.exe'
if (-not (Test-Path $tar)) {
    throw 'install-cadlib: tar.exe not found; it ships with Windows 10 1803 and later.'
}

$tmp = Join-Path ([IO.Path]::GetTempPath()) ('cadlib-' + [guid]::NewGuid())
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
    Write-Host "Downloading cadlib v$version for windows-x86_64..."
    $archive = Join-Path $tmp 'cadlib.tar.gz'
    try {
        Invoke-WebRequest -UseBasicParsing -Uri $url -OutFile $archive
    } catch {
        throw "install-cadlib: download failed: $url ($($_.Exception.Message))"
    }
    $actual = (Get-FileHash -Algorithm SHA256 -Path $archive).Hash.ToLowerInvariant()
    if ($actual -ne $sha256) {
        throw "install-cadlib: checksum mismatch for ${url}: expected $sha256, got $actual. Nothing was installed."
    }

    $unpacked = Join-Path $tmp 'x'
    New-Item -ItemType Directory -Path $unpacked | Out-Null
    & $tar -xzf $archive -C $unpacked
    if ($LASTEXITCODE -ne 0) { throw "install-cadlib: could not unpack $archive" }
    # Archive path -> install dir, for every file the archive must carry.
    $files = [ordered]@{
        'lib\cadlib.dll' = $libDir
        'lib\cadlib.lib' = $libDir
        'include\cadlib.h' = $includeDir
        'LICENSE' = $docDir
    }
    foreach ($f in $files.Keys) {
        if (-not (Test-Path (Join-Path $unpacked $f))) {
            throw "install-cadlib: unexpected archive layout: $f is missing"
        }
    }

    try {
        foreach ($d in $files.Values | Select-Object -Unique) { New-Item -ItemType Directory -Force -Path $d | Out-Null }
    } catch {
        throw "install-cadlib: cannot write to ${Prefix}: choose another -Prefix ($($_.Exception.Message))"
    }
    foreach ($f in $files.Keys) { Copy-Item -Force (Join-Path $unpacked $f) $files[$f] }
    $notices = Join-Path $unpacked 'THIRD-PARTY-NOTICES'
    if (Test-Path $notices) { Copy-Item -Force $notices $docDir }
} finally {
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}

Write-Host "Installed cadlib v$version into $Prefix ($libDir\cadlib.dll)."
Write-FollowUp
