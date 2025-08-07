scp := "scp.exe"
ssh := "ssh.exe"
host_addr := env_var_or_default("PI_HOST", "wasmpi.local")
user := env_var_or_default("PI_USER", "robin")
pi_arch := env_var_or_default("PI_ARCH", "aarch64-unknown-linux-musl")

default:
    @just --list

# Build a WASM Module
[group('Creation: Guest')]
build-p1:
	cd "wasip1-i2c-guest" && cargo build --target wasm32-wasip1 --release
	mkdir -p "./target/wasmodules"
	cp "wasip1-i2c-guest/target/wasm32-wasip1/release/wasip1_i2c_guest.wasm" "./target/wasmodules/guestp1.wasm"

# Build a WASM Component
[group('Creation: Guest')]
build-p2:
	cd "wasip2-i2c-guest" && cargo component build --target wasm32-wasip2 --release
	mkdir -p "./target/wasmodules"
	cp "wasip2-i2c-guest/target/wasm32-wasip2/release/wasip2_i2c_guest.wasm" "./target/wasmodules/guestp2.wasm"

# Build all WASM Guest implementations
[group('Creation: Guest')]
guests: (build-p1) (build-p2)

# Build the Native Host Implementation for the PI Architecture
[group('Creation: Host')]
native:
	cd "native_impl" && cargo build --target "{{pi_arch}}" --release
	mkdir -p "./target/wasmodules"
	cp "native_impl/target/{{pi_arch}}/release/native_impl" "./target/hostn"

# Build the WAMR Host Implementation for the PI Architecture
[group('Creation: Host')]
wamr: (build-p1)
	cd "wamr_impl" && cargo build --target "{{pi_arch}}" --release
	cp "wamr_impl/target/{{pi_arch}}/release/wamr_run" "./target/hostp1"

# Build the WASMTIME Host Implementation for the PI Architecture
[group('Creation: Host')]
wasmtime: (build-p2)
	cd "wasmtime_impl" && cargo build --target "{{pi_arch}}" --release -j 4
	cp "wasmtime_impl/target/{{pi_arch}}/release/wasmtime_run" "./target/hostp2"

# Build all Host implementations
[group('Creation: Host')]
hosts: (native) (wamr) (wasmtime)

# Build the BENCH Implementation to profile all the implementations
[group('Creation: Host')]
bench: (build-p1) (build-p2)
	mkdir -p "./target/benches"
	cd "benchall" && cargo build --all-targets --release --target {{pi_arch}} --features dhat-pingpong,dhat-runtime
	find "benchall/target/{{pi_arch}}/release/deps/" -type f -executable -name "bench*" -exec cp "{}" ./target/benches/ \;
	find "benchall/target/{{pi_arch}}/release/" -maxdepth 1 -type f -executable -name "bench*" -exec cp "{}" ./target/ \;

# Build an implementation that just runs all three ways to do the Ping Pong
[group('Creation: Host')]
pingpong: (build-p1) (build-p2)
	cd "benchall" && cargo build --bin bench_dhat --release --target {{pi_arch}}
	cp "benchall/target/{{pi_arch}}/release/bench_dhat" "./target/pingpong_all_three"

# Build a binary that generates the Flamegraph
[group('Creation: Host')]
flamegraph: (build-p1) (build-p2)
	cd "benchall" && cargo build --bin bench_dhat --release --target {{pi_arch}} --features pprof-flamegraph
	cp "benchall/target/{{pi_arch}}/release/bench_dhat" "./target/flamegraph"

# Generate binaries for benchall, WAMR and Wasmtime bins and benches
[group('Creation: Host')]
all: (bench) (wasmtime) (wamr) (pingpong) (flamegraph)


# Deploy all the previously compiled host binaries and WASM Guests to the PI and make them executable
[group('Other')]
deploy:
	{{scp}} -r "./target/" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
	-{{ssh}} "{{user}}"@"{{host_addr}}" 'find ./masterproef/target/ -type f -exec chmod +x "{}" +'




# Cargo clean all the different projects
[group('Other')]
deepclean: (clean)
	-cd "wasip1-i2c-lib" && cargo clean
	-cd "wasip1-i2c-guest" && cargo clean
	-cd "wasip2-i2c-guest" && cargo clean
	-cd "wamr_impl" && cargo clean
	-cd "wasmtime_impl" && cargo clean
	-cd "native_impl" && cargo clean
	-cd "benchall" && cargo clean

# Alias for deepclean
[group('Other')]
deep: (deepclean)

# Removes compiled files to be sent to the PI
[group('Other')]
clean:
	rm -rf "./target/"
