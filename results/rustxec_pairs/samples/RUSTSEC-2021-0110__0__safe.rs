pub fn to_wasm_binary(&self) -> Vec<u8> {
        let mut module = Module::new();

        // Encode the types for all functions that we are using.
        let mut types = TypeSection::new();

        // 0: "gc"
        types.function(
            vec![],
            // Return a bunch of stuff from `gc` so that we exercise GCing when
            // there is return pointer space allocated on the stack. This is
            // especially important because the x64 backend currently
            // dynamically adjusts the stack pointer for each call that uses
            // return pointers rather than statically allocating space in the
            // stack frame.
            vec![ValType::ExternRef, ValType::ExternRef, ValType::ExternRef],
        );

        // 1: "run"
        let mut params: Vec<ValType> = Vec::with_capacity(self.num_params() as usize);
        for _i in 0..self.num_params() {
            params.push(ValType::ExternRef);
        }
        let results = vec![];
        types.function(params, results);

        // 2: `take_refs`
        types.function(
            vec![ValType::ExternRef, ValType::ExternRef, ValType::ExternRef],
            vec![],
        );

        // 3: `make_refs`
        types.function(
            vec![],
            vec![ValType::ExternRef, ValType::ExternRef, ValType::ExternRef],
        );

        // Import the GC function.
        let mut imports = ImportSection::new();
        imports.import("", Some("gc"), EntityType::Function(0));
        imports.import("", Some("take_refs"), EntityType::Function(2));
        imports.import("", Some("make_refs"), EntityType::Function(3));

        // Define our table.
        let mut tables = TableSection::new();
        tables.table(TableType {
            element_type: ValType::ExternRef,
            minimum: self.table_size(),
            maximum: None,
        });

        // Define the "run" function export.
        let mut functions = FunctionSection::new();
        functions.function(1);

        let mut exports = ExportSection::new();
        exports.export("run", Export::Function(3));

        // Give ourselves one scratch local that we can use in various `TableOp`
        // implementations.
        let mut func = Function::new(vec![(1, ValType::ExternRef)]);

        func.instruction(Instruction::Loop(wasm_encoder::BlockType::Empty));
        for op in self.ops.iter().take(MAX_OPS) {
            op.insert(&mut func, self.num_params() as u32, self.table_size());
        }
        func.instruction(Instruction::Br(0));
        func.instruction(Instruction::End);
        func.instruction(Instruction::End);

        let mut code = CodeSection::new();
        code.function(&func);

        module
            .section(&types)
            .section(&imports)
            .section(&functions)
            .section(&tables)
            .section(&exports)
            .section(&code);

        module.finish()
    }