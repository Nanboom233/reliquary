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

//! Generated file from `RogueArea.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueArea)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueArea {
    // message fields
    // @@protoc_insertion_point(field:RogueArea.has_taken_rewards)
    pub has_taken_rewards: bool,
    // @@protoc_insertion_point(field:RogueArea.cur_reach_room_num)
    pub cur_reach_room_num: u32,
    // @@protoc_insertion_point(field:RogueArea.rogue_area_status)
    pub rogue_area_status: u32,
    // @@protoc_insertion_point(field:RogueArea.area_id)
    pub area_id: u32,
    // @@protoc_insertion_point(field:RogueArea.rogue_status)
    pub rogue_status: u32,
    // @@protoc_insertion_point(field:RogueArea.map_id)
    pub map_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueArea.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueArea {
    fn default() -> &'a RogueArea {
        <RogueArea as ::protobuf::Message>::default_instance()
    }
}

impl RogueArea {
    pub fn new() -> RogueArea {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "has_taken_rewards",
            |m: &RogueArea| { &m.has_taken_rewards },
            |m: &mut RogueArea| { &mut m.has_taken_rewards },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_reach_room_num",
            |m: &RogueArea| { &m.cur_reach_room_num },
            |m: &mut RogueArea| { &mut m.cur_reach_room_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rogue_area_status",
            |m: &RogueArea| { &m.rogue_area_status },
            |m: &mut RogueArea| { &mut m.rogue_area_status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "area_id",
            |m: &RogueArea| { &m.area_id },
            |m: &mut RogueArea| { &mut m.area_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rogue_status",
            |m: &RogueArea| { &m.rogue_status },
            |m: &mut RogueArea| { &mut m.rogue_status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "map_id",
            |m: &RogueArea| { &m.map_id },
            |m: &mut RogueArea| { &mut m.map_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueArea>(
            "RogueArea",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueArea {
    const NAME: &'static str = "RogueArea";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.has_taken_rewards = is.read_bool()?;
                },
                80 => {
                    self.cur_reach_room_num = is.read_uint32()?;
                },
                120 => {
                    self.rogue_area_status = is.read_uint32()?;
                },
                72 => {
                    self.area_id = is.read_uint32()?;
                },
                40 => {
                    self.rogue_status = is.read_uint32()?;
                },
                104 => {
                    self.map_id = is.read_uint32()?;
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
        if self.has_taken_rewards != false {
            my_size += 1 + 1;
        }
        if self.cur_reach_room_num != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.cur_reach_room_num);
        }
        if self.rogue_area_status != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.rogue_area_status);
        }
        if self.area_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.area_id);
        }
        if self.rogue_status != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.rogue_status);
        }
        if self.map_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.map_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.has_taken_rewards != false {
            os.write_bool(4, self.has_taken_rewards)?;
        }
        if self.cur_reach_room_num != 0 {
            os.write_uint32(10, self.cur_reach_room_num)?;
        }
        if self.rogue_area_status != 0 {
            os.write_uint32(15, self.rogue_area_status)?;
        }
        if self.area_id != 0 {
            os.write_uint32(9, self.area_id)?;
        }
        if self.rogue_status != 0 {
            os.write_uint32(5, self.rogue_status)?;
        }
        if self.map_id != 0 {
            os.write_uint32(13, self.map_id)?;
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

    fn new() -> RogueArea {
        RogueArea::new()
    }

    fn clear(&mut self) {
        self.has_taken_rewards = false;
        self.cur_reach_room_num = 0;
        self.rogue_area_status = 0;
        self.area_id = 0;
        self.rogue_status = 0;
        self.map_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueArea {
        static instance: RogueArea = RogueArea {
            has_taken_rewards: false,
            cur_reach_room_num: 0,
            rogue_area_status: 0,
            area_id: 0,
            rogue_status: 0,
            map_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueArea {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueArea").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueArea {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueArea {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fRogueArea.proto\"\xe3\x01\n\tRogueArea\x12*\n\x11has_taken_rewards\
    \x18\x04\x20\x01(\x08R\x0fhasTakenRewards\x12+\n\x12cur_reach_room_num\
    \x18\n\x20\x01(\rR\x0fcurReachRoomNum\x12*\n\x11rogue_area_status\x18\
    \x0f\x20\x01(\rR\x0frogueAreaStatus\x12\x17\n\x07area_id\x18\t\x20\x01(\
    \rR\x06areaId\x12!\n\x0crogue_status\x18\x05\x20\x01(\rR\x0brogueStatus\
    \x12\x15\n\x06map_id\x18\r\x20\x01(\rR\x05mapIdB\x15\n\x13emu.lunarcore.\
    protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueArea::generated_message_descriptor_data());
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
