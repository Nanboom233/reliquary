// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `EntityBuffChangeInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EntityBuffChangeInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EntityBuffChangeInfo {
    // message fields
    // @@protoc_insertion_point(field:EntityBuffChangeInfo.add_buff_info)
    pub add_buff_info: ::protobuf::MessageField<super::BuffInfo::BuffInfo>,
    // @@protoc_insertion_point(field:EntityBuffChangeInfo.remove_buff_id)
    pub remove_buff_id: u32,
    // @@protoc_insertion_point(field:EntityBuffChangeInfo.entity_id)
    pub entity_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EntityBuffChangeInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EntityBuffChangeInfo {
    fn default() -> &'a EntityBuffChangeInfo {
        <EntityBuffChangeInfo as ::protobuf::Message>::default_instance()
    }
}

impl EntityBuffChangeInfo {
    pub fn new() -> EntityBuffChangeInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BuffInfo::BuffInfo>(
            "add_buff_info",
            |m: &EntityBuffChangeInfo| { &m.add_buff_info },
            |m: &mut EntityBuffChangeInfo| { &mut m.add_buff_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "remove_buff_id",
            |m: &EntityBuffChangeInfo| { &m.remove_buff_id },
            |m: &mut EntityBuffChangeInfo| { &mut m.remove_buff_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &EntityBuffChangeInfo| { &m.entity_id },
            |m: &mut EntityBuffChangeInfo| { &mut m.entity_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EntityBuffChangeInfo>(
            "EntityBuffChangeInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EntityBuffChangeInfo {
    const NAME: &'static str = "EntityBuffChangeInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.add_buff_info)?;
                },
                72 => {
                    self.remove_buff_id = is.read_uint32()?;
                },
                104 => {
                    self.entity_id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.add_buff_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.remove_buff_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.remove_buff_id);
        }
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.entity_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.add_buff_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.remove_buff_id != 0 {
            os.write_uint32(9, self.remove_buff_id)?;
        }
        if self.entity_id != 0 {
            os.write_uint32(13, self.entity_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> EntityBuffChangeInfo {
        EntityBuffChangeInfo::new()
    }

    fn clear(&mut self) {
        self.add_buff_info.clear();
        self.remove_buff_id = 0;
        self.entity_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EntityBuffChangeInfo {
        static instance: EntityBuffChangeInfo = EntityBuffChangeInfo {
            add_buff_info: ::protobuf::MessageField::none(),
            remove_buff_id: 0,
            entity_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EntityBuffChangeInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EntityBuffChangeInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EntityBuffChangeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EntityBuffChangeInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aEntityBuffChangeInfo.proto\x1a\x0eBuffInfo.proto\"\x88\x01\n\x14En\
    tityBuffChangeInfo\x12-\n\radd_buff_info\x18\x08\x20\x01(\x0b2\t.BuffInf\
    oR\x0baddBuffInfo\x12$\n\x0eremove_buff_id\x18\t\x20\x01(\rR\x0cremoveBu\
    ffId\x12\x1b\n\tentity_id\x18\r\x20\x01(\rR\x08entityIdB\x15\n\x13emu.lu\
    narcore.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::BuffInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EntityBuffChangeInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
