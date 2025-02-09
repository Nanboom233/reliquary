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

//! Generated file from `HFAMBFECKLI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HFAMBFECKLI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HFAMBFECKLI {
    // message oneof groups
    pub DPMDNMKAINH: ::std::option::Option<hfambfeckli::DPMDNMKAINH>,
    // special fields
    // @@protoc_insertion_point(special_field:HFAMBFECKLI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HFAMBFECKLI {
    fn default() -> &'a HFAMBFECKLI {
        <HFAMBFECKLI as ::protobuf::Message>::default_instance()
    }
}

impl HFAMBFECKLI {
    pub fn new() -> HFAMBFECKLI {
        ::std::default::Default::default()
    }

    // bool JFBNEDKKLDC = 1670;

    pub fn JFBNEDKKLDC(&self) -> bool {
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::JFBNEDKKLDC(v)) => v,
            _ => false,
        }
    }

    pub fn clear_JFBNEDKKLDC(&mut self) {
        self.DPMDNMKAINH = ::std::option::Option::None;
    }

    pub fn has_JFBNEDKKLDC(&self) -> bool {
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::JFBNEDKKLDC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JFBNEDKKLDC(&mut self, v: bool) {
        self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::JFBNEDKKLDC(v))
    }

    // .ICDBLDMJNDE HIIGMJOFKGE = 404;

    pub fn HIIGMJOFKGE(&self) -> &super::ICDBLDMJNDE::ICDBLDMJNDE {
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(ref v)) => v,
            _ => <super::ICDBLDMJNDE::ICDBLDMJNDE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_HIIGMJOFKGE(&mut self) {
        self.DPMDNMKAINH = ::std::option::Option::None;
    }

    pub fn has_HIIGMJOFKGE(&self) -> bool {
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_HIIGMJOFKGE(&mut self, v: super::ICDBLDMJNDE::ICDBLDMJNDE) {
        self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_HIIGMJOFKGE(&mut self) -> &mut super::ICDBLDMJNDE::ICDBLDMJNDE {
        if let ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(_)) = self.DPMDNMKAINH {
        } else {
            self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(super::ICDBLDMJNDE::ICDBLDMJNDE::new()));
        }
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_HIIGMJOFKGE(&mut self) -> super::ICDBLDMJNDE::ICDBLDMJNDE {
        if self.has_HIIGMJOFKGE() {
            match self.DPMDNMKAINH.take() {
                ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ICDBLDMJNDE::ICDBLDMJNDE::new()
        }
    }

    // .ICDBLDMJNDE FNPNOLNOFFM = 846;

    pub fn FNPNOLNOFFM(&self) -> &super::ICDBLDMJNDE::ICDBLDMJNDE {
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(ref v)) => v,
            _ => <super::ICDBLDMJNDE::ICDBLDMJNDE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FNPNOLNOFFM(&mut self) {
        self.DPMDNMKAINH = ::std::option::Option::None;
    }

    pub fn has_FNPNOLNOFFM(&self) -> bool {
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FNPNOLNOFFM(&mut self, v: super::ICDBLDMJNDE::ICDBLDMJNDE) {
        self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FNPNOLNOFFM(&mut self) -> &mut super::ICDBLDMJNDE::ICDBLDMJNDE {
        if let ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(_)) = self.DPMDNMKAINH {
        } else {
            self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(super::ICDBLDMJNDE::ICDBLDMJNDE::new()));
        }
        match self.DPMDNMKAINH {
            ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FNPNOLNOFFM(&mut self) -> super::ICDBLDMJNDE::ICDBLDMJNDE {
        if self.has_FNPNOLNOFFM() {
            match self.DPMDNMKAINH.take() {
                ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ICDBLDMJNDE::ICDBLDMJNDE::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "JFBNEDKKLDC",
            HFAMBFECKLI::has_JFBNEDKKLDC,
            HFAMBFECKLI::JFBNEDKKLDC,
            HFAMBFECKLI::set_JFBNEDKKLDC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ICDBLDMJNDE::ICDBLDMJNDE>(
            "HIIGMJOFKGE",
            HFAMBFECKLI::has_HIIGMJOFKGE,
            HFAMBFECKLI::HIIGMJOFKGE,
            HFAMBFECKLI::mut_HIIGMJOFKGE,
            HFAMBFECKLI::set_HIIGMJOFKGE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ICDBLDMJNDE::ICDBLDMJNDE>(
            "FNPNOLNOFFM",
            HFAMBFECKLI::has_FNPNOLNOFFM,
            HFAMBFECKLI::FNPNOLNOFFM,
            HFAMBFECKLI::mut_FNPNOLNOFFM,
            HFAMBFECKLI::set_FNPNOLNOFFM,
        ));
        oneofs.push(hfambfeckli::DPMDNMKAINH::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HFAMBFECKLI>(
            "HFAMBFECKLI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HFAMBFECKLI {
    const NAME: &'static str = "HFAMBFECKLI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                13360 => {
                    self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::JFBNEDKKLDC(is.read_bool()?));
                },
                3234 => {
                    self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(is.read_message()?));
                },
                6770 => {
                    self.DPMDNMKAINH = ::std::option::Option::Some(hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.DPMDNMKAINH {
            match v {
                &hfambfeckli::DPMDNMKAINH::JFBNEDKKLDC(v) => {
                    my_size += 2 + 1;
                },
                &hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.DPMDNMKAINH {
            match v {
                &hfambfeckli::DPMDNMKAINH::JFBNEDKKLDC(v) => {
                    os.write_bool(1670, v)?;
                },
                &hfambfeckli::DPMDNMKAINH::HIIGMJOFKGE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(404, v, os)?;
                },
                &hfambfeckli::DPMDNMKAINH::FNPNOLNOFFM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(846, v, os)?;
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

    fn new() -> HFAMBFECKLI {
        HFAMBFECKLI::new()
    }

    fn clear(&mut self) {
        self.DPMDNMKAINH = ::std::option::Option::None;
        self.DPMDNMKAINH = ::std::option::Option::None;
        self.DPMDNMKAINH = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HFAMBFECKLI {
        static instance: HFAMBFECKLI = HFAMBFECKLI {
            DPMDNMKAINH: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HFAMBFECKLI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HFAMBFECKLI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HFAMBFECKLI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HFAMBFECKLI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `HFAMBFECKLI`
pub mod hfambfeckli {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:HFAMBFECKLI.DPMDNMKAINH)
    pub enum DPMDNMKAINH {
        // @@protoc_insertion_point(oneof_field:HFAMBFECKLI.JFBNEDKKLDC)
        JFBNEDKKLDC(bool),
        // @@protoc_insertion_point(oneof_field:HFAMBFECKLI.HIIGMJOFKGE)
        HIIGMJOFKGE(super::super::ICDBLDMJNDE::ICDBLDMJNDE),
        // @@protoc_insertion_point(oneof_field:HFAMBFECKLI.FNPNOLNOFFM)
        FNPNOLNOFFM(super::super::ICDBLDMJNDE::ICDBLDMJNDE),
    }

    impl ::protobuf::Oneof for DPMDNMKAINH {
    }

    impl ::protobuf::OneofFull for DPMDNMKAINH {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::HFAMBFECKLI as ::protobuf::MessageFull>::descriptor().oneof_by_name("DPMDNMKAINH").unwrap()).clone()
        }
    }

    impl DPMDNMKAINH {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DPMDNMKAINH>("DPMDNMKAINH")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HFAMBFECKLI.proto\x1a\x11ICDBLDMJNDE.proto\"\xa7\x01\n\x0bHFAMBFEC\
    KLI\x12#\n\x0bJFBNEDKKLDC\x18\x86\r\x20\x01(\x08H\0R\x0bJFBNEDKKLDC\x121\
    \n\x0bHIIGMJOFKGE\x18\x94\x03\x20\x01(\x0b2\x0c.ICDBLDMJNDEH\0R\x0bHIIGM\
    JOFKGE\x121\n\x0bFNPNOLNOFFM\x18\xce\x06\x20\x01(\x0b2\x0c.ICDBLDMJNDEH\
    \0R\x0bFNPNOLNOFFMB\r\n\x0bDPMDNMKAINHb\x06proto3\
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
            deps.push(super::ICDBLDMJNDE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HFAMBFECKLI::generated_message_descriptor_data());
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
