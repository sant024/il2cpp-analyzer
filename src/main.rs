mod app;
mod binary;
mod helper;
mod il2cpp;
mod metadata;
mod utils;
use std::io::Cursor;

use crate::app::App;
use crate::binary::*;
use crate::helper::ClassView;
use crate::helper::DataView;
use crate::il2cpp::*;
use crate::metadata::*;
use crate::utils::*;

use anyhow::Result;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use eframe::egui;
use object::read::elf::ElfFile32;
use object::read::elf::ElfFile64;
use object::{read::elf::ElfFile, Object};
use object::{Endianness, ObjectSection, ObjectSegment};
use std::path::PathBuf;
pub struct MetadataUtil {
    data: Vec<u8>,
    metadata_hdr: Il2CppGlobalMetadataHeader,
}

impl MetadataUtil {
    pub fn new(data: Vec<u8>) -> Self {
        let (_, metadata_hdr) =
            parse_metadata_header(&data).expect("Failed to parse metadata header");

        println!("Loader Sanity: {}", metadata_hdr.sanity);
        println!("Loader Version: {}", metadata_hdr.version);

        MetadataUtil {
            data: data,
            metadata_hdr: metadata_hdr,
        }
    }

    pub fn get_image_definition_by_id(&self, image_index: i32) -> Option<Il2CppImageDefinition> {
        let (_, metadata_image_definition) = parse_il2cpp_image_definition(
            &self.data[(self.metadata_hdr.images_offset
                + (image_index * std::mem::size_of::<Il2CppImageDefinition>() as i32))
                as usize..],
        )
        .expect("Failed to parse image definition");
        Some(metadata_image_definition)
    }
    pub fn get_string(&self, idx: i32) -> Option<String> {
        if idx < self.metadata_hdr.string_count {
            let name =
                get_str(&self.data, (self.metadata_hdr.string_offset + idx) as usize).unwrap();

            Some(name.to_string())
        } else {
            None
        }
    }

    pub fn get_type_definition_by_id(&self, type_index: i32) -> Option<Il2CppTypeDefinition> {
        // GetTypeDefFromIndex

        // let dbg_offset = self.metadata_hdr.type_definitions_offset
        //     + (type_index * std::mem::size_of::<Il2CppTypeDefinition>() as i32);

        // println!("(get_type_definition_by_id struct is in {:x}", dbg_offset);
        let (_, type_definition) = parse_il2cpp_type_definition(
            &self.data[(self.metadata_hdr.type_definitions_offset
                + (type_index * std::mem::size_of::<Il2CppTypeDefinition>() as i32))
                as usize..],
        )
        .expect("Failed to parse type definition");
        Some(type_definition)
    }

    pub fn get_method_definition_by_id(&self, method_index: i32) -> Option<Il2CppMethodDefinition> {
        let (_, method_definition) = parse_il2cpp_method_definition(
            &self.data[(self.metadata_hdr.methods_offset
                + (method_index * std::mem::size_of::<Il2CppMethodDefinition>() as i32))
                as usize..],
        )
        .expect("Failed to method  definition");
        Some(method_definition)
    }

    pub fn get_parameter_def_by_id(&self, param_index: i32) -> Option<Il2CppParameterDefinition> {
        let (_, param_definition) = parse_il2cpp_parameter_definition(
            &self.data[(self.metadata_hdr.parameters_offset
                + (param_index * std::mem::size_of::<Il2CppParameterDefinition>() as i32))
                as usize..],
        )
        .expect("Failed to parameter definition");
        Some(param_definition)
    }

    pub fn get_field_def_by_id(&self, field_index: i32) -> Option<Il2CppFieldDefinition> {
        let (_, field_definition) = parse_il2cpp_field_definition(
            &self.data[(self.metadata_hdr.fields_offset
                + (field_index * std::mem::size_of::<Il2CppFieldDefinition>() as i32))
                as usize..],
        )
        .expect("Failed to field definition");
        Some(field_definition)
    }

    pub fn get_field_default_by_id(&self, default_index: i32) -> Option<Il2CppFieldDefaultValue> {
        let (_, field_default) = parse_il2cpp_field_default_value(
            &self.data[(self.metadata_hdr.field_default_values_offset
                + (default_index * std::mem::size_of::<Il2CppFieldDefaultValue>() as i32))
                as usize..],
        )
        .expect("Failed to field_default");
        Some(field_default)
    }
}

pub struct BinUtil {
    code_registration: CodeRegistration,
    metadata_registration: MetadataRegistration,
}

