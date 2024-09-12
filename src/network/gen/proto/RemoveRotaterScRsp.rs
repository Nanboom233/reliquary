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

//! Generated file from `RemoveRotaterScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RemoveRotaterScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RemoveRotaterScRsp {
    // message fields
    // @@protoc_insertion_point(field:RemoveRotaterScRsp.rotater_data)
    pub rotater_data: ::protobuf::MessageField<super::RotaterData::RotaterData>,
    // @@protoc_insertion_point(field:RemoveRotaterScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:RemoveRotaterScRsp.energy_info)
    pub energy_info: ::protobuf::MessageField<super::RotatorEnergyInfo::RotatorEnergyInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:RemoveRotaterScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RemoveRotaterScRsp {
    fn default() -> &'a RemoveRotaterScRsp {
        <RemoveRotaterScRsp as ::protobuf::Message>::default_instance()
    }
}

impl RemoveRotaterScRsp {
    pub fn new() -> RemoveRotaterScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RotaterData::RotaterData>(
            "rotater_data",
            |m: &RemoveRotaterScRsp| { &m.rotater_data },
            |m: &mut RemoveRotaterScRsp| { &mut m.rotater_data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &RemoveRotaterScRsp| { &m.retcode },
            |m: &mut RemoveRotaterScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RotatorEnergyInfo::RotatorEnergyInfo>(
            "energy_info",
            |m: &RemoveRotaterScRsp| { &m.energy_info },
            |m: &mut RemoveRotaterScRsp| { &mut m.energy_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RemoveRotaterScRsp>(
            "RemoveRotaterScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RemoveRotaterScRsp {
    const NAME: &'static str = "RemoveRotaterScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rotater_data)?;
                },
                120 => {
                    self.retcode = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.energy_info)?;
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
        if let Some(v) = self.rotater_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.retcode);
        }
        if let Some(v) = self.energy_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.rotater_data.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(15, self.retcode)?;
        }
        if let Some(v) = self.energy_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> RemoveRotaterScRsp {
        RemoveRotaterScRsp::new()
    }

    fn clear(&mut self) {
        self.rotater_data.clear();
        self.retcode = 0;
        self.energy_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RemoveRotaterScRsp {
        static instance: RemoveRotaterScRsp = RemoveRotaterScRsp {
            rotater_data: ::protobuf::MessageField::none(),
            retcode: 0,
            energy_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RemoveRotaterScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RemoveRotaterScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RemoveRotaterScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveRotaterScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18RemoveRotaterScRsp.proto\x1a\x11RotaterData.proto\x1a\x17RotatorEn\
    ergyInfo.proto\"\x94\x01\n\x12RemoveRotaterScRsp\x12/\n\x0crotater_data\
    \x18\x07\x20\x01(\x0b2\x0c.RotaterDataR\x0brotaterData\x12\x18\n\x07retc\
    ode\x18\x0f\x20\x01(\rR\x07retcode\x123\n\x0benergy_info\x18\x06\x20\x01\
    (\x0b2\x12.RotatorEnergyInfoR\nenergyInfoB\x15\n\x13emu.lunarcore.protob\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::RotaterData::file_descriptor().clone());
            deps.push(super::RotatorEnergyInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RemoveRotaterScRsp::generated_message_descriptor_data());
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
