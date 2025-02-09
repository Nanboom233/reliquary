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

//! Generated file from `UpdatePsnSettingsInfoCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UpdatePsnSettingsInfoCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdatePsnSettingsInfoCsReq {
    // message oneof groups
    pub KINPPGKIOFD: ::std::option::Option<update_psn_settings_info_cs_req::KINPPGKIOFD>,
    // special fields
    // @@protoc_insertion_point(special_field:UpdatePsnSettingsInfoCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdatePsnSettingsInfoCsReq {
    fn default() -> &'a UpdatePsnSettingsInfoCsReq {
        <UpdatePsnSettingsInfoCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdatePsnSettingsInfoCsReq {
    pub fn new() -> UpdatePsnSettingsInfoCsReq {
        ::std::default::Default::default()
    }

    // .HGGIAPIOCLG OPEGBHOPKCN = 1103;

    pub fn OPEGBHOPKCN(&self) -> &super::HGGIAPIOCLG::HGGIAPIOCLG {
        match self.KINPPGKIOFD {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(ref v)) => v,
            _ => <super::HGGIAPIOCLG::HGGIAPIOCLG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OPEGBHOPKCN(&mut self) {
        self.KINPPGKIOFD = ::std::option::Option::None;
    }

    pub fn has_OPEGBHOPKCN(&self) -> bool {
        match self.KINPPGKIOFD {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OPEGBHOPKCN(&mut self, v: super::HGGIAPIOCLG::HGGIAPIOCLG) {
        self.KINPPGKIOFD = ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OPEGBHOPKCN(&mut self) -> &mut super::HGGIAPIOCLG::HGGIAPIOCLG {
        if let ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(_)) = self.KINPPGKIOFD {
        } else {
            self.KINPPGKIOFD = ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(super::HGGIAPIOCLG::HGGIAPIOCLG::new()));
        }
        match self.KINPPGKIOFD {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OPEGBHOPKCN(&mut self) -> super::HGGIAPIOCLG::HGGIAPIOCLG {
        if self.has_OPEGBHOPKCN() {
            match self.KINPPGKIOFD.take() {
                ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HGGIAPIOCLG::HGGIAPIOCLG::new()
        }
    }

    // .HFAMBFECKLI MHBFNKMFGCG = 673;

    pub fn MHBFNKMFGCG(&self) -> &super::HFAMBFECKLI::HFAMBFECKLI {
        match self.KINPPGKIOFD {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(ref v)) => v,
            _ => <super::HFAMBFECKLI::HFAMBFECKLI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MHBFNKMFGCG(&mut self) {
        self.KINPPGKIOFD = ::std::option::Option::None;
    }

    pub fn has_MHBFNKMFGCG(&self) -> bool {
        match self.KINPPGKIOFD {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MHBFNKMFGCG(&mut self, v: super::HFAMBFECKLI::HFAMBFECKLI) {
        self.KINPPGKIOFD = ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MHBFNKMFGCG(&mut self) -> &mut super::HFAMBFECKLI::HFAMBFECKLI {
        if let ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(_)) = self.KINPPGKIOFD {
        } else {
            self.KINPPGKIOFD = ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(super::HFAMBFECKLI::HFAMBFECKLI::new()));
        }
        match self.KINPPGKIOFD {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MHBFNKMFGCG(&mut self) -> super::HFAMBFECKLI::HFAMBFECKLI {
        if self.has_MHBFNKMFGCG() {
            match self.KINPPGKIOFD.take() {
                ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HFAMBFECKLI::HFAMBFECKLI::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HGGIAPIOCLG::HGGIAPIOCLG>(
            "OPEGBHOPKCN",
            UpdatePsnSettingsInfoCsReq::has_OPEGBHOPKCN,
            UpdatePsnSettingsInfoCsReq::OPEGBHOPKCN,
            UpdatePsnSettingsInfoCsReq::mut_OPEGBHOPKCN,
            UpdatePsnSettingsInfoCsReq::set_OPEGBHOPKCN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HFAMBFECKLI::HFAMBFECKLI>(
            "MHBFNKMFGCG",
            UpdatePsnSettingsInfoCsReq::has_MHBFNKMFGCG,
            UpdatePsnSettingsInfoCsReq::MHBFNKMFGCG,
            UpdatePsnSettingsInfoCsReq::mut_MHBFNKMFGCG,
            UpdatePsnSettingsInfoCsReq::set_MHBFNKMFGCG,
        ));
        oneofs.push(update_psn_settings_info_cs_req::KINPPGKIOFD::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdatePsnSettingsInfoCsReq>(
            "UpdatePsnSettingsInfoCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdatePsnSettingsInfoCsReq {
    const NAME: &'static str = "UpdatePsnSettingsInfoCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8826 => {
                    self.KINPPGKIOFD = ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(is.read_message()?));
                },
                5386 => {
                    self.KINPPGKIOFD = ::std::option::Option::Some(update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.KINPPGKIOFD {
            match v {
                &update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.KINPPGKIOFD {
            match v {
                &update_psn_settings_info_cs_req::KINPPGKIOFD::OPEGBHOPKCN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1103, v, os)?;
                },
                &update_psn_settings_info_cs_req::KINPPGKIOFD::MHBFNKMFGCG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(673, v, os)?;
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

    fn new() -> UpdatePsnSettingsInfoCsReq {
        UpdatePsnSettingsInfoCsReq::new()
    }

    fn clear(&mut self) {
        self.KINPPGKIOFD = ::std::option::Option::None;
        self.KINPPGKIOFD = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdatePsnSettingsInfoCsReq {
        static instance: UpdatePsnSettingsInfoCsReq = UpdatePsnSettingsInfoCsReq {
            KINPPGKIOFD: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdatePsnSettingsInfoCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdatePsnSettingsInfoCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdatePsnSettingsInfoCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdatePsnSettingsInfoCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `UpdatePsnSettingsInfoCsReq`
pub mod update_psn_settings_info_cs_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:UpdatePsnSettingsInfoCsReq.KINPPGKIOFD)
    pub enum KINPPGKIOFD {
        // @@protoc_insertion_point(oneof_field:UpdatePsnSettingsInfoCsReq.OPEGBHOPKCN)
        OPEGBHOPKCN(super::super::HGGIAPIOCLG::HGGIAPIOCLG),
        // @@protoc_insertion_point(oneof_field:UpdatePsnSettingsInfoCsReq.MHBFNKMFGCG)
        MHBFNKMFGCG(super::super::HFAMBFECKLI::HFAMBFECKLI),
    }

    impl ::protobuf::Oneof for KINPPGKIOFD {
    }

    impl ::protobuf::OneofFull for KINPPGKIOFD {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::UpdatePsnSettingsInfoCsReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("KINPPGKIOFD").unwrap()).clone()
        }
    }

    impl KINPPGKIOFD {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<KINPPGKIOFD>("KINPPGKIOFD")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20UpdatePsnSettingsInfoCsReq.proto\x1a\x11HFAMBFECKLI.proto\x1a\x11H\
    GGIAPIOCLG.proto\"\x91\x01\n\x1aUpdatePsnSettingsInfoCsReq\x121\n\x0bOPE\
    GBHOPKCN\x18\xcf\x08\x20\x01(\x0b2\x0c.HGGIAPIOCLGH\0R\x0bOPEGBHOPKCN\
    \x121\n\x0bMHBFNKMFGCG\x18\xa1\x05\x20\x01(\x0b2\x0c.HFAMBFECKLIH\0R\x0b\
    MHBFNKMFGCGB\r\n\x0bKINPPGKIOFDb\x06proto3\
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
            deps.push(super::HFAMBFECKLI::file_descriptor().clone());
            deps.push(super::HGGIAPIOCLG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdatePsnSettingsInfoCsReq::generated_message_descriptor_data());
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
