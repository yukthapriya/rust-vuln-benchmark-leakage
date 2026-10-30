fn imm_logic_from_imm64(&mut self, ty: Type, n: Imm64) -> Option<ImmLogic> {
        self.imm_logic_from_u64(ty, n.bits() as u64)
    }