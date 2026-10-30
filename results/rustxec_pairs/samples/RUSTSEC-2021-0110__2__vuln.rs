pub fn table_ops(
    fuzz_config: crate::generators::Config,
    ops: crate::generators::table_ops::TableOps,
) {
    let _ = env_logger::try_init();

    let num_dropped = Arc::new(AtomicUsize::new(0));

    {
        let mut config = fuzz_config.to_wasmtime();
        config.wasm_reference_types(true);
        let engine = Engine::new(&config).unwrap();
        let mut store = create_store(&engine);
        if fuzz_config.consume_fuel {
            store.add_fuel(u64::max_value()).unwrap();
        }

        let wasm = ops.to_wasm_binary();
        log_wasm(&wasm);
        let module = match Module::new(&engine, &wasm) {
            Ok(m) => m,
            Err(_) => return,
        };

        // To avoid timeouts, limit the number of explicit GCs we perform per
        // test case.
        const MAX_GCS: usize = 5;

        let num_gcs = AtomicUsize::new(0);
        let gc = Func::wrap(&mut store, move |mut caller: Caller<'_, StoreLimits>| {
            if num_gcs.fetch_add(1, SeqCst) < MAX_GCS {
                caller.gc();
            }
        });

        let instance = Instance::new(&mut store, &module, &[gc.into()]).unwrap();
        let run = instance.get_func(&mut store, "run").unwrap();

        let args: Vec<_> = (0..ops.num_params())
            .map(|_| Val::ExternRef(Some(ExternRef::new(CountDrops(num_dropped.clone())))))
            .collect();
        let _ = run.call(&mut store, &args);
    }

    assert_eq!(num_dropped.load(SeqCst), ops.num_params() as usize);
    return;

    struct CountDrops(Arc<AtomicUsize>);

    impl Drop for CountDrops {
        fn drop(&mut self) {
            self.0.fetch_add(1, SeqCst);
        }
    }
}