impl BinUtil {
    pub fn new(data: Vec<u8>) -> Self {
        let elf_file: ElfFile32 = ElfFile::parse(&*data).unwrap();
        let code_registration = CodeRegistration::read(&elf_file, 0x1448398).unwrap();
        let metadata_registration = MetadataRegistration::read(&elf_file, 0x1469ad0).unwrap();
        BinUtil {
            code_registration: code_registration,
            metadata_registration: metadata_registration,
        }
    }

    pub fn get_type_from_type_index(&self, type_index: i32) -> Type {
        let mut type_mut: i32 = type_index;
        if type_index as usize >= self.metadata_registration.types.len() {
            println!(
                "(get_type_from_type_index) type_index too big: {}",
                type_index
            );
            type_mut = 0;
        }
        return self.metadata_registration.types[type_mut as usize];
    }

    pub fn get_method_offset_by_id(&self, method_index: usize) -> u64 {
        // add error check
        self.code_registration.method_pointers[method_index]
    }

    pub fn get_field_offset_by_id(&self, field_index: usize) -> u32 {
        // add error check
        self.metadata_registration.field_offset_addrs[field_index]
    }
}
fn get_class(dv: &mut DataView, mm: &MetadataUtil, bn: &BinUtil, type_index: i32) {
    // let file_data = std::fs::read(metadata_path).expect("Failed to read file");
    // let mm = MetadataUtil::new(file_data);
    // let raw_metadata = &mm.metadata_hdr;

    // let def = mm.get_method_definition_by_id(0).unwrap();

    // let image_count =
    //     raw_metadata.images_count / std::mem::size_of::<Il2CppImageDefinition>() as i32;
    // let namespace = mm.get_string(def.name_index).unwrap();
    println!("\t\t(dump class) idx: {}", type_index);

    let def = mm.get_type_definition_by_id(type_index).unwrap(); // could possibly be from here?
                                                                 //println!("[!] namespace index : {} ", def.namespace_index);

    let namespace = mm.get_string(def.namespace_index).unwrap();
    println!("[Namespace: {}]", namespace);
    let mut attr_builder = String::new();

    if (def.flags as u32 & TYPE_ATTRIBUTE_SERIALIZABLE) != 0 {
        attr_builder.push_str("[Serializable] ");
    }

    if (def.flags as u32 & TYPE_ATTRIBUTE_VISIBILITY_MASK) == TYPE_ATTRIBUTE_PUBLIC {
        attr_builder.push_str("public ");
    }

    if def.flags as u32 & TYPE_ATTRIBUTE_ABSTRACT != 0 {
        attr_builder.push_str("abstract");
    }

    if def.flags as u32 & TYPE_ATTRIBUTE_SEALED != 0 {
        attr_builder.push_str("sealed ");
    }

    if def.flags as u32 & TYPE_ATTRIBUTE_INTERFACE != 0 {
        attr_builder.push_str(" interface ");
    } else {
        attr_builder.push_str("class ");
    }
    let mut cv = ClassView::new();

    let class_name = mm.get_string(def.name_index).unwrap();
    cv.name = class_name.clone();
    let class_line = format!("{} {}", attr_builder, class_name);
    println!("{}", class_line);

    println!("\t Fields");

    // u16 (-1) 65535
    let field_start: usize = def.field_start as usize;

    // println!("[!] field start: {}", def.field_start);
    // println!("[!] field count: {}", def.field_count);
    // if field_start > 0 {

    let field_end = def.field_start as usize + def.field_count as usize;
    for field_index in field_start..field_end {
        get_field(&mut cv, &mm, &bn, field_index as i32);
    }

    println!("\t Methods");

    let method_start: usize = def.method_start as usize;

    for method_index in method_start..def.method_count as usize {
        get_method(&mut cv, &mm, &bn, method_index as i32);
    }

    dv.class_dump.push(cv);
}

fn type_string_for_id(id: i32) -> String {
    let index = id as usize;

    let type_dic: [&str; 36] = [
        "END",
        "void",
        "bool",
        "char",
        "sbyte",  // I1
        "byte",   // U1
        "short",  // I2
        "ushort", // U2
        "int",    // I4
        "uint",   // U4
        "long",   // I8
        "ulong",  // U8
        "float",  // R4
        "double", // R8
        "String",
        "Ptr",
        "REF",
        "ValueType",
        "Class",
        "var",
        "array",
        "Generic",
        "TypedREF",
        "int",  // I
        "uint", // U
        "FuncPtr",
        "Object",
        "sArray",
        "mvar",
        "cmod_reqd",
        "cmod_opt",
        "internal_type",
        "modifier",
        "sentinel",
        "pinned",
        "enum",
    ];
    return String::from(type_dic[index]);
}

