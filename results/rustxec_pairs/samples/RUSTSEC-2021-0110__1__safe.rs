fn try_from(ty: wasmparser::FuncType) -> Result<Self, Self::Error> {
        let params = ty
            .params
            .into_vec()
            .into_iter()
            .map(WasmType::try_from)
            .collect::<Result<_, Self::Error>>()?;
        let returns = ty
            .returns
            .into_vec()
            .into_iter()
            .map(WasmType::try_from)
            .collect::<Result<_, Self::Error>>()?;
        Ok(Self::new(params, returns))
    }