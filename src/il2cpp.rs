// UNITY VERSION 20/21 (actually 21 works)

#![allow(unused_imports)]
#![allow(dead_code)]

pub const FIELD_ATTRIBUTE_PRIVATE: u32 = 0x0001;
pub const FIELD_ATTRIBUTE_FAM_AND_ASSEM: u32 = 0x0002;
pub const FIELD_ATTRIBUTE_ASSEMBLY: u32 = 0x0003;
pub const FIELD_ATTRIBUTE_FAMILY: u32 = 0x0004;
pub const FIELD_ATTRIBUTE_FAM_OR_ASSEM: u32 = 0x0005;
pub const FIELD_ATTRIBUTE_PUBLIC: u32 = 0x0006;

pub const FIELD_ATTRIBUTE_STATIC: u32 = 0x0010;
pub const FIELD_ATTRIBUTE_INIT_ONLY: u32 = 0x0020;
pub const FIELD_ATTRIBUTE_LITERAL: u32 = 0x0040;
pub const FIELD_ATTRIBUTE_NOT_SERIALIZED: u32 = 0x0080;
pub const FIELD_ATTRIBUTE_SPECIAL_NAME: u32 = 0x0200;
pub const FIELD_ATTRIBUTE_PINVOKE_IMPL: u32 = 0x2000;

pub const FIELD_ATTRIBUTE_RESERVED_MASK: u32 = 0x9500;
pub const FIELD_ATTRIBUTE_RT_SPECIAL_NAME: u32 = 0x0400;
pub const FIELD_ATTRIBUTE_HAS_FIELD_MARSHAL: u32 = 0x1000;
pub const FIELD_ATTRIBUTE_HAS_DEFAULT: u32 = 0x8000;
pub const FIELD_ATTRIBUTE_HAS_FIELD_RVA: u32 = 0x0100;

pub const METHOD_IMPL_ATTRIBUTE_CODE_TYPE_MASK: u32 = 0x0003;
pub const METHOD_IMPL_ATTRIBUTE_IL: u32 = 0x0000;
pub const METHOD_IMPL_ATTRIBUTE_NATIVE: u32 = 0x0001;
pub const METHOD_IMPL_ATTRIBUTE_OPTIL: u32 = 0x0002;
pub const METHOD_IMPL_ATTRIBUTE_RUNTIME: u32 = 0x0003;

pub const METHOD_IMPL_ATTRIBUTE_MANAGED_MASK: u32 = 0x0004;
pub const METHOD_IMPL_ATTRIBUTE_UNMANAGED: u32 = 0x0004;
pub const METHOD_IMPL_ATTRIBUTE_MANAGED: u32 = 0x0000;

pub const METHOD_IMPL_ATTRIBUTE_FORWARD_REF: u32 = 0x0010;
pub const METHOD_IMPL_ATTRIBUTE_PRESERVE_SIG: u32 = 0x0080;
pub const METHOD_IMPL_ATTRIBUTE_INTERNAL_CALL: u32 = 0x1000;
pub const METHOD_IMPL_ATTRIBUTE_SYNCHRONIZED: u32 = 0x0020;
pub const METHOD_IMPL_ATTRIBUTE_NOINLINING: u32 = 0x0008;
pub const METHOD_IMPL_ATTRIBUTE_MAX_METHOD_IMPL_VAL: u32 = 0xffff;

pub const METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK: u32 = 0x0007;
pub const METHOD_ATTRIBUTE_COMPILER_CONTROLLED: u32 = 0x0000;
pub const METHOD_ATTRIBUTE_PRIVATE: u32 = 0x0001;
pub const METHOD_ATTRIBUTE_FAM_AND_ASSEM: u32 = 0x0002;
pub const METHOD_ATTRIBUTE_ASSEM: u32 = 0x0003;
pub const METHOD_ATTRIBUTE_FAMILY: u32 = 0x0004;
pub const METHOD_ATTRIBUTE_FAM_OR_ASSEM: u32 = 0x0005;
pub const METHOD_ATTRIBUTE_PUBLIC: u32 = 0x0006;

