pub fn to_wasm_binary(&self) -> Vec<u8> {
        let mut module = Module::new();

        // Import the GC function.
        let mut imports = ImportSection::new();
        imports.import("", Some("gc"), EntityType::Function(0));

        // Define our table.
        let mut tables = TableSection::new();
        tables.table(TableType {
            element_type: ValType::ExternRef,
            minimum: self.table_size(),
            maximum: None,
        });

        // Encode the types for all functions that we are using.
        let mut types = TypeSection::new();
        types.function(vec![], vec![]); // 0: "gc"
        let mut params: Vec<ValType> = Vec::with_capacity(self.num_params() as usize);
        for _i in 0..self.num_params() {
            params.push(ValType::ExternRef);
        }
        let results = vec![];
        types.function(params, results); // 1: "run"

        // Define the "run" function export.
        let mut functions = FunctionSection::new();
        functions.function(1);

        let mut exports = ExportSection::new();
        exports.export("run", Export::Function(1));

        let mut params: Vec<(u32, ValType)> = Vec::with_capacity(self.num_params() as usize);
        for _i in 0..self.num_params() {
            params.push((0, ValType::ExternRef));
        }
        let mut func = Function::new(params);

        for op in self.ops.iter().take(MAX_OPS) {
            op.insert(&mut func);
        }

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