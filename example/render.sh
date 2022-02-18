EXAMPLE="$PWD"
cd .. 
cargo run "$EXAMPLE/test_blends" "$EXAMPLE/resources/renders" 128 128 8 -o
cd .. 
cd example
cargo run