fn get_type_name(type_: &Type) -> String {
    //println!("get_type_name. ty: {:?}", type_);
    let mut ret = String::from("unk");
    if type_.ty == Il2CppTypeEnum::IL2CPP_TYPE_CLASS as u8
        || type_.ty == Il2CppTypeEnum::IL2CPP_TYPE_VALUETYPE as u8
    {
        //println!("(get_type_name) first if ");
    } else if type_.ty == Il2CppTypeEnum::IL2CPP_TYPE_GENERICINST as u8 {
        //println!("tget_type_name else if");
    } else {
        //println!("(get_type_name) else correct");
        ret = type_string_for_id(type_.ty as i32);
    }

    return ret;
}

pub fn get_field(cv: &mut ClassView, mm: &MetadataUtil, bn: &BinUtil, field_index: i32) -> String {
    // when 4,5 nothing prints?
    // the field index is wrong
    //println!("idx: {}", field_index);
    let field = mm.get_field_def_by_id(field_index).unwrap();

    // println!(
    //     "Debug il2cppfielddefinition name index {}",
    //     field.name_index
    // );
    // if field.type_index < 65535

    //println!("field definition type index: {}", field.type_index);
    let field_type = bn.get_type_from_type_index(field.type_index); // sometimes too big ? or -1 old

    //println!("attrs {}", field_type.attrs);

    let mut modifier = String::new();

    if field_type.attrs & FIELD_ATTRIBUTE_PRIVATE == FIELD_ATTRIBUTE_PRIVATE {
        modifier.push_str("private ");
    }

    if field_type.attrs & FIELD_ATTRIBUTE_PUBLIC == FIELD_ATTRIBUTE_PUBLIC {
        modifier.push_str("public ");
    }

    if field_type.attrs & FIELD_ATTRIBUTE_STATIC != 0 {
        modifier.push_str("static");
    }

    let type_name = get_type_name(&field_type);
    let field_name = mm.get_string(field.name_index).unwrap();
    let line_field = format!("{} {} {}", modifier, type_name, field_name);

    //let offset = bn.get_field_offset_by_id(field_index as usize);
    cv.fields.push(line_field.clone());
    println!("\t {} ", line_field); // #0x{:x} check offset if correct

    return line_field;
    // add field defaults " = .. ";
    //let field_default = mm.get_field_default_by_id(field_index).unwrap();
}

fn get_method(cv: &mut ClassView, mm: &MetadataUtil, bn: &BinUtil, method_index: i32) {
    // let method_ptrs_count = bn.code_registration.method_pointers.len();  // get_method in main()
    // println!("method ptrs count: {}", method_ptrs_count);
    // for method_index in 0..method_ptrs_count

    let def = mm.get_method_definition_by_id(method_index as i32).unwrap();

    //println!("def flags: {}", def.flags);

    let mut modifier = String::new();
    if ((def.flags as u32 & METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK) == METHOD_ATTRIBUTE_PRIVATE) {
        modifier.push_str("private ");
    }

    if ((def.flags as u32 & METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK) == METHOD_ATTRIBUTE_PUBLIC) {
        modifier.push_str("public ");
    }

    if (def.flags as u32 & METHOD_ATTRIBUTE_VIRTUAL != 0) {
        modifier.push_str("virtual ");
    }

    if (def.flags as u32 & METHOD_ATTRIBUTE_STATIC != 0) {
        modifier.push_str("static ");
    }

    //modifier

    let return_type = bn.metadata_registration.types[def.return_type as usize]; // pType = // GetTypeFromTypeIndex()
                                                                                //println!("return type full: {:?}", return_type);
                                                                                //println!("[=] method paremter count: {}", def.parameter_count);

    let method_name = mm.get_string(def.name_index).unwrap();
    let method_type_name = get_type_name(&return_type);

    // println!(
    //     "method name: {}, method returns: {}",
    //     method_name, method_type_name
    // );
    let mut method_line = format! {"{}{} {} (", modifier,method_type_name, method_name};

    for parameter_index in 0..def.parameter_count {
        //println!("[v] parameter_index : {}", parameter_index);
        let param = mm.get_parameter_def_by_id(parameter_index as i32).unwrap();
        let param_name: String = mm.get_string(param.name_index).unwrap();
        //println!("param type_index: {}", param.type_index);
        //println!("param name: {}", param_name);

        let mut idx_param = param.type_index;

        if idx_param as usize >= bn.metadata_registration.types.len() {
            println!("idx_param too big: {}", idx_param);
            idx_param = 0;
        }

        let param_type = bn.metadata_registration.types[idx_param as usize];
        let param_type_name = get_type_name(&param_type);
        //println!("param type name: {}", param_type_name);

        method_line.push_str(&format!("{} {}", param_type_name, param_name));

        //println!("attrs{}", param_type.attrs);
        let mut addt = String::new();
        // println!(
        //     "optional see: {}",
        //     param_type.attrs & PARAM_ATTRIBUTE_OPTIONAL
        // );
        if (param_type.attrs & PARAM_ATTRIBUTE_OPTIONAL != 0) {
            addt.push_str("optional");
        }
        // println!("out see: {}", param_type.attrs & PARAM_ATTRIBUTE_OUT);
        if (param_type.attrs & PARAM_ATTRIBUTE_OUT != 0) {
            addt.push_str("out");
        }

        method_line.push_str(&addt);

        if parameter_index != def.parameter_count - 1 {
            method_line.push_str(", ");
        }
    }

    // offset
    let offset = bn.code_registration.method_pointers[method_index as usize];
    //println!("method definiton offset: {:x}", offset);

    method_line.push_str(&format!("); //{:x}", offset));
    cv.methods.push(method_line.clone());
    println!("{}", method_line);

    // let image_count =
    //     mm.metadata_hdr.images_count / std::mem::size_of::<Il2CppImageDefinition>() as i32;

    // // get image definition
    // for image_index in 0..image_count {
    //     let s_image_definition = mm.get_image_definition_by_id(image_index).unwrap();
    //     let name_index = s_image_definition.name_index;
}

