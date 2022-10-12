RFLAGS="-C link-arg=-s"

all: token escrow

token: contracts/rpc-token
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p rpc-token --target wasm32-unknown-unknown --release
	@mkdir -p res
	cp target/wasm32-unknown-unknown/release/rpc_token.wasm ./res/

escrow: contracts/rpc-escrow
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p rpc-escrow --target wasm32-unknown-unknown --release
	@mkdir -p res
	cp target/wasm32-unknown-unknown/release/rpc_escrow.wasm ./res/
