use bincode::config::{Configuration, Fixint, LittleEndian, NoLimit};
use object::{Object, ObjectSection, ObjectSymbol, RelocationKind};
use sve::{
    SVEBlobDescriptor, SVEExportDescriptor, SVEHeader, SVEImportDescriptor, SVERelocDescriptor,
    SVERelocType,
};

type BincodeConfig = Configuration<LittleEndian, Fixint, NoLimit>;

// TODO: propagate errors

fn convert_section(
    object: &object::File<'_>,
    section: &Option<object::Section<'_, '_>>,
    base: &mut u32,
    offset: &mut u32,
) -> Option<SVEBlobDescriptor> {
    if let Some(section) = section {
        let range = section.file_range().unwrap_or((0, 0));
        let blob = SVEBlobDescriptor {
            base: (section.address() - object.relative_address_base()) as u32,
            size: section.size() as u32,
            offset: *offset,
            raw_size: range.1 as u32,
        };
        *base = blob.base + blob.size;
        *offset += blob.offset;
        return Some(blob);
    }

    None
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

fn add_blob(base: &mut u32, size: u32, offset: &mut u32, raw_size: u32) -> SVEBlobDescriptor {
    let blob = SVEBlobDescriptor {
        base: *base,
        size,
        offset: *offset,
        raw_size,
    };
    *offset += size;
    blob
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
    // get the sections
    let code_section = object.section_by_name(".text");
    let data_section = object.section_by_name(".data");
    let rodata_section = object.section_by_name(".rdata");
    let zdata_section = object.section_by_name(".bss");

    // convert section descriptors
    let mut offset = size_of::<SVEHeader>() as u32;
    let mut base = 0;

    let sve_code = convert_section(object, &code_section, &mut base, &mut offset);
    let sve_data = convert_section(object, &data_section, &mut base, &mut offset);
    let sve_rodata = convert_section(object, &rodata_section, &mut base, &mut offset);
    let sve_zdata = convert_section(object, &zdata_section, &mut base, &mut offset);

    let entry = if let Some(code) = code_section {
        object.entry() - code.address()
    } else {
        0
    } as u32;

    // build the header
    let header = SVEHeader {
        magic: sve::SVE_MAGIC,
        revision: sve::SVE_REVISION,
        machine: sve::SVEMachine::RISCV, // TODO: figure out the actual machine type
        entry,
        code: sve_code.clone().unwrap_or_default(),
        data: sve_data.clone().unwrap_or_default(),
        rodata: sve_rodata.clone().unwrap_or_default(),
        zdata: sve_zdata.clone().unwrap_or_default(),
        relocs: add_blob(&mut base, relocs_size, &mut offset, relocs_size),
        imports: add_blob(&mut base, imports_size, &mut offset, imports_size),
        import_ptrs: add_blob(&mut base, import_ptrs_size, &mut offset, 0), // import pointers are in-memory only, same as zdata
        exports: add_blob(&mut base, exports_size, &mut offset, exports_size),
        strings: add_blob(&mut base, strings_size, &mut offset, strings_size),
    };
    let mut header_data =
        bincode::encode_to_vec(header, config.clone()).expect("failed to encode header");

    // lay out the data
    let mut data = vec![];
    data.append(&mut header_data);
    let sections = vec![sve_code, sve_data, sve_rodata, sve_zdata];
    sections.iter().for_each(|section| {
        if let Some(section) = section {
            let mut section_data = bincode::encode_to_vec(section, config.clone()).expect("failed to encode section");
            data.append(&mut section_data);
        }
    });

    data
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
                .expect("failed to encode relocation descriptor");
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
                .expect("failed to encode import descriptor");
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
                .expect("failed to encode export descriptor");
            exports.append(&mut export_raw);
        }
    });

    exports
}

pub fn build_sve(object: &object::File) -> Vec<u8> {
    let config: BincodeConfig = Default::default();

    let mut strings = vec![];
    let relocs = build_relocs(object, &config);
    let (imports, import_ptrs_size) = build_imports(object, &config, &mut strings);
    let exports = build_exports(object, &config, &mut strings);

    let header_and_sections = build_header_and_sections(
        &object,
        strings.len() as u32,
        relocs.len() as u32,
        imports.len() as u32,
        import_ptrs_size,
        exports.len() as u32,
        &config,
    );

    vec![header_and_sections, relocs, imports, exports, strings]
        .iter()
        .flatten()
        .cloned()
        .collect()
}
