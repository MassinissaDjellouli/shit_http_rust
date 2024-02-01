mkdir -p release
cargo build --release
mv target/release/shit_http_rust.exe ../release/