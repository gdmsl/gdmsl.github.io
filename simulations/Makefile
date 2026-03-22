.PHONY: all quantum-walk ising rydberg tweezer clean

all: quantum-walk ising rydberg tweezer

quantum-walk:
	wasm-pack build sim-quantum-walk --target web --release \
	  --out-dir ../../static/wasm/quantum-walk --no-typescript
	rm -f ../../static/wasm/quantum-walk/.gitignore ../../static/wasm/quantum-walk/package.json ../../static/wasm/quantum-walk/README.md

ising:
	wasm-pack build sim-ising --target web --release \
	  --out-dir ../../static/wasm/ising --no-typescript
	rm -f ../../static/wasm/ising/.gitignore ../../static/wasm/ising/package.json ../../static/wasm/ising/README.md

rydberg:
	wasm-pack build sim-rydberg --target web --release \
	  --out-dir ../../static/wasm/rydberg --no-typescript
	rm -f ../../static/wasm/rydberg/.gitignore ../../static/wasm/rydberg/package.json ../../static/wasm/rydberg/README.md

tweezer:
	wasm-pack build sim-tweezer --target web --release \
	  --out-dir ../../static/wasm/tweezer --no-typescript
	rm -f ../../static/wasm/tweezer/.gitignore ../../static/wasm/tweezer/package.json ../../static/wasm/tweezer/README.md

clean:
	cargo clean
	rm -rf ../static/wasm/quantum-walk ../static/wasm/ising ../static/wasm/rydberg ../static/wasm/tweezer
