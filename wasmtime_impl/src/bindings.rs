use wasmtime::component::*;

bindgen!({
	world: "pingpong",
	path: "../wit"
});
