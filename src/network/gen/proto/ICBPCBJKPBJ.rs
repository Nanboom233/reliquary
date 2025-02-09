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

//! Generated file from `ICBPCBJKPBJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ICBPCBJKPBJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ICBPCBJKPBJ {
    // message oneof groups
    pub DPHMLALGMFJ: ::std::option::Option<icbpcbjkpbj::DPHMLALGMFJ>,
    // special fields
    // @@protoc_insertion_point(special_field:ICBPCBJKPBJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ICBPCBJKPBJ {
    fn default() -> &'a ICBPCBJKPBJ {
        <ICBPCBJKPBJ as ::protobuf::Message>::default_instance()
    }
}

impl ICBPCBJKPBJ {
    pub fn new() -> ICBPCBJKPBJ {
        ::std::default::Default::default()
    }

    // .SceneEntityInfo OALKDEDCFDJ = 8;

    pub fn OALKDEDCFDJ(&self) -> &super::SceneEntityInfo::SceneEntityInfo {
        match self.DPHMLALGMFJ {
            ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(ref v)) => v,
            _ => <super::SceneEntityInfo::SceneEntityInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OALKDEDCFDJ(&mut self) {
        self.DPHMLALGMFJ = ::std::option::Option::None;
    }

    pub fn has_OALKDEDCFDJ(&self) -> bool {
        match self.DPHMLALGMFJ {
            ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OALKDEDCFDJ(&mut self, v: super::SceneEntityInfo::SceneEntityInfo) {
        self.DPHMLALGMFJ = ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OALKDEDCFDJ(&mut self) -> &mut super::SceneEntityInfo::SceneEntityInfo {
        if let ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(_)) = self.DPHMLALGMFJ {
        } else {
            self.DPHMLALGMFJ = ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(super::SceneEntityInfo::SceneEntityInfo::new()));
        }
        match self.DPHMLALGMFJ {
            ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OALKDEDCFDJ(&mut self) -> super::SceneEntityInfo::SceneEntityInfo {
        if self.has_OALKDEDCFDJ() {
            match self.DPHMLALGMFJ.take() {
                ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::SceneEntityInfo::SceneEntityInfo::new()
        }
    }

    // uint32 KGLMIMBICIN = 12;

    pub fn KGLMIMBICIN(&self) -> u32 {
        match self.DPHMLALGMFJ {
            ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::KGLMIMBICIN(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_KGLMIMBICIN(&mut self) {
        self.DPHMLALGMFJ = ::std::option::Option::None;
    }

    pub fn has_KGLMIMBICIN(&self) -> bool {
        match self.DPHMLALGMFJ {
            ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::KGLMIMBICIN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KGLMIMBICIN(&mut self, v: u32) {
        self.DPHMLALGMFJ = ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::KGLMIMBICIN(v))
    }

    // uint32 MELFHAOJLEL = 13;

    pub fn MELFHAOJLEL(&self) -> u32 {
        match self.DPHMLALGMFJ {
            ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::MELFHAOJLEL(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_MELFHAOJLEL(&mut self) {
        self.DPHMLALGMFJ = ::std::option::Option::None;
    }

    pub fn has_MELFHAOJLEL(&self) -> bool {
        match self.DPHMLALGMFJ {
            ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::MELFHAOJLEL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MELFHAOJLEL(&mut self, v: u32) {
        self.DPHMLALGMFJ = ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::MELFHAOJLEL(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::SceneEntityInfo::SceneEntityInfo>(
            "OALKDEDCFDJ",
            ICBPCBJKPBJ::has_OALKDEDCFDJ,
            ICBPCBJKPBJ::OALKDEDCFDJ,
            ICBPCBJKPBJ::mut_OALKDEDCFDJ,
            ICBPCBJKPBJ::set_OALKDEDCFDJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "KGLMIMBICIN",
            ICBPCBJKPBJ::has_KGLMIMBICIN,
            ICBPCBJKPBJ::KGLMIMBICIN,
            ICBPCBJKPBJ::set_KGLMIMBICIN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "MELFHAOJLEL",
            ICBPCBJKPBJ::has_MELFHAOJLEL,
            ICBPCBJKPBJ::MELFHAOJLEL,
            ICBPCBJKPBJ::set_MELFHAOJLEL,
        ));
        oneofs.push(icbpcbjkpbj::DPHMLALGMFJ::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ICBPCBJKPBJ>(
            "ICBPCBJKPBJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ICBPCBJKPBJ {
    const NAME: &'static str = "ICBPCBJKPBJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.DPHMLALGMFJ = ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(is.read_message()?));
                },
                96 => {
                    self.DPHMLALGMFJ = ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::KGLMIMBICIN(is.read_uint32()?));
                },
                104 => {
                    self.DPHMLALGMFJ = ::std::option::Option::Some(icbpcbjkpbj::DPHMLALGMFJ::MELFHAOJLEL(is.read_uint32()?));
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
        if let ::std::option::Option::Some(ref v) = self.DPHMLALGMFJ {
            match v {
                &icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &icbpcbjkpbj::DPHMLALGMFJ::KGLMIMBICIN(v) => {
                    my_size += ::protobuf::rt::uint32_size(12, v);
                },
                &icbpcbjkpbj::DPHMLALGMFJ::MELFHAOJLEL(v) => {
                    my_size += ::protobuf::rt::uint32_size(13, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.DPHMLALGMFJ {
            match v {
                &icbpcbjkpbj::DPHMLALGMFJ::OALKDEDCFDJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &icbpcbjkpbj::DPHMLALGMFJ::KGLMIMBICIN(v) => {
                    os.write_uint32(12, v)?;
                },
                &icbpcbjkpbj::DPHMLALGMFJ::MELFHAOJLEL(v) => {
                    os.write_uint32(13, v)?;
                },
            };
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

    fn new() -> ICBPCBJKPBJ {
        ICBPCBJKPBJ::new()
    }

    fn clear(&mut self) {
        self.DPHMLALGMFJ = ::std::option::Option::None;
        self.DPHMLALGMFJ = ::std::option::Option::None;
        self.DPHMLALGMFJ = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ICBPCBJKPBJ {
        static instance: ICBPCBJKPBJ = ICBPCBJKPBJ {
            DPHMLALGMFJ: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ICBPCBJKPBJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ICBPCBJKPBJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ICBPCBJKPBJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ICBPCBJKPBJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ICBPCBJKPBJ`
pub mod icbpcbjkpbj {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ICBPCBJKPBJ.DPHMLALGMFJ)
    pub enum DPHMLALGMFJ {
        // @@protoc_insertion_point(oneof_field:ICBPCBJKPBJ.OALKDEDCFDJ)
        OALKDEDCFDJ(super::super::SceneEntityInfo::SceneEntityInfo),
        // @@protoc_insertion_point(oneof_field:ICBPCBJKPBJ.KGLMIMBICIN)
        KGLMIMBICIN(u32),
        // @@protoc_insertion_point(oneof_field:ICBPCBJKPBJ.MELFHAOJLEL)
        MELFHAOJLEL(u32),
    }

    impl ::protobuf::Oneof for DPHMLALGMFJ {
    }

    impl ::protobuf::OneofFull for DPHMLALGMFJ {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ICBPCBJKPBJ as ::protobuf::MessageFull>::descriptor().oneof_by_name("DPHMLALGMFJ").unwrap()).clone()
        }
    }

    impl DPHMLALGMFJ {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DPHMLALGMFJ>("DPHMLALGMFJ")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ICBPCBJKPBJ.proto\x1a\x15SceneEntityInfo.proto\"\x9a\x01\n\x0bICBP\
    CBJKPBJ\x124\n\x0bOALKDEDCFDJ\x18\x08\x20\x01(\x0b2\x10.SceneEntityInfoH\
    \0R\x0bOALKDEDCFDJ\x12\"\n\x0bKGLMIMBICIN\x18\x0c\x20\x01(\rH\0R\x0bKGLM\
    IMBICIN\x12\"\n\x0bMELFHAOJLEL\x18\r\x20\x01(\rH\0R\x0bMELFHAOJLELB\r\n\
    \x0bDPHMLALGMFJb\x06proto3\
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
            deps.push(super::SceneEntityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ICBPCBJKPBJ::generated_message_descriptor_data());
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
