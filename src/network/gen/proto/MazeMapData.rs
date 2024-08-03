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

//! Generated file from `MazeMapData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MazeMapData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MazeMapData {
    // message fields
    // @@protoc_insertion_point(field:MazeMapData.maze_group_list)
    pub maze_group_list: ::std::vec::Vec<super::MazeGroup::MazeGroup>,
    // @@protoc_insertion_point(field:MazeMapData.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:MazeMapData.unlocked_chest_list)
    pub unlocked_chest_list: ::std::vec::Vec<super::MazeChest::MazeChest>,
    // @@protoc_insertion_point(field:MazeMapData.lighten_section_list)
    pub lighten_section_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MazeMapData.maze_prop_list)
    pub maze_prop_list: ::std::vec::Vec<super::MazeProp::MazeProp>,
    // @@protoc_insertion_point(field:MazeMapData.unlocked_teleport_list)
    pub unlocked_teleport_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MazeMapData.content_id)
    pub content_id: u32,
    // @@protoc_insertion_point(field:MazeMapData.entry_id)
    pub entry_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MazeMapData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MazeMapData {
    fn default() -> &'a MazeMapData {
        <MazeMapData as ::protobuf::Message>::default_instance()
    }
}

impl MazeMapData {
    pub fn new() -> MazeMapData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "maze_group_list",
            |m: &MazeMapData| { &m.maze_group_list },
            |m: &mut MazeMapData| { &mut m.maze_group_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MazeMapData| { &m.retcode },
            |m: &mut MazeMapData| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_chest_list",
            |m: &MazeMapData| { &m.unlocked_chest_list },
            |m: &mut MazeMapData| { &mut m.unlocked_chest_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "lighten_section_list",
            |m: &MazeMapData| { &m.lighten_section_list },
            |m: &mut MazeMapData| { &mut m.lighten_section_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "maze_prop_list",
            |m: &MazeMapData| { &m.maze_prop_list },
            |m: &mut MazeMapData| { &mut m.maze_prop_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_teleport_list",
            |m: &MazeMapData| { &m.unlocked_teleport_list },
            |m: &mut MazeMapData| { &mut m.unlocked_teleport_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "content_id",
            |m: &MazeMapData| { &m.content_id },
            |m: &mut MazeMapData| { &mut m.content_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entry_id",
            |m: &MazeMapData| { &m.entry_id },
            |m: &mut MazeMapData| { &mut m.entry_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MazeMapData>(
            "MazeMapData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MazeMapData {
    const NAME: &'static str = "MazeMapData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.maze_group_list.push(is.read_message()?);
                },
                56 => {
                    self.retcode = is.read_uint32()?;
                },
                26 => {
                    self.unlocked_chest_list.push(is.read_message()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.lighten_section_list)?;
                },
                40 => {
                    self.lighten_section_list.push(is.read_uint32()?);
                },
                50 => {
                    self.maze_prop_list.push(is.read_message()?);
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.unlocked_teleport_list)?;
                },
                112 => {
                    self.unlocked_teleport_list.push(is.read_uint32()?);
                },
                8 => {
                    self.content_id = is.read_uint32()?;
                },
                32 => {
                    self.entry_id = is.read_uint32()?;
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
        for value in &self.maze_group_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.retcode);
        }
        for value in &self.unlocked_chest_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.lighten_section_list {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for value in &self.maze_prop_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.unlocked_teleport_list {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        if self.content_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.content_id);
        }
        if self.entry_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.entry_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.maze_group_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(7, self.retcode)?;
        }
        for v in &self.unlocked_chest_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.lighten_section_list {
            os.write_uint32(5, *v)?;
        };
        for v in &self.maze_prop_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for v in &self.unlocked_teleport_list {
            os.write_uint32(14, *v)?;
        };
        if self.content_id != 0 {
            os.write_uint32(1, self.content_id)?;
        }
        if self.entry_id != 0 {
            os.write_uint32(4, self.entry_id)?;
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

    fn new() -> MazeMapData {
        MazeMapData::new()
    }

    fn clear(&mut self) {
        self.maze_group_list.clear();
        self.retcode = 0;
        self.unlocked_chest_list.clear();
        self.lighten_section_list.clear();
        self.maze_prop_list.clear();
        self.unlocked_teleport_list.clear();
        self.content_id = 0;
        self.entry_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MazeMapData {
        static instance: MazeMapData = MazeMapData {
            maze_group_list: ::std::vec::Vec::new(),
            retcode: 0,
            unlocked_chest_list: ::std::vec::Vec::new(),
            lighten_section_list: ::std::vec::Vec::new(),
            maze_prop_list: ::std::vec::Vec::new(),
            unlocked_teleport_list: ::std::vec::Vec::new(),
            content_id: 0,
            entry_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MazeMapData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MazeMapData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MazeMapData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MazeMapData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MazeMapData.proto\x1a\x0fMazeGroup.proto\x1a\x0fMazeChest.proto\
    \x1a\x0eMazeProp.proto\"\xea\x02\n\x0bMazeMapData\x122\n\x0fmaze_group_l\
    ist\x18\x0b\x20\x03(\x0b2\n.MazeGroupR\rmazeGroupList\x12\x18\n\x07retco\
    de\x18\x07\x20\x01(\rR\x07retcode\x12:\n\x13unlocked_chest_list\x18\x03\
    \x20\x03(\x0b2\n.MazeChestR\x11unlockedChestList\x120\n\x14lighten_secti\
    on_list\x18\x05\x20\x03(\rR\x12lightenSectionList\x12/\n\x0emaze_prop_li\
    st\x18\x06\x20\x03(\x0b2\t.MazePropR\x0cmazePropList\x124\n\x16unlocked_\
    teleport_list\x18\x0e\x20\x03(\rR\x14unlockedTeleportList\x12\x1d\n\ncon\
    tent_id\x18\x01\x20\x01(\rR\tcontentId\x12\x19\n\x08entry_id\x18\x04\x20\
    \x01(\rR\x07entryIdB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::MazeGroup::file_descriptor().clone());
            deps.push(super::MazeChest::file_descriptor().clone());
            deps.push(super::MazeProp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MazeMapData::generated_message_descriptor_data());
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
