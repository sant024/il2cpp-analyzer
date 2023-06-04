use crate::il2cpp::*;
use nom::{
    number::complete::{le_i32, le_u16, le_u32, le_u64},
    IResult,
};

pub fn parse_metadata_header(input: &[u8]) -> IResult<&[u8], Il2CppGlobalMetadataHeader> {
    let (input, sanity) = le_i32(input)?;
    let (input, version) = le_i32(input)?;
    let (input, string_literal_offset) = le_i32(input)?;
    let (input, string_literal_count) = le_i32(input)?;
    let (input, string_literal_data_offset) = le_i32(input)?;
    let (input, string_literal_data_count) = le_i32(input)?;
    let (input, string_offset) = le_i32(input)?;
    let (input, string_count) = le_i32(input)?;
    let (input, events_offset) = le_i32(input)?;
    let (input, events_count) = le_i32(input)?;
    let (input, properties_offset) = le_i32(input)?;
    let (input, properties_count) = le_i32(input)?;
    let (input, methods_offset) = le_i32(input)?;
    let (input, methods_count) = le_i32(input)?;
    let (input, parameter_default_values_offset) = le_i32(input)?;
    let (input, parameter_default_values_count) = le_i32(input)?;
    let (input, field_default_values_offset) = le_i32(input)?;
    let (input, field_default_values_count) = le_i32(input)?;
    let (input, field_and_parameter_default_value_data_offset) = le_i32(input)?;
    let (input, field_and_parameter_default_value_data_count) = le_i32(input)?;
    let (input, field_marshaled_sizes_offset) = le_i32(input)?;
    let (input, field_marshaled_sizes_count) = le_i32(input)?;
    let (input, parameters_offset) = le_i32(input)?;
    let (input, parameters_count) = le_i32(input)?;
    let (input, fields_offset) = le_i32(input)?;
    let (input, fields_count) = le_i32(input)?;
    let (input, generic_parameters_offset) = le_i32(input)?;
    let (input, generic_parameters_count) = le_i32(input)?;
    let (input, generic_parameter_constraints_offset) = le_i32(input)?;
    let (input, generic_parameter_constraints_count) = le_i32(input)?;
    let (input, generic_containers_offset) = le_i32(input)?;
    let (input, generic_containers_count) = le_i32(input)?;
    let (input, nested_types_offset) = le_i32(input)?;
    let (input, nested_types_count) = le_i32(input)?;
    let (input, interfaces_offset) = le_i32(input)?;
    let (input, interfaces_count) = le_i32(input)?;
    let (input, vtable_methods_offset) = le_i32(input)?;
    let (input, vtable_methods_count) = le_i32(input)?;
    let (input, interface_offsets_offset) = le_i32(input)?;
    let (input, interface_offsets_count) = le_i32(input)?;
    let (input, type_definitions_offset) = le_i32(input)?;
    let (input, type_definitions_count) = le_i32(input)?;
    let (input, rgctx_entries_offset) = le_i32(input)?;
    let (input, rgctx_entries_count) = le_i32(input)?;
    let (input, images_offset) = le_i32(input)?;
    let (input, images_count) = le_i32(input)?;
    let (input, assemblies_offset) = le_i32(input)?;
    let (input, assemblies_count) = le_i32(input)?;
    let (input, metadata_usage_lists_offset) = le_i32(input)?;
    let (input, metadata_usage_lists_count) = le_i32(input)?;
    let (input, metadata_usage_pairs_offset) = le_i32(input)?;
    let (input, metadata_usage_pairs_count) = le_i32(input)?;
    let (input, field_refs_offset) = le_i32(input)?;
    let (input, field_refs_count) = le_i32(input)?;
    let (input, referenced_assemblies_offset) = le_i32(input)?;
    let (input, referenced_assemblies_count) = le_i32(input)?;
    let (input, attributes_info_offset) = le_i32(input)?;
    let (input, attributes_info_count) = le_i32(input)?;
    let (input, attribute_types_offset) = le_i32(input)?;
    let (input, attribute_types_count) = le_i32(input)?;

    let header = Il2CppGlobalMetadataHeader {
        sanity,
        version,
        string_literal_offset,
        string_literal_count,
        string_literal_data_offset,
        string_literal_data_count,
        string_offset,
        string_count,
        events_offset,
        events_count,
        properties_offset,
        properties_count,
        methods_offset,
        methods_count,
        parameter_default_values_offset,
        parameter_default_values_count,
        field_default_values_offset,
        field_default_values_count,
        field_and_parameter_default_value_data_offset,
        field_and_parameter_default_value_data_count,
        field_marshaled_sizes_offset,
        field_marshaled_sizes_count,
        parameters_offset,
        parameters_count,
        fields_offset,
        fields_count,
        generic_parameters_offset,
        generic_parameters_count,
        generic_parameter_constraints_offset,
        generic_parameter_constraints_count,
        generic_containers_offset,
        generic_containers_count,
        nested_types_offset,
        nested_types_count,
        interfaces_offset,
        interfaces_count,
        vtable_methods_offset,
        vtable_methods_count,
        interface_offsets_offset,
        interface_offsets_count,
        type_definitions_offset,
        type_definitions_count,
        rgctx_entries_offset,
        rgctx_entries_count,
        images_offset,
        images_count,
        assemblies_offset,
        assemblies_count,
        metadata_usage_lists_offset,
        metadata_usage_lists_count,
        metadata_usage_pairs_offset,
        metadata_usage_pairs_count,
        field_refs_offset,
        field_refs_count,
        referenced_assemblies_offset,
        referenced_assemblies_count,
        attributes_info_offset,
        attributes_info_count,
        attribute_types_offset,
        attribute_types_count,
    };

    Ok((input, header))
}

