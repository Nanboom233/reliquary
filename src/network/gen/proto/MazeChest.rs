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

//! Generated file from `MazeChest.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MazeChest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MazeChest {
    // message fields
    // @@protoc_insertion_point(field:MazeChest.map_info_chest_type)
    pub map_info_chest_type: ::protobuf::EnumOrUnknown<super::MapInfoChestType::MapInfoChestType>,
    // @@protoc_insertion_point(field:MazeChest.total_amount_list)
    pub total_amount_list: u32,
    // @@protoc_insertion_point(field:MazeChest.unlocked_amount_list)
    pub unlocked_amount_list: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MazeChest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MazeChest {
    fn default() -> &'a MazeChest {
        <MazeChest as ::protobuf::Message>::default_instance()
    }
}

impl MazeChest {
    pub fn new() -> MazeChest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "map_info_chest_type",
            |m: &MazeChest| { &m.map_info_chest_type },
            |m: &mut MazeChest| { &mut m.map_info_chest_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_amount_list",
            |m: &MazeChest| { &m.total_amount_list },
            |m: &mut MazeChest| { &mut m.total_amount_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unlocked_amount_list",
            |m: &MazeChest| { &m.unlocked_amount_list },
            |m: &mut MazeChest| { &mut m.unlocked_amount_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MazeChest>(
            "MazeChest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MazeChest {
    const NAME: &'static str = "MazeChest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.map_info_chest_type = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.total_amount_list = is.read_uint32()?;
                },
                96 => {
                    self.unlocked_amount_list = is.read_uint32()?;
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
        if self.map_info_chest_type != ::protobuf::EnumOrUnknown::new(super::MapInfoChestType::MapInfoChestType::MAP_INFO_CHEST_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.map_info_chest_type.value());
        }
        if self.total_amount_list != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.total_amount_list);
        }
        if self.unlocked_amount_list != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.unlocked_amount_list);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.map_info_chest_type != ::protobuf::EnumOrUnknown::new(super::MapInfoChestType::MapInfoChestType::MAP_INFO_CHEST_TYPE_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.map_info_chest_type))?;
        }
        if self.total_amount_list != 0 {
            os.write_uint32(9, self.total_amount_list)?;
        }
        if self.unlocked_amount_list != 0 {
            os.write_uint32(12, self.unlocked_amount_list)?;
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

    fn new() -> MazeChest {
        MazeChest::new()
    }

    fn clear(&mut self) {
        self.map_info_chest_type = ::protobuf::EnumOrUnknown::new(super::MapInfoChestType::MapInfoChestType::MAP_INFO_CHEST_TYPE_NONE);
        self.total_amount_list = 0;
        self.unlocked_amount_list = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MazeChest {
        static instance: MazeChest = MazeChest {
            map_info_chest_type: ::protobuf::EnumOrUnknown::from_i32(0),
            total_amount_list: 0,
            unlocked_amount_list: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MazeChest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MazeChest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MazeChest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MazeChest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fMazeChest.proto\x1a\x16MapInfoChestType.proto\"\xab\x01\n\tMazeChe\
    st\x12@\n\x13map_info_chest_type\x18\x04\x20\x01(\x0e2\x11.MapInfoChestT\
    ypeR\x10mapInfoChestType\x12*\n\x11total_amount_list\x18\t\x20\x01(\rR\
    \x0ftotalAmountList\x120\n\x14unlocked_amount_list\x18\x0c\x20\x01(\rR\
    \x12unlockedAmountListB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::MapInfoChestType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MazeChest::generated_message_descriptor_data());
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
