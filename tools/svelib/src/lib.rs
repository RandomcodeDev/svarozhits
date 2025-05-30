use std::collections::HashMap;

use bincode::config::{Configuration, Fixint, LittleEndian, NoLimit};
use object::{Object, ObjectSymbol, RelocationKind};
use sve::{SVEExportDescriptor, SVEHeader, SVERelocDescriptor, SVERelocType};

type BincodeConfig = Configuration<LittleEndian, Fixint, NoLimit>;

fn build_header_and_sections(object: &object::File<'_>, config: BincodeConfig) -> Vec<u8> {
    let mut header = SVEHeader::default();
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
                _ => SVERelocType::Unknown
            };

            let desc = SVERelocDescriptor { addr: addr as u32, reloc_type };
            let mut desc_raw = bincode::encode_to_vec(desc, config.clone()).expect("failed to encode relocation descriptor!");
            reloc_table.append(&mut desc_raw);
        }
    }

    reloc_table
}

// fn build_imports(object: &object::File, strings: &mut Vec<u8>) -> Vec<u8> {
//     let mut import_ptrs = vec![];
//     let mut imports = vec![];

//     let libs = HashMap::new();

//     object.dynamic_symbols().for_each(|sym| {
//         sym.
//     });

//     imports
// }

fn build_exports(object: &object::File, config: &BincodeConfig, strings: &mut Vec<u8>) -> Vec<u8> {
    let mut exports = vec![];

    object.dynamic_symbols().for_each(|sym| {
        if sym.is_global() && let Ok(name) = sym.name() {
            let sym_name = add_string(name, strings);
            let export = SVEExportDescriptor {
                sym_name,
                addr: sym.address() as u32
            };

            let mut export_raw = bincode::encode_to_vec(export, config.clone()).expect("failed to encode export descriptor!");
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

pub fn build_sve(object: &object::File) -> Vec<u8> {
    let config: BincodeConfig = Default::default();

    let header = build_header_and_sections(object, &config);


    let mut strings = vec![];
    let relocs = build_relocs(object, &config);
    let exports = build_exports(object, &config, &mut strings);

    vec![header]
}