pub fn parse_il2cpp_image_definition(input: &[u8]) -> IResult<&[u8], Il2CppImageDefinition> {
    let (input, name_index) = le_i32(input)?;
    let (input, assembly_index) = le_i32(input)?;
    let (input, type_start) = le_i32(input)?;
    let (input, type_count) = le_u32(input)?;
    let (input, entry_point_index) = le_i32(input)?;
    let (input, token) = le_u32(input)?;

    let image_definition = Il2CppImageDefinition {
        name_index,
        assembly_index,
        type_start,
        type_count,
        entry_point_index,
        token,
    };

    Ok((input, image_definition))
}

pub fn parse_il2cpp_type_definition(input: &[u8]) -> IResult<&[u8], Il2CppTypeDefinition> {
    let (input, name_index) = le_i32(input)?;
    let (input, namespace_index) = le_i32(input)?;
    let (input, custom_attribute_index) = le_i32(input)?;
    let (input, byval_type_index) = le_i32(input)?;
    let (input, byref_type_index) = le_i32(input)?;
    let (input, declaring_type_index) = le_i32(input)?;
    let (input, parent_index) = le_i32(input)?;
    let (input, element_type_index) = le_i32(input)?;
    let (input, rgctx_start_index) = le_i32(input)?;
    let (input, rgctx_count) = le_i32(input)?;
    let (input, generic_container_index) = le_i32(input)?;
    let (input, delegate_wrapper_from_managed_to_native_index) = le_i32(input)?;
    let (input, marshaling_functions_index) = le_i32(input)?;
    let (input, ccw_function_index) = le_i32(input)?;
    let (input, guid_index) = le_i32(input)?;
    let (input, flags) = le_u32(input)?;
    let (input, field_start) = le_i32(input)?;
    let (input, method_start) = le_i32(input)?;
    let (input, event_start) = le_i32(input)?;
    let (input, property_start) = le_i32(input)?;
    let (input, nested_types_start) = le_i32(input)?;
    let (input, interfaces_start) = le_i32(input)?;
    let (input, vtable_start) = le_i32(input)?;
    let (input, interface_offsets_start) = le_i32(input)?;
    let (input, method_count) = le_u16(input)?;
    let (input, property_count) = le_u16(input)?;
    let (input, field_count) = le_u16(input)?;
    let (input, event_count) = le_u16(input)?;
    let (input, nested_type_count) = le_u16(input)?;
    let (input, vtable_count) = le_u16(input)?;
    let (input, interfaces_count) = le_u16(input)?;
    let (input, interface_offsets_count) = le_u16(input)?;
    let (input, bitfield) = le_u32(input)?;
    let (input, token) = le_u32(input)?;

    let type_definition = Il2CppTypeDefinition {
        name_index,
        namespace_index,
        custom_attribute_index,
        byval_type_index,
        byref_type_index,
        declaring_type_index,
        parent_index,
        element_type_index,
        rgctx_start_index,
        rgctx_count,
        generic_container_index,
        delegate_wrapper_from_managed_to_native_index,
        marshaling_functions_index,
        ccw_function_index,
        guid_index,
        flags,
        field_start: field_start,
        method_start: method_start,
        event_start: event_start,
        property_start: property_start,
        nested_types_start: nested_types_start,
        interfaces_start: interfaces_start,
        vtable_start: vtable_start,
        interface_offsets_start: interface_offsets_start,
        method_count,
        property_count,
        field_count,
        event_count,
        nested_type_count,
        vtable_count,
        interfaces_count,
        interface_offsets_count,
        bitfield,
        token,
    };

    Ok((input, type_definition))
}

// testing only