pub const METHOD_ATTRIBUTE_STATIC: u32 = 0x0010;
pub const METHOD_ATTRIBUTE_FINAL: u32 = 0x0020;
pub const METHOD_ATTRIBUTE_VIRTUAL: u32 = 0x0040;

pub const TYPE_ATTRIBUTE_VISIBILITY_MASK: u32 = 0x00000007;
pub const TYPE_ATTRIBUTE_NOT_PUBLIC: u32 = 0x00000000;
pub const TYPE_ATTRIBUTE_PUBLIC: u32 = 0x00000001;
pub const TYPE_ATTRIBUTE_NESTED_PUBLIC: u32 = 0x00000002;
pub const TYPE_ATTRIBUTE_NESTED_PRIVATE: u32 = 0x00000003;
pub const TYPE_ATTRIBUTE_NESTED_FAMILY: u32 = 0x00000004;
pub const TYPE_ATTRIBUTE_NESTED_ASSEMBLY: u32 = 0x00000005;
pub const TYPE_ATTRIBUTE_NESTED_FAM_AND_ASSEM: u32 = 0x00000006;
pub const TYPE_ATTRIBUTE_NESTED_FAM_OR_ASSEM: u32 = 0x00000007;

pub const TYPE_ATTRIBUTE_LAYOUT_MASK: u32 = 0x00000018;
pub const TYPE_ATTRIBUTE_AUTO_LAYOUT: u32 = 0x00000000;
pub const TYPE_ATTRIBUTE_SEQUENTIAL_LAYOUT: u32 = 0x00000008;
pub const TYPE_ATTRIBUTE_EXPLICIT_LAYOUT: u32 = 0x00000010;

pub const TYPE_ATTRIBUTE_CLASS_SEMANTIC_MASK: u32 = 0x00000020;
pub const TYPE_ATTRIBUTE_CLASS: u32 = 0x00000000;
pub const TYPE_ATTRIBUTE_INTERFACE: u32 = 0x00000020;

pub const TYPE_ATTRIBUTE_ABSTRACT: u32 = 0x00000080;
pub const TYPE_ATTRIBUTE_SEALED: u32 = 0x00000100;
pub const TYPE_ATTRIBUTE_SPECIAL_NAME: u32 = 0x00000400;

pub const TYPE_ATTRIBUTE_IMPORT: u32 = 0x00001000;
pub const TYPE_ATTRIBUTE_SERIALIZABLE: u32 = 0x00002000;

pub const TYPE_ATTRIBUTE_STRING_FORMAT_MASK: u32 = 0x00030000;
pub const TYPE_ATTRIBUTE_ANSI_CLASS: u32 = 0x00000000;
pub const TYPE_ATTRIBUTE_UNICODE_CLASS: u32 = 0x00010000;
pub const TYPE_ATTRIBUTE_AUTO_CLASS: u32 = 0x00020000;

pub const TYPE_ATTRIBUTE_BEFORE_FIELD_INIT: u32 = 0x00100000;
pub const TYPE_ATTRIBUTE_FORWARDER: u32 = 0x00200000;

pub const TYPE_ATTRIBUTE_RESERVED_MASK: u32 = 0x00040800;
pub const TYPE_ATTRIBUTE_RT_SPECIAL_NAME: u32 = 0x00000800;
pub const TYPE_ATTRIBUTE_HAS_SECURITY: u32 = 0x00040000;

