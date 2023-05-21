# Start
Start and watch with `cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --debug"`
then run dev server for the frontend with `cd web && rm -R node_modules && npm i && npm run dev`
and the dev server with `cd server && npm run dev`