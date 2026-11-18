<#
.SYNOPSIS
    Puts the built viewer and its files in a folder ready to launch (or to
    zip): exe, assets, licences. The GitHub release and build-release.ps1
    both use it: their folders are identical. The Linux job of the release
    runs it too (PowerShell 7), with the executable without extension.

.EXAMPLE
    ./scripts/package.ps1 -Exe target/release/deeply-viewer.exe -Out Deeply-Viewer
.EXAMPLE
    ./scripts/package.ps1 -Exe target/release/deeply-viewer -Out Deeply-Viewer-v0.9.11-linux-x64
#>
param(
    [Parameter(Mandatory)][string]$Exe,
    [Parameter(Mandatory)][string]$Out,
    # repository whose assets go with the exe (default: this script's)
    [string]$Source = (Split-Path $PSScriptRoot -Parent),
    [string]$ModelPath
)

New-Item -ItemType Directory -Force -Path (Join-Path $Out 'assets/emoji') -ErrorAction Stop | Out-Null
Copy-Item -LiteralPath $Exe -Destination (Join-Path $Out 'deeply-viewer.exe') -ErrorAction Stop
# Reserved location for the optional offline NLLB model. The viewer reports
# an explicit unavailable state until the model and a compatible native
# CTranslate2/ct2rs runtime are supplied; it never falls back to the network.
$modelOut = Join-Path $Out 'models/nllb-200-600M-int8'
New-Item -ItemType Directory -Force -Path $modelOut -ErrorAction Stop | Out-Null
if ($ModelPath) {
    if (-not (Test-Path -LiteralPath $ModelPath -PathType Container)) {
        throw "Offline translation model directory was not found: $ModelPath"
    }
    Copy-Item -Path (Join-Path $ModelPath '*') -Destination $modelOut -Recurse -Force -ErrorAction Stop
} else {
    Set-Content -LiteralPath (Join-Path $modelOut 'README.txt') -Value 'Optional offline NLLB-200 600M int8 model files go here.' -Encoding UTF8
}
# The emoji font is inside the exe; its licence ships, and the assets folder
# stays: the updater of the published versions refuses a package without it.
Copy-Item -LiteralPath (Join-Path $Source 'assets/emoji/LICENSE-OFL.txt') -Destination (Join-Path $Out 'assets/emoji') -ErrorAction Stop
# The spelling dictionaries are inside the exe too: their licences ship
# (GPL, LGPL and MPL ask for the text), one per dictionary, with the list of
# their sources.
$dictionaries = Join-Path $Source 'crates/deeply-viewer/assets/dictionaries'
$dictionariesOut = Join-Path $Out 'assets/dictionaries'
New-Item -ItemType Directory -Force -Path $dictionariesOut -ErrorAction Stop | Out-Null
Copy-Item -LiteralPath (Join-Path $dictionaries 'README.md') -Destination $dictionariesOut -ErrorAction Stop
Get-ChildItem -LiteralPath $dictionaries -Directory -ErrorAction Stop | ForEach-Object {
    $license = Join-Path $_.FullName 'LICENSE.txt'
    if (Test-Path -LiteralPath $license) {
        Copy-Item -LiteralPath $license -Destination (Join-Path $dictionariesOut "$($_.Name)-LICENSE.txt") -ErrorAction Stop
    }
}
$docs = @(
    'LICENSE',
    'NOTICE.md',
    'TRADEMARKS.md',
    'README.md',
    'Deeply-Viewer-0.9.12-Miglioramenti-e-Potenzialita.txt',
    'Deeply-Viewer-0.9.12-Improvements-and-Capabilities.txt',
    'Deeply-Viewer-0.9.12-Release-Notes-it.txt',
    'Deeply-Viewer-0.9.12-Release-Notes-en.txt',
    'Deeply-Viewer-0.9.12-Guida-uso-it.txt',
    'Deeply-Viewer-0.9.12-User-Guide-en.txt'
) | ForEach-Object { Join-Path $Source $_ }
Copy-Item -LiteralPath $docs -Destination $Out -ErrorAction Stop
# Every bundled third-party component keeps its complete license text in a
# package-relative directory. Flattening the repository path makes names
# stable while avoiding collisions between similarly named component files.
$licensesOut = Join-Path $Out 'licenses'
New-Item -ItemType Directory -Force -Path $licensesOut -ErrorAction Stop | Out-Null
$licenseRoots = @(
    (Join-Path $Source 'assets'),
    (Join-Path $Source 'crates')
)
Get-ChildItem -LiteralPath $licenseRoots -Recurse -File -ErrorAction Stop |
    Where-Object { $_.Name -match '^(LICENSE|COPYING|NOTICE)([-_.].*)?$' } |
    ForEach-Object {
        $relative = $_.FullName.Substring($Source.Length).TrimStart('\')
        $name = ($relative -replace '[\\/]', '-') -replace '[^A-Za-z0-9._-]', '_'
        Copy-Item -LiteralPath $_.FullName -Destination (Join-Path $licensesOut $name) -ErrorAction Stop
    }
$branding = Join-Path $Out 'assets/deeply'
New-Item -ItemType Directory -Force -Path $branding -ErrorAction Stop | Out-Null
Copy-Item -LiteralPath (Join-Path $Source 'assets/deeply/deeply-logo.png') -Destination $branding -ErrorAction Stop
Copy-Item -LiteralPath (Join-Path $Source 'assets/deeply/deeply-splash.png') -Destination $branding -ErrorAction Stop
Copy-Item -LiteralPath (Join-Path $Source 'assets/deeply/deeply-viewer.ico') -Destination $branding -ErrorAction Stop
# Linux (an executable without extension): the desktop entry and its icon,
# named after the viewer's Wayland app_id / X11 WM_CLASS (cli.rs DESKTOP_ID)
# so the desktop matches the window to them. docs/LINUX.md installs them.
# (macOS has its own bundle: package-macos.ps1; $IsMacOS is unset, so
# false, in Windows PowerShell 5.1.)
if (-not [IO.Path]::GetExtension($Exe) -and -not $IsMacOS) {
    Copy-Item -LiteralPath (Join-Path $Source 'packaging/linux/deeply-viewer.desktop') -Destination $Out -ErrorAction Stop
    Copy-Item -LiteralPath (Join-Path $Source 'assets/branding/deeply-logo.svg') -Destination (Join-Path $Out 'deeply-viewer.svg') -ErrorAction Stop
}
$taskExePath = (Resolve-Path -LiteralPath $Exe).Path
$taskVersion = (& $taskExePath --language=en --version | Out-String).Trim() -replace '^Deeply Viewer ', ''
if ($LASTEXITCODE -ne 0 -or $taskVersion -notmatch '^\d+\.\d+\.\d+(\.\d+)?([+-][0-9A-Za-z.+-]+)?$') { throw 'Invalid package version.' }
$taskJson = @{ schema = 1; version = $taskVersion } | ConvertTo-Json
$taskMetadataPath = Join-Path (Resolve-Path -LiteralPath $Out -ErrorAction Stop).Path 'deeply-install.json'
# Windows PowerShell 5.1 does not support Set-Content -Encoding utf8NoBOM.
[IO.File]::WriteAllText($taskMetadataPath, $taskJson, [Text.UTF8Encoding]::new($false))
