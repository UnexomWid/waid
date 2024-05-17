@echo OFF

cd server

npm i

copy config.sample.json config.json

cd ../client

cargo run -- --setup

cd ..