fn main() {
    //let native_options = eframe::NativeOptions::default();
    let mut dv: DataView = cli();

    let native_options = eframe::NativeOptions {
        initial_window_size: Option::from(egui::Vec2::new(575 as f32, 350 as f32)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "IL2CPP Analysis",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, dv))),
    );

    //cli();
}

fn cli() -> DataView {
    let metadata_path = concat!(env!("CARGO_MANIFEST_DIR"), "\\global-metadata.dat");
    let sodata_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\libil2cpp.so");
    let file_data = std::fs::read(metadata_path).expect("Failed to read file");
    let so_data = std::fs::read(sodata_path).expect("Failed to read file");

    let (_, metadata_hdr) =
        parse_metadata_header(&file_data).expect("Failed to parse metadata header");

    println!("Loader Sanity: {}", metadata_hdr.sanity);
    println!("Loader Version: {}", metadata_hdr.version);

    // let mut line = String::new();
    // let input = std::io::stdin()
    //     .read_line(&mut line)
    //     .expect("Failed to read line");

    let mm = MetadataUtil::new(file_data);
    //  must know if elf file is 32 or 64 binary
    let bn = BinUtil::new(so_data);

    let mut dv = DataView::new();

    let image_count =
        mm.metadata_hdr.images_count / std::mem::size_of::<Il2CppImageDefinition>() as i32;

    //get image definition
    for image_index in 0..image_count {
        let s_image_definition = mm.get_image_definition_by_id(image_index).unwrap();
        let image_name = mm.get_string(s_image_definition.name_index).unwrap();
        let image_typestart = s_image_definition.type_start;
        println!(" -- Image Info {0}, {1}", image_name, image_typestart);
        dv.image.push(image_name);
        // get type definition for image
        let type_end =
            s_image_definition.type_start as usize + s_image_definition.type_count as usize;
        for type_index in s_image_definition.type_start as usize..type_end {
            //println!("type index: {}", type_index);
            let s_type_definition = mm.get_type_definition_by_id(type_index as i32).unwrap();
            let type_name = mm.get_string(s_type_definition.name_index).unwrap();

            println!("(*) Typename: {}", type_name);
            // println!(
            //     "namespace type index: {}",
            //     s_type_definition.namespace_index
            // );
            let type_namespace = mm.get_string(s_type_definition.namespace_index).unwrap();

            println!("(*) Namespace: {}", type_namespace);
        }
    }

    let ui_num_types =
        mm.metadata_hdr.type_definitions_count / std::mem::size_of::<Il2CppTypeDefinition>() as i32;

    println!("ui_num_types: {}", ui_num_types);

    for i in 0..ui_num_types {
        get_class(&mut dv, &mm, &bn, i);
    }

    return dv;
    //get_class(&elf_file);
    //get_method(&elf_file);
}
