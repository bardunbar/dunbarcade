@echo off
SETLOCAL

echo %1

IF "%1" == "debug" GOTO Proceed
IF "%1" == "release" GOTO Proceed

:Usage
echo "Error: Invalid arguments. Usage: package [debug|release]"
EXIT /B 1

:Proceed
set outputdir="target\wasm32-unknown-unknown\%1"
robocopy assets "%outputdir%\assets" /mir

IF %ERRORLEVEL% EQU 2 GOTO ProcessHtml
IF %ERRORLEVEL% EQU 1 GOTO ProcessHtml
IF %ERRORLEVEL% EQU 0 GOTO ProcessHtml

:RobocopyError
echo "A robocopy error occurred. Examine log for details"
EXIT /B 1

:ProcessHtml
robocopy "www" "%outputdir%" "index.html"

IF %ERRORLEVEL% EQU 1 GOTO Package
IF %ERRORLEVEL% EQU 0 GOTO Package

GOTO RobocopyError

:Package
7z a -tzip "%outputdir%\%1.zip" ".\%outputdir%\arcade.wasm" ".\%outputdir%\index.html" ".\%outputdir%\assets"