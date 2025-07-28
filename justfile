pi_host_addr := env_var_or_default("PI_HOST", "wasmpi.local")
pi_user := env_var_or_default("PI_USER", "robin")
pi_arch := "aarch64-unknown-linux-musl"

d_dir := "guest-as-application"
d_guest := "guest"
d_host := "host"

default: (build-local)

build-guest dir=d_dir guest=d_guest:
	cd "{{dir}}/{{guest}}" && cargo build --target wasm32-wasip1 --release
	mkdir -p "./target/wasmodules"
	cp "{{dir}}/{{guest}}/target/wasm32-wasip1/release/{{guest}}.wasm" "./target/wasmodules/"

build-local dir=d_dir guest=d_guest host=d_host: (build-guest dir guest)
	cd "{{dir}}/{{host}}" && cargo build --release
	cp "{{dir}}/{{host}}/target/release/{{host}}" "./target/"

build-pi dir=d_dir guest=d_guest host=d_host: (build-guest dir guest)
	cd "{{dir}}/{{host}}" && cargo build --target "{{pi_arch}}" --release
	cp "{{dir}}/{{host}}/target/{{pi_arch}}/release/{{host}}" "./target/"

deploy host_addr=pi_host_addr user=pi_user dir=d_dir guest=d_guest host=d_host: (build-pi dir guest host)
	scp.exe -o StrictHostKeyChecking=no "./target/{{host}}" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/
	scp.exe -o StrictHostKeyChecking=no "./target/wasmodules/{{guest}}.wasm" "{{user}}"@"{{host_addr}}":/home/"{{user}}"/masterproef/wasmodules
	ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "chmod +x ./masterproef/{{host}}"

run-pi host_addr=pi_host_addr user=pi_user dir=d_dir guest=d_guest host=d_host: (deploy host_addr user dir guest host)
	@echo "\n===================\nStarting program...\n===================\n"
	@ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host_addr}}" "cd /home/{{user}}/masterproef/ && ./{{host}}"

run-local dir=d_dir guest=d_guest host=d_host: (build-local dir guest host)
	cd "target" && "./{{host}}"

driver: (run-local "guest-as-driver")
app: (run-local)

clean dir=d_dir guest=d_guest host=d_host:
	cd "{{dir}}/{{guest}}" && cargo clean || true
	cd "{{dir}}/{{host}}" && cargo clean || true
	rm -f "./target/wasmodules/*.wasm"
	rm -f "./target/{{host}}"

logs host=pi_host_addr user=pi_user:
	ssh.exe -o StrictHostKeyChecking=no "{{user}}"@"{{host}}" "journalctl -f"
