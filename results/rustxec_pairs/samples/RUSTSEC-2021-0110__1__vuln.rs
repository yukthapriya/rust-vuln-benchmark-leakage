fn try_from(ty: wasmparser::FuncType) -> Result<Self, Self::Error> {
        Ok(Self {
            params: ty
                .params
                .into_vec()
                .into_iter()
                .map(WasmType::try_from)
                .collect::<Result<_, Self::Error>>()?,
            returns: ty
                .returns
                .into_vec()
                .into_iter()
                .map(WasmType::try_from)
                .collect::<Result<_, Self::Error>>()?,
        })
    }