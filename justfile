scp := "scp.exe"
ssh := "ssh.exe"
host_addr := env_var_or_default("PI_HOST", "wasmpi.local")
user := env_var_or_default("PI_USER", "robin")
pi_arch := env_var_or_default("PI_ARCH", "aarch64-unknown-linux-musl")

dir := "guest-as-application"

default:
    @just --list

# Build a WASM Module
[group('Creation: Guest')]
build-p1-guest:
	cd "{{dir}}/wasip1-i2c-guest" && cargo build --target wasm32-wasip1 --release
	mkdir -p "./target/wasmodules"
	cp "{{dir}}/wasip1-i2c-guest/target/wasm32-wasip1/release/wasip1_i2c_guest.wasm" "./target/wasmodules/guestp1.wasm"

# Build a WASM Component
[group('Creation: Guest')]
build-p2-guest:
	cd "{{dir}}/wasip2-i2c-guest" && cargo component build --target wasm32-wasip2 --release
	mkdir -p "./target/wasmodules"
	cp "{{dir}}/wasip2-i2c-guest/target/wasm32-wasip2/release/wasip2_i2c_guest.wasm" "./target/wasmodules/guestp2.wasm"

# Build the Native Host Implementation for the PI Architecture
[group('Creation: Host')]
native:
	cd "{{dir}}/native_impl" && cargo build --target "{{pi_arch}}" --release
	mkdir -p "./target/wasmodules"
	cp "{{dir}}/native_impl/target/{{pi_arch}}/release/native_impl" "./target/hostn"

# Build the WAMR Host Implementation for the PI Architecture
[group('Creation: Host')]
wamr: (build-p1-guest)
	cd "{{dir}}/wamr_impl" && cargo build --target "{{pi_arch}}" --release
	cp "{{dir}}/wamr_impl/target/{{pi_arch}}/release/wamr_impl" "./target/hostp1"

# Build the WASMTIME Host Implementation for the PI Architecture
[group('Creation: Host')]
wasmtime: (build-p2-guest)
	cd "{{dir}}/wasmtime_impl" && cargo build --target "{{pi_arch}}" --release -j 4
	cp "{{dir}}/wasmtime_impl/target/{{pi_arch}}/release/wasmtime_impl" "./target/hostp2"

# Build the BENCH Implementation to profile all the implementations
[group('Creation: Host')]
bench: (build-p1-guest) (build-p2-guest)
	mkdir -p "./target/benches"
	cd "{{dir}}/benchall" && cargo bench --no-run --benches --target "{{pi_arch}}"
	find "{{dir}}/benchall/target/{{pi_arch}}/release/deps/" -type f -executable -name "*bench*" -exec sh -c 'dest=$(basename "{}" | awk -F"-" "{print \$1}"); cp "{}" "./target/benches/$dest"' \;
	
	cd "{{dir}}/benchall" && cargo build --bins --release --target {{pi_arch}} --features dhat-heap
	find "{{dir}}/benchall/target/{{pi_arch}}/release/" -maxdepth 1 -type f -executable -name "bench*" -exec sh -c 'cp "{}" "./target/"' \;

# Deploy all the previously compiled host binaries and WASM Guests to the PI and make them executable
[group('Other')]
deploy:
	{{scp}} -r "./target/" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
	-{{ssh}} "{{user}}"@"{{host_addr}}" 'find ./masterproef/target/ -type f -exec chmod +x "{}" +'




# Cargo clean all the different projects
[group('Other')]
deepclean: (clean)
	-cd "{{dir}}/wasip1-i2c-lib" && cargo clean
	-cd "{{dir}}/wasip1-i2c-guest" && cargo clean
	-cd "{{dir}}/wasip2-i2c-guest" && cargo clean
	-cd "{{dir}}/wamr_impl" && cargo clean
	-cd "{{dir}}/wasmtime_impl" && cargo clean
	-cd "{{dir}}/native_impl" && cargo clean
	-cd "{{dir}}/benchall" && cargo clean

# Removes compiled files to be sent to the PI
[group('Other')]
clean:
	rm -rf "./target/"
