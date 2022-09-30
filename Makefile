build:
	cargo build --release
	upx target/release/marketprice.exe --lzma -9

dev:
	cargo run

clean:
	cargo clean