pub const PARAM_ATTRIBUTE_IN: u32 = 0x0001;
pub const PARAM_ATTRIBUTE_OUT: u32 = 0x0002;
pub const PARAM_ATTRIBUTE_OPTIONAL: u32 = 0x0010;
pub const PARAM_ATTRIBUTE_RESERVED_MASK: u32 = 0xf000;
pub const PARAM_ATTRIBUTE_HAS_DEFAULT: u32 = 0x1000;
pub const PARAM_ATTRIBUTE_HAS_FIELD_MARSHAL: u32 = 0x2000;
pub const PARAM_ATTRIBUTE_UNUSED: u32 = 0xcfe0;

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum Il2CppTypeEnum {
    IL2CPP_TYPE_END = 0x00,
    IL2CPP_TYPE_VOID = 0x01,
    IL2CPP_TYPE_BOOLEAN = 0x02,
    IL2CPP_TYPE_CHAR = 0x03,
    IL2CPP_TYPE_I1 = 0x04,
    IL2CPP_TYPE_U1 = 0x05,
    IL2CPP_TYPE_I2 = 0x06,
    IL2CPP_TYPE_U2 = 0x07,
    IL2CPP_TYPE_I4 = 0x08,
    IL2CPP_TYPE_U4 = 0x09,
    IL2CPP_TYPE_I8 = 0x0a,
    IL2CPP_TYPE_U8 = 0x0b,
    IL2CPP_TYPE_R4 = 0x0c,
    IL2CPP_TYPE_R8 = 0x0d,
    IL2CPP_TYPE_STRING = 0x0e,
    IL2CPP_TYPE_PTR = 0x0f,
    IL2CPP_TYPE_BYREF = 0x10,
    IL2CPP_TYPE_VALUETYPE = 0x11,
    IL2CPP_TYPE_CLASS = 0x12,
    IL2CPP_TYPE_VAR = 0x13,
    IL2CPP_TYPE_ARRAY = 0x14,
    IL2CPP_TYPE_GENERICINST = 0x15,
    IL2CPP_TYPE_TYPEDBYREF = 0x16,
    IL2CPP_TYPE_I = 0x18,
    IL2CPP_TYPE_U = 0x19,
    IL2CPP_TYPE_FNPTR = 0x1b,
    IL2CPP_TYPE_OBJECT = 0x1c,
    IL2CPP_TYPE_SZARRAY = 0x1d,
    IL2CPP_TYPE_MVAR = 0x1e,
    IL2CPP_TYPE_CMOD_REQD = 0x1f,
    IL2CPP_TYPE_CMOD_OPT = 0x20,
    IL2CPP_TYPE_INTERNAL = 0x21,
    IL2CPP_TYPE_MODIFIER = 0x40,
    IL2CPP_TYPE_SENTINEL = 0x41,
    IL2CPP_TYPE_PINNED = 0x45,
    IL2CPP_TYPE_ENUM = 0x55,
}

type TypeIndex = i32;
type TypeDefinitionIndex = i32;
type FieldIndex = i32;
type DefaultValueIndex = i32;
type DefaultValueDataIndex = i32;
type CustomAttributeIndex = i32;
type ParameterIndex = i32;
type MethodIndex = i32;
type GenericMethodIndex = i32;
type PropertyIndex = i32;
type EventIndex = i32;
type GenericContainerIndex = i32;
type GenericParameterIndex = i32;
type GenericParameterConstraintIndex = i16;
type NestedTypeIndex = i32;
type InterfacesIndex = i32;
type VTableIndex = i32;
type InterfaceOffsetIndex = i32;
type RGCTXIndex = i32;
type StringIndex = i32;
type StringLiteralIndex = i32;
type GenericInstIndex = i32;
type ImageIndex = i32;
type AssemblyIndex = i32;
type GuidIndex = i32;

pub struct Il2CppParameterDefinition {
    pub name_index: StringIndex,
    pub token: u32,
    pub custom_attribute_index: CustomAttributeIndex,
    pub type_index: TypeIndex,
}

