# build it!

cd "$(dirname "$0")/.."
wasm-pack build --target web --out-dir wasm-build/pkg --no-typescript