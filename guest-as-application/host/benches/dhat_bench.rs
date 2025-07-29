use host::wamr_manager;

/* 
Setup custom harness
*/
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug)]
struct CustomBenchmark {
    name: &'static str,
    benchmark_fn: fn() -> dhat::HeapStats,
}
impl CustomBenchmark {
    fn run(&self) -> serde_json::Value {
        let heap_stats = (self.benchmark_fn)();
        let measures =
            serde_json::json!({
            "Final Blocks": {
                "value": heap_stats.curr_blocks,
            },
            "Final Bytes": {
                "value": heap_stats.curr_bytes,
            },
            "Max Blocks": {
                "value": heap_stats.max_blocks,
            },
            "Max Bytes": {
                "value": heap_stats.max_bytes,
            },
            "Total Blocks": {
                "value": heap_stats.total_blocks,
            },
            "Total Bytes": {
                "value": heap_stats.total_bytes,
            },
        });
        let mut benchmark_map = serde_json::Map::new();
        benchmark_map.insert(self.name.to_string(), measures);
        benchmark_map.into()
    }
}
inventory::collect!(CustomBenchmark);

macro_rules! bench_fn {
    ($name:ident, $dispname:literal, $code:expr) => {
        fn $name() -> dhat::HeapStats {
            let _profiler = dhat::Profiler::builder().testing().build();
            std::hint::black_box($code);
            dhat::HeapStats::get()
        }

		inventory::submit!(CustomBenchmark {
			name: $dispname,
			benchmark_fn: $name,
		});
    };
}

/* 
Register functions that neet to be profiled
*/
bench_fn!(bench_runtime, "Runtime", {
    wamr_manager::setup_runtime().expect("Bench: Runtime: Runtime Failed");
});

bench_fn!(bench_module, "Module", {
    let runtime = wamr_manager::setup_runtime().expect("Bench: Module: Runtime Failed");
    wamr_manager::setup_module(&runtime).expect("Bench: Module: Module Failed");
});

bench_fn!(bench_instance, "Instance", {
    let runtime = wamr_manager::setup_runtime().expect("Bench: Instance: Runtime Failed");
    let module = wamr_manager::setup_module(&runtime).expect("Bench: Instance: Module Failed");
    wamr_manager
        ::setup_module_instance(&runtime, &module)
        .expect("Bench: Instance: Instance Failed");
});

bench_fn!(bench_full, "Full", {
    let runtime = wamr_manager::setup_runtime().expect("Bench: Full: Runtime Failed");
    let module = wamr_manager::setup_module(&runtime).expect("Bench: Full: Module Failed");
    let instance = wamr_manager
        ::setup_module_instance(&runtime, &module)
        .expect("Bench: Full: Instance Failed");
    wamr_manager::run_guest_function(&instance).expect("Bench: Full: Function Failed");
});

/* 
Run profiler
*/
fn main() {
    let mut bmf = serde_json::Map::new();

    for benchmark in inventory::iter::<CustomBenchmark> {
        let mut results = benchmark.run();
        bmf.append(results.as_object_mut().unwrap());
    }

    let bmf_str = serde_json::to_string_pretty(&bmf).unwrap();
    std::fs::write("results.json", &bmf_str).unwrap();
    println!("{bmf_str}");
}
