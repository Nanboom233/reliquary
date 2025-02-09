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

//! Generated file from `CDGPKIJBFDK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CDGPKIJBFDK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CDGPKIJBFDK {
    // message oneof groups
    pub HNMDAGKACJF: ::std::option::Option<cdgpkijbfdk::HNMDAGKACJF>,
    // special fields
    // @@protoc_insertion_point(special_field:CDGPKIJBFDK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CDGPKIJBFDK {
    fn default() -> &'a CDGPKIJBFDK {
        <CDGPKIJBFDK as ::protobuf::Message>::default_instance()
    }
}

impl CDGPKIJBFDK {
    pub fn new() -> CDGPKIJBFDK {
        ::std::default::Default::default()
    }

    // .FJLHHGACNGD JBEOEFLHDPM = 101;

    pub fn JBEOEFLHDPM(&self) -> &super::FJLHHGACNGD::FJLHHGACNGD {
        match self.HNMDAGKACJF {
            ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(ref v)) => v,
            _ => <super::FJLHHGACNGD::FJLHHGACNGD as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JBEOEFLHDPM(&mut self) {
        self.HNMDAGKACJF = ::std::option::Option::None;
    }

    pub fn has_JBEOEFLHDPM(&self) -> bool {
        match self.HNMDAGKACJF {
            ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JBEOEFLHDPM(&mut self, v: super::FJLHHGACNGD::FJLHHGACNGD) {
        self.HNMDAGKACJF = ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JBEOEFLHDPM(&mut self) -> &mut super::FJLHHGACNGD::FJLHHGACNGD {
        if let ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(_)) = self.HNMDAGKACJF {
        } else {
            self.HNMDAGKACJF = ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(super::FJLHHGACNGD::FJLHHGACNGD::new()));
        }
        match self.HNMDAGKACJF {
            ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JBEOEFLHDPM(&mut self) -> super::FJLHHGACNGD::FJLHHGACNGD {
        if self.has_JBEOEFLHDPM() {
            match self.HNMDAGKACJF.take() {
                ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FJLHHGACNGD::FJLHHGACNGD::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FJLHHGACNGD::FJLHHGACNGD>(
            "JBEOEFLHDPM",
            CDGPKIJBFDK::has_JBEOEFLHDPM,
            CDGPKIJBFDK::JBEOEFLHDPM,
            CDGPKIJBFDK::mut_JBEOEFLHDPM,
            CDGPKIJBFDK::set_JBEOEFLHDPM,
        ));
        oneofs.push(cdgpkijbfdk::HNMDAGKACJF::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CDGPKIJBFDK>(
            "CDGPKIJBFDK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CDGPKIJBFDK {
    const NAME: &'static str = "CDGPKIJBFDK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                810 => {
                    self.HNMDAGKACJF = ::std::option::Option::Some(cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.HNMDAGKACJF {
            match v {
                &cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.HNMDAGKACJF {
            match v {
                &cdgpkijbfdk::HNMDAGKACJF::JBEOEFLHDPM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(101, v, os)?;
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

    fn new() -> CDGPKIJBFDK {
        CDGPKIJBFDK::new()
    }

    fn clear(&mut self) {
        self.HNMDAGKACJF = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CDGPKIJBFDK {
        static instance: CDGPKIJBFDK = CDGPKIJBFDK {
            HNMDAGKACJF: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CDGPKIJBFDK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CDGPKIJBFDK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CDGPKIJBFDK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDGPKIJBFDK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CDGPKIJBFDK`
pub mod cdgpkijbfdk {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CDGPKIJBFDK.HNMDAGKACJF)
    pub enum HNMDAGKACJF {
        // @@protoc_insertion_point(oneof_field:CDGPKIJBFDK.JBEOEFLHDPM)
        JBEOEFLHDPM(super::super::FJLHHGACNGD::FJLHHGACNGD),
    }

    impl ::protobuf::Oneof for HNMDAGKACJF {
    }

    impl ::protobuf::OneofFull for HNMDAGKACJF {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CDGPKIJBFDK as ::protobuf::MessageFull>::descriptor().oneof_by_name("HNMDAGKACJF").unwrap()).clone()
        }
    }

    impl HNMDAGKACJF {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<HNMDAGKACJF>("HNMDAGKACJF")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CDGPKIJBFDK.proto\x1a\x11FJLHHGACNGD.proto\"N\n\x0bCDGPKIJBFDK\x12\
    0\n\x0bJBEOEFLHDPM\x18e\x20\x01(\x0b2\x0c.FJLHHGACNGDH\0R\x0bJBEOEFLHDPM\
    B\r\n\x0bHNMDAGKACJFb\x06proto3\
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
            deps.push(super::FJLHHGACNGD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CDGPKIJBFDK::generated_message_descriptor_data());
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
