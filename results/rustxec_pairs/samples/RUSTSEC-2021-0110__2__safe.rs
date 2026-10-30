pub fn table_ops(
    fuzz_config: crate::generators::Config,
    ops: crate::generators::table_ops::TableOps,
) {
    let _ = env_logger::try_init();

    let expected_drops = Arc::new(AtomicUsize::new(ops.num_params() as usize));
    let num_dropped = Arc::new(AtomicUsize::new(0));

    {
        let mut config = fuzz_config.to_wasmtime();
        config.wasm_reference_types(true);
        config.consume_fuel(true);

        let engine = Engine::new(&config).unwrap();
        let mut store = create_store(&engine);
        store.add_fuel(100).unwrap();

        let wasm = ops.to_wasm_binary();
        log_wasm(&wasm);
        let module = match Module::new(&engine, &wasm) {
            Ok(m) => m,
            Err(_) => return,
        };

        let mut linker = Linker::new(&engine);

        // To avoid timeouts, limit the number of explicit GCs we perform per
        // test case.
        const MAX_GCS: usize = 5;

        let num_gcs = AtomicUsize::new(0);
        linker
            .define(
                "",
                "gc",
                // NB: use `Func::new` so that this can still compile on the old x86
                // backend, where `IntoFunc` isn't implemented for multi-value
                // returns.
                Func::new(
                    &mut store,
                    FuncType::new(
                        vec![],
                        vec![ValType::ExternRef, ValType::ExternRef, ValType::ExternRef],
                    ),
                    {
                        let num_dropped = num_dropped.clone();
                        let expected_drops = expected_drops.clone();
                        move |mut caller: Caller<'_, StoreLimits>, _params, results| {
                            if num_gcs.fetch_add(1, SeqCst) < MAX_GCS {
                                caller.gc();
                            }

                            expected_drops.fetch_add(3, SeqCst);
                            results[0] =
                                Some(ExternRef::new(CountDrops(num_dropped.clone()))).into();
                            results[1] =
                                Some(ExternRef::new(CountDrops(num_dropped.clone()))).into();
                            results[2] =
                                Some(ExternRef::new(CountDrops(num_dropped.clone()))).into();
                            Ok(())
                        }
                    },
                ),
            )
            .unwrap();

        linker
            .func_wrap("", "take_refs", {
                let expected_drops = expected_drops.clone();
                move |a: Option<ExternRef>, b: Option<ExternRef>, c: Option<ExternRef>| {
                    // Do the assertion on each ref's inner data, even though it
                    // all points to the same atomic, so that if we happen to
                    // run into a use-after-free bug with one of these refs we
                    // are more likely to trigger a segfault.
                    if let Some(a) = a {
                        let a = a.data().downcast_ref::<CountDrops>().unwrap();
                        assert!(a.0.load(SeqCst) <= expected_drops.load(SeqCst));
                    }
                    if let Some(b) = b {
                        let b = b.data().downcast_ref::<CountDrops>().unwrap();
                        assert!(b.0.load(SeqCst) <= expected_drops.load(SeqCst));
                    }
                    if let Some(c) = c {
                        let c = c.data().downcast_ref::<CountDrops>().unwrap();
                        assert!(c.0.load(SeqCst) <= expected_drops.load(SeqCst));
                    }
                }
            })
            .unwrap();

        linker
            .define(
                "",
                "make_refs",
                // NB: use `Func::new` so that this can still compile on the old
                // x86 backend, where `IntoFunc` isn't implemented for
                // multi-value returns.
                Func::new(
                    &mut store,
                    FuncType::new(
                        vec![],
                        vec![ValType::ExternRef, ValType::ExternRef, ValType::ExternRef],
                    ),
                    {
                        let num_dropped = num_dropped.clone();
                        let expected_drops = expected_drops.clone();
                        move |_caller, _params, results| {
                            expected_drops.fetch_add(3, SeqCst);
                            results[0] =
                                Some(ExternRef::new(CountDrops(num_dropped.clone()))).into();
                            results[1] =
                                Some(ExternRef::new(CountDrops(num_dropped.clone()))).into();
                            results[2] =
                                Some(ExternRef::new(CountDrops(num_dropped.clone()))).into();
                            Ok(())
                        }
                    },
                ),
            )
            .unwrap();

        let instance = linker.instantiate(&mut store, &module).unwrap();
        let run = instance.get_func(&mut store, "run").unwrap();

        let args: Vec<_> = (0..ops.num_params())
            .map(|_| Val::ExternRef(Some(ExternRef::new(CountDrops(num_dropped.clone())))))
            .collect();
        let _ = run.call(&mut store, &args);
    }

    assert_eq!(num_dropped.load(SeqCst), expected_drops.load(SeqCst));
    return;

    struct CountDrops(Arc<AtomicUsize>);

    impl Drop for CountDrops {
        fn drop(&mut self) {
            self.0.fetch_add(1, SeqCst);
        }
    }
}