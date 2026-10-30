fn imm_logic_from_u64(&mut self, ty: Type, n: u64) -> Option<ImmLogic> {
        let ty = if ty.bits() < 32 { I32 } else { ty };
        ImmLogic::maybe_from_u64(n, ty)
    }