pub struct Il2CppImageDefinition {
    pub name_index: StringIndex,
    pub assembly_index: AssemblyIndex,
    pub type_start: TypeDefinitionIndex,
    pub type_count: u32,
    pub entry_point_index: MethodIndex,
    pub token: u32,
}

pub struct Il2CppImage {
    pub name: *const std::os::raw::c_char,
    pub assembly_index: AssemblyIndex,
    pub type_start: TypeDefinitionIndex,
    pub type_count: u32,
    pub entry_point_index: MethodIndex,
    pub name_to_class_hash_table: *mut std::ffi::c_void, // Adjust the type accordingly
    pub token: u32,
}

pub struct Il2CppGlobalMetadataHeader {
    pub sanity: i32,
    pub version: i32,
    pub string_literal_offset: i32, // string data for managed code
    pub string_literal_count: i32,
    pub string_literal_data_offset: i32,
    pub string_literal_data_count: i32,
    pub string_offset: i32, // string data for metadata
    pub string_count: i32,
    pub events_offset: i32, // Il2CppEventDefinition
    pub events_count: i32,
    pub properties_offset: i32, // Il2CppPropertyDefinition
    pub properties_count: i32,
    pub methods_offset: i32, // Il2CppMethodDefinition
    pub methods_count: i32,
    pub parameter_default_values_offset: i32, // Il2CppParameterDefaultValue
    pub parameter_default_values_count: i32,
    pub field_default_values_offset: i32, // Il2CppFieldDefaultValue
    pub field_default_values_count: i32,
    pub field_and_parameter_default_value_data_offset: i32, // uint8_t
    pub field_and_parameter_default_value_data_count: i32,
    pub field_marshaled_sizes_offset: i32, // Il2CppFieldMarshaledSize
    pub field_marshaled_sizes_count: i32,
    pub parameters_offset: i32, // Il2CppParameterDefinition
    pub parameters_count: i32,
    pub fields_offset: i32, // Il2CppFieldDefinition
    pub fields_count: i32,
    pub generic_parameters_offset: i32, // Il2CppGenericParameter
    pub generic_parameters_count: i32,
    pub generic_parameter_constraints_offset: i32, // TypeIndex
    pub generic_parameter_constraints_count: i32,
    pub generic_containers_offset: i32, // Il2CppGenericContainer
    pub generic_containers_count: i32,
    pub nested_types_offset: i32, // TypeDefinitionIndex
    pub nested_types_count: i32,
    pub interfaces_offset: i32, // TypeIndex
    pub interfaces_count: i32,
    pub vtable_methods_offset: i32, // EncodedMethodIndex
    pub vtable_methods_count: i32,
    pub interface_offsets_offset: i32, // Il2CppInterfaceOffsetPair
    pub interface_offsets_count: i32,
    pub type_definitions_offset: i32, // Il2CppTypeDefinition
    pub type_definitions_count: i32,
    pub rgctx_entries_offset: i32, // Il2CppRGCTXDefinition
    pub rgctx_entries_count: i32,
    pub images_offset: i32, // Il2CppImageDefinition
    pub images_count: i32,
    pub assemblies_offset: i32, // Il2CppAssemblyDefinition
    pub assemblies_count: i32,
    pub metadata_usage_lists_offset: i32, // Il2CppMetadataUsageList
    pub metadata_usage_lists_count: i32,
    pub metadata_usage_pairs_offset: i32, // Il2CppMetadataUsagePair
    pub metadata_usage_pairs_count: i32,
    pub field_refs_offset: i32, // Il2CppFieldRef
    pub field_refs_count: i32,
    pub referenced_assemblies_offset: i32, // i32
    pub referenced_assemblies_count: i32,
    pub attributes_info_offset: i32, // Il2CppCustomAttributeTypeRange
    pub attributes_info_count: i32,
    pub attribute_types_offset: i32, // TypeIndex
    pub attribute_types_count: i32,
}

