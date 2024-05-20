@echo OFF

net session >nul 2>&1
if %errorlevel% == 0 (
    goto :run
)

echo WAID needs to be ran as admin, requesting permissions...
powershell start cmd.exe -verb runas -ArgumentList '/c', 'cd /d "%cd%" ^&^& "%0"' & exit /b

:run

wt ; new-tab -p "Command Prompt" -d "server" --title "WAID Server" cmd.exe /k npm start ; new-tab -p "Command Prompt" -d "client" --title "WAIT Client" cmd.exe /k cargo run --release