pub fn get_data(&self, elf_file: &ElfFile<'a>) -> Result<SectionData<'a>, &'static str> {
        macro_rules! array_data {
            ($data32: ident, $data64: ident) => {{
                let data = self.raw_data(elf_file);
                match elf_file.header.pt1.class() {
                    Class::ThirtyTwo => SectionData::$data32(read_array(data)),
                    Class::SixtyFour => SectionData::$data64(read_array(data)),
                    Class::None | Class::Other(_) => unreachable!(),
                }
            }}
        }

        self.get_type().and_then(|typ| Ok(match typ {
            ShType::Null | ShType::NoBits => SectionData::Empty,
            ShType::ProgBits |
            ShType::ShLib |
            ShType::OsSpecific(_) |
            ShType::ProcessorSpecific(_) |
            ShType::User(_) => SectionData::Undefined(self.raw_data(elf_file)),
            ShType::SymTab => array_data!(SymbolTable32, SymbolTable64),
            ShType::DynSym => array_data!(DynSymbolTable32, DynSymbolTable64),
            ShType::StrTab => SectionData::StrArray(self.raw_data(elf_file)),
            ShType::InitArray | ShType::FiniArray | ShType::PreInitArray => {
                array_data!(FnArray32, FnArray64)
            }
            ShType::Rela => array_data!(Rela32, Rela64),
            ShType::Rel => array_data!(Rel32, Rel64),
            ShType::Dynamic => array_data!(Dynamic32, Dynamic64),
            ShType::Group => {
                let data = self.raw_data(elf_file);
                unsafe {
                    let flags: &'a u32 = mem::transmute(&data[0]);
                    let indicies: &'a [u32] = read_array(&data[4..]);
                    SectionData::Group {
                        flags,
                        indicies,
                    }
                }
            }
            ShType::SymTabShIndex => {
                SectionData::SymTabShIndex(read_array(self.raw_data(elf_file)))
            }
            ShType::Note => {
                let data = self.raw_data(elf_file);
                match elf_file.header.pt1.class() {
                    Class::ThirtyTwo => return Err("32-bit binaries not implemented"),
                    Class::SixtyFour => {
                        let header: &'a NoteHeader = read(&data[0..12]);
                        let index = &data[12..];
                        SectionData::Note64(header, index)
                    }
                    Class::None | Class::Other(_) => return Err("Unknown ELF class"),
                }
            }
            ShType::Hash => {
                let data = self.raw_data(elf_file);
                SectionData::HashTable(HashTable::read(data))
            }
        }))
    }