pub struct Il2CppTypeDefinition {
    pub name_index: StringIndex,
    pub namespace_index: StringIndex,
    pub custom_attribute_index: CustomAttributeIndex,
    pub byval_type_index: TypeIndex,
    pub byref_type_index: TypeIndex,
    pub declaring_type_index: TypeIndex,
    pub parent_index: TypeIndex,
    pub element_type_index: TypeIndex,
    pub rgctx_start_index: RGCTXIndex,
    pub rgctx_count: i32,
    pub generic_container_index: GenericContainerIndex,
    pub delegate_wrapper_from_managed_to_native_index: MethodIndex,
    pub marshaling_functions_index: i32,
    pub ccw_function_index: i32,
    pub guid_index: GuidIndex, //version 21 only
    pub flags: u32,
    pub field_start: FieldIndex,
    pub method_start: MethodIndex,
    pub event_start: EventIndex,
    pub property_start: PropertyIndex,
    pub nested_types_start: NestedTypeIndex,
    pub interfaces_start: InterfacesIndex,
    pub vtable_start: VTableIndex,
    pub interface_offsets_start: InterfacesIndex,
    pub method_count: u16,
    pub property_count: u16,
    pub field_count: u16, // previous: u16, prints 65535 {wrong def}
    pub event_count: u16,
    pub nested_type_count: u16,
    pub vtable_count: u16,
    pub interfaces_count: u16,
    pub interface_offsets_count: u16,
    pub bitfield: u32,
    pub token: u32,
}

pub struct Il2CppMarshalingFunctions {
    pub marshal_to_native_func: u32,
    pub marshal_from_native_func: u32,
    pub marshal_cleanup_func: u32,
}

pub struct Il2CppCodeRegistration {
    pub method_pointers_count: u32,
    pub method_pointers: *mut u32, // 6 of these *mut
    pub delegate_wrappers_from_native_to_managed_count: u32,
    pub delegate_wrappers_from_native_to_managed: *mut *mut u32,
    pub delegate_wrappers_from_managed_to_native_count: u32,
    pub delegate_wrappers_from_managed_to_native: *mut u32, // 2
    pub marshaling_functions_count: u32,
    pub marshaling_functions: *const Il2CppMarshalingFunctions,
    pub ccw_marshaling_functions_count: u32,
    pub ccw_marshaling_functions: *mut u32, //3
    pub generic_method_pointers_count: u32,
    pub generic_method_pointers: *mut u32, // 4
    pub invoker_pointers_count: u32,
    pub invoker_pointers: *mut u32, // 5
    pub custom_attribute_count: CustomAttributeIndex,
    pub custom_attribute_generators: *mut u32, //6
    pub guid_count: GuidIndex,
    pub guids: *const *const std::ffi::c_void,
}

pub struct Il2CppMethodDefinition {
    pub name_index: StringIndex,
    pub declaring_type: TypeDefinitionIndex,
    pub return_type: TypeIndex,
    pub parameter_start: ParameterIndex,
    pub custom_attribute_index: CustomAttributeIndex,
    pub generic_container_index: GenericContainerIndex,
    pub method_index: MethodIndex,
    pub invoker_index: MethodIndex,
    pub delegate_wrapper_index: MethodIndex,
    pub rgctx_start_index: RGCTXIndex,
    pub rgctx_count: i32,
    pub token: u32,
    pub flags: u16,
    pub iflags: u16,
    pub slot: u16,
    pub parameter_count: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Type {
    pub data: u32,
    pub ty: u8,
    pub by_ref: bool,
    pub attrs: u32,
}

pub struct Il2CppFieldDefinition {
    pub name_index: StringIndex,
    pub type_index: TypeIndex,
    pub custom_attribute_index: CustomAttributeIndex,
    pub token: u32,
}

pub struct Il2CppFieldDefaultValue {
    pub field_index: FieldIndex,
    pub type_index: TypeIndex,
    pub data_index: DefaultValueDataIndex,
}
