@echo OFF

echo "Setting up server..."

cd server

call npm i

echo "Copying config..."

copy config.sample.json config.json

echo "Setting up client..."

cd ../client

cargo run -- --setup

cd ..

echo "All done!"
