@echo OFF

wt ; new-tab -p "Command Prompt" -d "server" --title "WAID Server" cmd.exe /k npm start ; new-tab -p "Command Prompt" -d "client" --title "WAIT Client" cmd.exe /k cargo run --release