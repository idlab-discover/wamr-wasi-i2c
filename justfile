host_addr := env_var_or_default("PI_HOST", "wasmpi.local")
user := env_var_or_default("PI_USER", "robin")
pi_arch := "aarch64-unknown-linux-musl"

dir := "guest-as-application"
guest := "guest"
host := "wamr_impl"
lib := "wasip1-i2c"

default: (build-guest)

build-guest:
	cd "{{dir}}/{{guest}}" && cargo build --target wasm32-wasip1 --release
	mkdir -p "./target/wasmodules"
	cp "{{dir}}/{{guest}}/target/wasm32-wasip1/release/{{guest}}.wasm" "./target/wasmodules/guest.wasm"

build-pi: (build-guest)
	cd "{{dir}}/{{host}}" && cargo build --target "{{pi_arch}}" --release
	cp "{{dir}}/{{host}}/target/{{pi_arch}}/release/{{host}}" "./target/host"

deploy-pi: (build-pi)
	scp.exe -o StrictHostKeyChecking=no "./target/host" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
	scp.exe -o StrictHostKeyChecking=no "./target/wasmodules/guest.wasm" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/wasmodules/
	ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "chmod +x ~/masterproef/host"

run-pi: (deploy-pi)
	@echo "\n===================\nStarting program...\n===================\n"
	@ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "cd /home/{{user}}/masterproef/ && ./host"




build-local: (build-guest)
	cd "{{dir}}/host" && cargo build --release
	cp "{{dir}}/{{host}}/target/release/{{host}}" "./target/host"

run-local: (build-local)
	cd "target" && "./host"

# build-bench-pi: (build-guest)
# 	cd "{{dir}}/{{host}}" && cargo bench --no-run --benches --target "{{pi_arch}}"
# 	find "{{dir}}/{{host}}/target/{{pi_arch}}/release/deps/" -type f -executable -name "*bench*" -exec sh -c 'dest=$(basename "{}" | awk -F"-" "{print \$1}"); cp "{}" "./target/$dest"' \;

# deploy-bench: (build-bench-pi)
# 	scp.exe -o StrictHostKeyChecking=no ./target/*bench* "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
# 	scp.exe -o StrictHostKeyChecking=no "./target/wasmodules/{{guest}}.wasm" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/wasmodules/
# 	ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "chmod +x ./masterproef/*bench*"


clean:
	cd "{{dir}}/{{guest}}" && cargo clean || true
	cd "{{dir}}/{{host}}" && cargo clean || true
	cd "{{dir}}/{{d_lib}}" && cargo clean || true
	rm -rf "./target/"

pi-logs:
	ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "journalctl -f"
