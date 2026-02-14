@echo off
setlocal

REM cargo から渡される「実行ファイルのパス」は最初の引数 %1
set BIN=%~1

echo [probe-rs] flashing: "%BIN%"
rem probe-rs download --chip RP2040 --verify "%BIN%"
probe-rs download --chip RP2040 "%BIN%"
if errorlevel 1 exit /b %errorlevel%

probe-rs reset --chip RP2040
exit /b %errorlevel%
