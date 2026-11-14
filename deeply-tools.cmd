@echo off
setlocal DisableDelayedExpansion
rem Deeply Tools entry point: keyboard menus, PR tracking and release management.
rem Load scripts\tools\deeply-tools.ps1 from the repository or the project folder;
rem those scripts reopen the menu automatically after a tools update.
rem Alone in a folder, download the installer and install the project there.
rem (goto rather than ( ) blocks: a path with a parenthesis would break a block)
title Deeply Tools
cd /d "%~dp0"
if exist "%~dp0scripts\tools\deeply-tools.ps1" goto repo
if exist "%~dp0deeply-viewer\scripts\tools\deeply-tools.ps1" goto project
powershell -NoProfile -ExecutionPolicy Bypass -Command "$f = Join-Path $env:TEMP 'deeply-setup.ps1'; irm https://raw.githubusercontent.com/Deeply-Viewer/Deeply-Viewer/main/scripts/setup.ps1 -OutFile $f; & $f"
set "DeeplyToolsExitCode=%errorlevel%"
pause
exit /b %DeeplyToolsExitCode%
:repo
set "DeeplyToolsScript=%~dp0scripts\tools\deeply-tools.ps1"
goto launch
:project
set "DeeplyToolsScript=%~dp0deeply-viewer\scripts\tools\deeply-tools.ps1"
:launch
powershell -NoProfile -ExecutionPolicy Bypass -File "%DeeplyToolsScript%" %*
exit /b %errorlevel%
