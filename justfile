scp := "scp.exe"
ssh := "ssh.exe"
host_addr := env_var_or_default("PI_HOST", "wasmpi.local")
user := env_var_or_default("PI_USER", "robin")
pi_arch := "aarch64-unknown-linux-musl"

dir := "guest-as-application"
lib := "wasip1-i2c-lib"

default: (build-p1-guest)

build-p1-guest:
	cd "{{dir}}/wasip1-i2c-guest" && cargo build --target wasm32-wasip1 --release
	mkdir -p "./target/wasmodules"
	cp "{{dir}}/wasip1-i2c-guest/target/wasm32-wasip1/release/wasip1_i2c_guest.wasm" "./target/wasmodules/guestp1.wasm"

build-p2-guest:
	cd "{{dir}}/wasip2-i2c-guest" && cargo component build --target wasm32-wasip2 --release
	mkdir -p "./target/wasmodules"
	cp "{{dir}}/wasip2-i2c-guest/target/wasm32-wasip2/release/wasip2_i2c_guest.wasm" "./target/wasmodules/guestp2.wasm"

wasmtime: (build-p2-guest)
	cd "{{dir}}/wasmtime_impl" && cargo build --target "{{pi_arch}}" --release
	cp "{{dir}}/wasmtime_impl/target/{{pi_arch}}/release/wasmtime_impl" "./target/hostp2"

wamr: (build-p1-guest)
	cd "{{dir}}/wamr_impl" && cargo build --target "{{pi_arch}}" --release
	cp "{{dir}}/wamr_impl/target/{{pi_arch}}/release/wamr_impl" "./target/hostp1"

bench: (build-p1-guest) (build-p2-guest)
	cd "{{dir}}/benchall" && cargo build --all-targets --release --target {{pi_arch}}
	find "{{dir}}/benchall/target/{{pi_arch}}/release/deps/" -type f -executable -name "*bench*" -exec sh -c 'dest=$(basename "{}" | awk -F"-" "{print \$1}"); cp "{}" "./target/$dest"' \;
	cp "{{dir}}/benchall/target/{{pi_arch}}/release/bench*" "./target/"


deploy-pi:
	{{scp}} -o StrictHostKeyChecking=no "./target/host*" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
	{{scp}} -o StrictHostKeyChecking=no "./target/bench*" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
	{{scp}} -o StrictHostKeyChecking=no "./target/wasmodules/guest*.wasm" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/wasmodules/
	{{ssh}} -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "chmod +x ~/masterproef/host*"







# build-bench-pi: (build-guest)
# 	cd "{{dir}}/{{host}}" && cargo bench --no-run --benches --target "{{pi_arch}}"
# 	find "{{dir}}/{{host}}/target/{{pi_arch}}/release/deps/" -type f -executable -name "*bench*" -exec sh -c 'dest=$(basename "{}" | awk -F"-" "{print \$1}"); cp "{}" "./target/$dest"' \;

# deploy-bench: (build-bench-pi)
# 	scp.exe -o StrictHostKeyChecking=no ./target/*bench* "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
# 	scp.exe -o StrictHostKeyChecking=no "./target/wasmodules/{{guest}}.wasm" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/wasmodules/
# 	ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "chmod +x ./masterproef/*bench*"


clean:
	cd "{{dir}}/wasip1-i2c-guest" && cargo clean || true
	cd "{{dir}}/wasip2-i2c-guest" && cargo clean || true
	cd "{{dir}}/wamr_impl" && cargo clean || true
	cd "{{dir}}/wasmtime_impl" && cargo clean || true
	rm -rf "./target/"

pi-logs:
	ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "journalctl -f"
