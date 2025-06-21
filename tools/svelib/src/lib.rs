use std::collections::HashMap;

use bincode::config::{Configuration, Fixint, LittleEndian, NoLimit};
use object::{Object, ObjectSection, ObjectSymbol, RelocationKind};
use sve::{
    SVEBlobDescriptor, SVEExportDescriptor, SVEHeader, SVEImportDescriptor, SVERelocDescriptor,
    SVERelocType, SVESectionDescriptor,
};

type BincodeConfig = Configuration<LittleEndian, Fixint, NoLimit>;

fn convert_section(object: &object::File<'_>, section: &object::Section<'_, '_>, offset: u32) -> SVESectionDescriptor {
    let range = section.file_range().unwrap();
    SVESectionDescriptor {
        base: (section.address() - object.relative_address_base()) as u32,
        size: section.size() as u32,
        offset,
        raw_size: (range.1 - range.0) as u32
    }
}

fn build_header_and_sections(
    object: &object::File<'_>,
    strings_size: u32,
    relocs_size: u32,
    imports_size: u32,
    import_ptrs_size: u32,
    exports_size: u32,
    config: &BincodeConfig,
) -> Vec<u8> {
    let code_section = object.section_by_name(".text").expect("failed to get .text section!");
    let data_section = object.section_by_name(".data").expect("failed to get .data section!");
    let rodata_section = object.section_by_name(".rdata").expect("failed to get .rdata section!");
    let zdata_section = object.section_by_name(".bss").expect("failed to get .bss section!");

    let mut offset = size_of::<SVEHeader>() as u32;
    let sve_code = convert_section(object, &code_section, offset);
    offset += sve_code.raw_size;

    let mut header = SVEHeader {
        magic: sve::SVE_MAGIC,
        revision: sve::SVE_REVISION,
        machine: sve::SVEMachine::RISCV,
        entry: (object.entry() - code_section.address()) as u32,
        code: todo!(),
        data: todo!(),
        rodata: todo!(),
        zdata: todo!(),
        strings: todo!(),
        relocs: todo!(),
        imports: todo!(),
        import_ptrs: todo!(),
        exports: todo!(),
    };
}

fn build_relocs(object: &object::File, config: &BincodeConfig) -> Vec<u8> {
    let mut reloc_table = vec![];
    if let Some(relocs) = object.dynamic_relocations() {
        for (addr, reloc) in relocs {
            // resolve absolute relocations
            let reloc_type = match reloc.kind() {
                RelocationKind::Absolute => {
                    if reloc.size() == 4 {
                        SVERelocType::Absolute32
                    } else if reloc.size() == 8 {
                        SVERelocType::Absolute64
                    } else {
                        SVERelocType::Unknown
                    }
                }
                RelocationKind::Relative => SVERelocType::Relative,
                RelocationKind::PltRelative => SVERelocType::Imported,
                _ => SVERelocType::Unknown,
            };

            let desc = SVERelocDescriptor {
                addr: addr as u32,
                reloc_type,
            };
            let mut desc_raw = bincode::encode_to_vec(desc, config.clone())
                .expect("failed to encode relocation descriptor!");
            reloc_table.append(&mut desc_raw);
        }
    }

    reloc_table
}

fn build_imports(
    object: &object::File,
    config: &BincodeConfig,
    strings: &mut Vec<u8>,
) -> (Vec<u8>, u32) {
    let mut import_ptrs_size = 0;
    let mut imports = vec![];
    object
        .imports()
        .expect("failed to get imports")
        .iter()
        .for_each(|import| {
            let sve_import = SVEImportDescriptor {
                lib_name: add_string_raw(import.library(), strings),
                sym_name: add_string_raw(import.name(), strings),
                ptr_offset: import_ptrs_size,
            };
            let mut import_data = bincode::encode_to_vec(sve_import, config.clone())
                .expect("failed to encode import descriptor!");
            imports.append(&mut import_data);
            import_ptrs_size += 8;
        });

    (imports, import_ptrs_size)
}

fn build_exports(object: &object::File, config: &BincodeConfig, strings: &mut Vec<u8>) -> Vec<u8> {
    let mut exports = vec![];

    object.dynamic_symbols().for_each(|sym| {
        if sym.is_global()
            && let Ok(name) = sym.name()
        {
            let sym_name = add_string(name, strings);
            let export = SVEExportDescriptor {
                sym_name,
                addr: sym.address() as u32,
            };

            let mut export_raw = bincode::encode_to_vec(export, config.clone())
                .expect("failed to encode export descriptor!");
            exports.append(&mut export_raw);
        }
    });

    exports
}

fn add_string(string: &str, strings: &mut Vec<u8>) -> u32 {
    let offset = strings.len() as u32;
    strings.append(&mut Vec::from(string));
    offset
}

fn add_string_raw(string: &[u8], strings: &mut Vec<u8>) -> u32 {
    let offset = strings.len() as u32;
    let mut vec = string.to_vec();
    strings.append(&mut vec);
    offset
}

pub fn build_sve(object: &object::File) -> Vec<u8> {
    let config: BincodeConfig = Default::default();

    let mut strings = vec![];
    let relocs = build_relocs(object, &config);
    let (imports, import_ptrs_size) = build_imports(object, &config, &mut strings);
    let exports = build_exports(object, &config, &mut strings);

    let header = build_header_and_sections(
        &object,
        &strings,
        &relocs,
        &imports,
        import_ptrs_size,
        &exports,
        &config,
    );

    let data = vec![header, relocs, exports, strings];
}