pub fn parse_il2cpp_code_registration(input: &[u8]) -> IResult<&[u8], Il2CppCodeRegistration> {
    let (input, method_pointers_count) = le_u32(input)?;
    let (input, method_pointers) = le_u64(input)?;
    let (input, delegate_wrappers_from_native_to_managed_count) = le_u32(input)?;
    let (input, delegate_wrappers_from_native_to_managed) = le_u64(input)?;
    let (input, delegate_wrappers_from_managed_to_native_count) = le_u32(input)?;
    let (input, delegate_wrappers_from_managed_to_native) = le_u64(input)?;
    let (input, marshaling_functions_count) = le_u32(input)?;
    let (input, marshaling_functions) = le_u64(input)?;
    let (input, ccw_marshaling_functions_count) = le_u32(input)?;
    let (input, ccw_marshaling_functions) = le_u64(input)?;
    let (input, generic_method_pointers_count) = le_u32(input)?;
    let (input, generic_method_pointers) = le_u64(input)?;
    let (input, invoker_pointers_count) = le_u32(input)?;
    let (input, invoker_pointers) = le_u64(input)?;
    let (input, custom_attribute_count) = le_u32(input)?;
    let (input, custom_attribute_generators) = le_u64(input)?;
    let (input, guid_count) = le_u32(input)?;
    let (input, guids) = le_u64(input)?;

    let code_registration = Il2CppCodeRegistration {
        method_pointers_count,
        method_pointers: method_pointers as *mut u32,
        delegate_wrappers_from_native_to_managed_count,
        delegate_wrappers_from_native_to_managed: delegate_wrappers_from_native_to_managed
            as *mut *mut u32,
        delegate_wrappers_from_managed_to_native_count,
        delegate_wrappers_from_managed_to_native: delegate_wrappers_from_managed_to_native
            as *mut u32,
        marshaling_functions_count,
        marshaling_functions: marshaling_functions as *const Il2CppMarshalingFunctions,
        ccw_marshaling_functions_count,
        ccw_marshaling_functions: ccw_marshaling_functions as *mut u32,
        generic_method_pointers_count,
        generic_method_pointers: generic_method_pointers as *mut u32,
        invoker_pointers_count,
        invoker_pointers: invoker_pointers as *mut u32,
        custom_attribute_count: (custom_attribute_count as i32),
        custom_attribute_generators: custom_attribute_generators as *mut u32,
        guid_count: (guid_count as i32),
        guids: guids as *const *const std::ffi::c_void,
    };

    Ok((input, code_registration))
}

pub fn parse_il2cpp_method_definition(input: &[u8]) -> IResult<&[u8], Il2CppMethodDefinition> {
    let (input, name_index) = le_i32(input)?;
    let (input, declaring_type) = le_i32(input)?;
    let (input, return_type) = le_i32(input)?;
    let (input, parameter_start) = le_i32(input)?;
    let (input, custom_attribute_index) = le_i32(input)?;
    let (input, generic_container_index) = le_i32(input)?;
    let (input, method_index) = le_i32(input)?;
    let (input, invoker_index) = le_i32(input)?;
    let (input, delegate_wrapper_index) = le_i32(input)?;
    let (input, rgctx_start_index) = le_i32(input)?;
    let (input, rgctx_count) = le_i32(input)?;
    let (input, token) = le_u32(input)?;
    let (input, flags) = le_u16(input)?;
    let (input, iflags) = le_u16(input)?;
    let (input, slot) = le_u16(input)?;
    let (input, parameter_count) = le_u16(input)?;

    let method_definition = Il2CppMethodDefinition {
        name_index,
        declaring_type,
        return_type,
        parameter_start,
        custom_attribute_index,
        generic_container_index,
        method_index,
        invoker_index,
        delegate_wrapper_index,
        rgctx_start_index,
        rgctx_count,
        token,
        flags,
        iflags,
        slot,
        parameter_count,
    };

    Ok((input, method_definition))
}

pub fn parse_il2cpp_parameter_definition(
    input: &[u8],
) -> IResult<&[u8], Il2CppParameterDefinition> {
    let (input, name_index) = le_i32(input)?;
    let (input, token) = le_u32(input)?;
    let (input, custom_attribute_index) = le_i32(input)?;
    let (input, type_index) = le_i32(input)?;

    let parameter_definition = Il2CppParameterDefinition {
        name_index,
        token,
        custom_attribute_index,
        type_index,
    };

    Ok((input, parameter_definition))
}

pub fn parse_il2cpp_field_definition(input: &[u8]) -> IResult<&[u8], Il2CppFieldDefinition> {
    let (input, name_index) = le_i32(input)?;
    let (input, type_index) = le_i32(input)?;
    let (input, custom_attribute_index) = le_i32(input)?;
    let (input, token) = le_u32(input)?;

    let field_definition = Il2CppFieldDefinition {
        name_index,
        type_index,
        custom_attribute_index,
        token,
    };

    Ok((input, field_definition))
}

pub fn parse_il2cpp_field_default_value(input: &[u8]) -> IResult<&[u8], Il2CppFieldDefaultValue> {
    let (input, field_index) = le_i32(input)?;
    let (input, type_index) = le_i32(input)?;
    let (input, data_index) = le_i32(input)?;

    let field_default_value = Il2CppFieldDefaultValue {
        field_index,
        type_index,
        data_index,
    };

    Ok((input, field_default_value))
}
