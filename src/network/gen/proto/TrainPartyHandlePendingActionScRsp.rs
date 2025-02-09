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

//! Generated file from `TrainPartyHandlePendingActionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TrainPartyHandlePendingActionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrainPartyHandlePendingActionScRsp {
    // message fields
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.FHOPJNKMNPB)
    pub FHOPJNKMNPB: u32,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.KLDKOBLFAAC)
    pub KLDKOBLFAAC: bool,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.CBIOEMAOPNC)
    pub CBIOEMAOPNC: ::protobuf::MessageField<super::LDPBKMPNJFG::LDPBKMPNJFG>,
    // message oneof groups
    pub PGOMJOKHFCN: ::std::option::Option<train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN>,
    // special fields
    // @@protoc_insertion_point(special_field:TrainPartyHandlePendingActionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrainPartyHandlePendingActionScRsp {
    fn default() -> &'a TrainPartyHandlePendingActionScRsp {
        <TrainPartyHandlePendingActionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TrainPartyHandlePendingActionScRsp {
    pub fn new() -> TrainPartyHandlePendingActionScRsp {
        ::std::default::Default::default()
    }

    // .FPFAFNFPJDK PPPBLDKJHBG = 158;

    pub fn PPPBLDKJHBG(&self) -> &super::FPFAFNFPJDK::FPFAFNFPJDK {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(ref v)) => v,
            _ => <super::FPFAFNFPJDK::FPFAFNFPJDK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_PPPBLDKJHBG(&mut self) {
        self.PGOMJOKHFCN = ::std::option::Option::None;
    }

    pub fn has_PPPBLDKJHBG(&self) -> bool {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PPPBLDKJHBG(&mut self, v: super::FPFAFNFPJDK::FPFAFNFPJDK) {
        self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_PPPBLDKJHBG(&mut self) -> &mut super::FPFAFNFPJDK::FPFAFNFPJDK {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(_)) = self.PGOMJOKHFCN {
        } else {
            self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(super::FPFAFNFPJDK::FPFAFNFPJDK::new()));
        }
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_PPPBLDKJHBG(&mut self) -> super::FPFAFNFPJDK::FPFAFNFPJDK {
        if self.has_PPPBLDKJHBG() {
            match self.PGOMJOKHFCN.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FPFAFNFPJDK::FPFAFNFPJDK::new()
        }
    }

    // .AKFEELJJOCI MPGFEGDIDDC = 1236;

    pub fn MPGFEGDIDDC(&self) -> &super::AKFEELJJOCI::AKFEELJJOCI {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(ref v)) => v,
            _ => <super::AKFEELJJOCI::AKFEELJJOCI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MPGFEGDIDDC(&mut self) {
        self.PGOMJOKHFCN = ::std::option::Option::None;
    }

    pub fn has_MPGFEGDIDDC(&self) -> bool {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MPGFEGDIDDC(&mut self, v: super::AKFEELJJOCI::AKFEELJJOCI) {
        self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MPGFEGDIDDC(&mut self) -> &mut super::AKFEELJJOCI::AKFEELJJOCI {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(_)) = self.PGOMJOKHFCN {
        } else {
            self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(super::AKFEELJJOCI::AKFEELJJOCI::new()));
        }
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MPGFEGDIDDC(&mut self) -> super::AKFEELJJOCI::AKFEELJJOCI {
        if self.has_MPGFEGDIDDC() {
            match self.PGOMJOKHFCN.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::AKFEELJJOCI::AKFEELJJOCI::new()
        }
    }

    // .HCOHPHFOHAE NNAMFMIPNKN = 1980;

    pub fn NNAMFMIPNKN(&self) -> &super::HCOHPHFOHAE::HCOHPHFOHAE {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(ref v)) => v,
            _ => <super::HCOHPHFOHAE::HCOHPHFOHAE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NNAMFMIPNKN(&mut self) {
        self.PGOMJOKHFCN = ::std::option::Option::None;
    }

    pub fn has_NNAMFMIPNKN(&self) -> bool {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NNAMFMIPNKN(&mut self, v: super::HCOHPHFOHAE::HCOHPHFOHAE) {
        self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NNAMFMIPNKN(&mut self) -> &mut super::HCOHPHFOHAE::HCOHPHFOHAE {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(_)) = self.PGOMJOKHFCN {
        } else {
            self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(super::HCOHPHFOHAE::HCOHPHFOHAE::new()));
        }
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NNAMFMIPNKN(&mut self) -> super::HCOHPHFOHAE::HCOHPHFOHAE {
        if self.has_NNAMFMIPNKN() {
            match self.PGOMJOKHFCN.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HCOHPHFOHAE::HCOHPHFOHAE::new()
        }
    }

    // .NGBCAJHPIGK DHMJFKPPOGI = 337;

    pub fn DHMJFKPPOGI(&self) -> &super::NGBCAJHPIGK::NGBCAJHPIGK {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(ref v)) => v,
            _ => <super::NGBCAJHPIGK::NGBCAJHPIGK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DHMJFKPPOGI(&mut self) {
        self.PGOMJOKHFCN = ::std::option::Option::None;
    }

    pub fn has_DHMJFKPPOGI(&self) -> bool {
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DHMJFKPPOGI(&mut self, v: super::NGBCAJHPIGK::NGBCAJHPIGK) {
        self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DHMJFKPPOGI(&mut self) -> &mut super::NGBCAJHPIGK::NGBCAJHPIGK {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(_)) = self.PGOMJOKHFCN {
        } else {
            self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(super::NGBCAJHPIGK::NGBCAJHPIGK::new()));
        }
        match self.PGOMJOKHFCN {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DHMJFKPPOGI(&mut self) -> super::NGBCAJHPIGK::NGBCAJHPIGK {
        if self.has_DHMJFKPPOGI() {
            match self.PGOMJOKHFCN.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NGBCAJHPIGK::NGBCAJHPIGK::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHOPJNKMNPB",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.FHOPJNKMNPB },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.FHOPJNKMNPB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KLDKOBLFAAC",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.KLDKOBLFAAC },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.KLDKOBLFAAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.retcode },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LDPBKMPNJFG::LDPBKMPNJFG>(
            "CBIOEMAOPNC",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.CBIOEMAOPNC },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.CBIOEMAOPNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FPFAFNFPJDK::FPFAFNFPJDK>(
            "PPPBLDKJHBG",
            TrainPartyHandlePendingActionScRsp::has_PPPBLDKJHBG,
            TrainPartyHandlePendingActionScRsp::PPPBLDKJHBG,
            TrainPartyHandlePendingActionScRsp::mut_PPPBLDKJHBG,
            TrainPartyHandlePendingActionScRsp::set_PPPBLDKJHBG,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::AKFEELJJOCI::AKFEELJJOCI>(
            "MPGFEGDIDDC",
            TrainPartyHandlePendingActionScRsp::has_MPGFEGDIDDC,
            TrainPartyHandlePendingActionScRsp::MPGFEGDIDDC,
            TrainPartyHandlePendingActionScRsp::mut_MPGFEGDIDDC,
            TrainPartyHandlePendingActionScRsp::set_MPGFEGDIDDC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HCOHPHFOHAE::HCOHPHFOHAE>(
            "NNAMFMIPNKN",
            TrainPartyHandlePendingActionScRsp::has_NNAMFMIPNKN,
            TrainPartyHandlePendingActionScRsp::NNAMFMIPNKN,
            TrainPartyHandlePendingActionScRsp::mut_NNAMFMIPNKN,
            TrainPartyHandlePendingActionScRsp::set_NNAMFMIPNKN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NGBCAJHPIGK::NGBCAJHPIGK>(
            "DHMJFKPPOGI",
            TrainPartyHandlePendingActionScRsp::has_DHMJFKPPOGI,
            TrainPartyHandlePendingActionScRsp::DHMJFKPPOGI,
            TrainPartyHandlePendingActionScRsp::mut_DHMJFKPPOGI,
            TrainPartyHandlePendingActionScRsp::set_DHMJFKPPOGI,
        ));
        oneofs.push(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrainPartyHandlePendingActionScRsp>(
            "TrainPartyHandlePendingActionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrainPartyHandlePendingActionScRsp {
    const NAME: &'static str = "TrainPartyHandlePendingActionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.FHOPJNKMNPB = is.read_uint32()?;
                },
                48 => {
                    self.KLDKOBLFAAC = is.read_bool()?;
                },
                88 => {
                    self.retcode = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CBIOEMAOPNC)?;
                },
                1266 => {
                    self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(is.read_message()?));
                },
                9890 => {
                    self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(is.read_message()?));
                },
                15842 => {
                    self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(is.read_message()?));
                },
                2698 => {
                    self.PGOMJOKHFCN = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(is.read_message()?));
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
        if self.FHOPJNKMNPB != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.FHOPJNKMNPB);
        }
        if self.KLDKOBLFAAC != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.retcode);
        }
        if let Some(v) = self.CBIOEMAOPNC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.PGOMJOKHFCN {
            match v {
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(ref v) => {
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
        if self.FHOPJNKMNPB != 0 {
            os.write_uint32(3, self.FHOPJNKMNPB)?;
        }
        if self.KLDKOBLFAAC != false {
            os.write_bool(6, self.KLDKOBLFAAC)?;
        }
        if self.retcode != 0 {
            os.write_uint32(11, self.retcode)?;
        }
        if let Some(v) = self.CBIOEMAOPNC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.PGOMJOKHFCN {
            match v {
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::PPPBLDKJHBG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(158, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::MPGFEGDIDDC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1236, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::NNAMFMIPNKN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1980, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::PGOMJOKHFCN::DHMJFKPPOGI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(337, v, os)?;
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

    fn new() -> TrainPartyHandlePendingActionScRsp {
        TrainPartyHandlePendingActionScRsp::new()
    }

    fn clear(&mut self) {
        self.FHOPJNKMNPB = 0;
        self.KLDKOBLFAAC = false;
        self.retcode = 0;
        self.CBIOEMAOPNC.clear();
        self.PGOMJOKHFCN = ::std::option::Option::None;
        self.PGOMJOKHFCN = ::std::option::Option::None;
        self.PGOMJOKHFCN = ::std::option::Option::None;
        self.PGOMJOKHFCN = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrainPartyHandlePendingActionScRsp {
        static instance: TrainPartyHandlePendingActionScRsp = TrainPartyHandlePendingActionScRsp {
            FHOPJNKMNPB: 0,
            KLDKOBLFAAC: false,
            retcode: 0,
            CBIOEMAOPNC: ::protobuf::MessageField::none(),
            PGOMJOKHFCN: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrainPartyHandlePendingActionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrainPartyHandlePendingActionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrainPartyHandlePendingActionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainPartyHandlePendingActionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `TrainPartyHandlePendingActionScRsp`
pub mod train_party_handle_pending_action_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:TrainPartyHandlePendingActionScRsp.PGOMJOKHFCN)
    pub enum PGOMJOKHFCN {
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.PPPBLDKJHBG)
        PPPBLDKJHBG(super::super::FPFAFNFPJDK::FPFAFNFPJDK),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.MPGFEGDIDDC)
        MPGFEGDIDDC(super::super::AKFEELJJOCI::AKFEELJJOCI),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.NNAMFMIPNKN)
        NNAMFMIPNKN(super::super::HCOHPHFOHAE::HCOHPHFOHAE),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.DHMJFKPPOGI)
        DHMJFKPPOGI(super::super::NGBCAJHPIGK::NGBCAJHPIGK),
    }

    impl ::protobuf::Oneof for PGOMJOKHFCN {
    }

    impl ::protobuf::OneofFull for PGOMJOKHFCN {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::TrainPartyHandlePendingActionScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("PGOMJOKHFCN").unwrap()).clone()
        }
    }

    impl PGOMJOKHFCN {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<PGOMJOKHFCN>("PGOMJOKHFCN")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(TrainPartyHandlePendingActionScRsp.proto\x1a\x11AKFEELJJOCI.proto\x1a\
    \x11FPFAFNFPJDK.proto\x1a\x11HCOHPHFOHAE.proto\x1a\x11LDPBKMPNJFG.proto\
    \x1a\x11NGBCAJHPIGK.proto\"\x8d\x03\n\"TrainPartyHandlePendingActionScRs\
    p\x12\x20\n\x0bFHOPJNKMNPB\x18\x03\x20\x01(\rR\x0bFHOPJNKMNPB\x12\x20\n\
    \x0bKLDKOBLFAAC\x18\x06\x20\x01(\x08R\x0bKLDKOBLFAAC\x12\x18\n\x07retcod\
    e\x18\x0b\x20\x01(\rR\x07retcode\x12.\n\x0bCBIOEMAOPNC\x18\x0f\x20\x01(\
    \x0b2\x0c.LDPBKMPNJFGR\x0bCBIOEMAOPNC\x121\n\x0bPPPBLDKJHBG\x18\x9e\x01\
    \x20\x01(\x0b2\x0c.FPFAFNFPJDKH\0R\x0bPPPBLDKJHBG\x121\n\x0bMPGFEGDIDDC\
    \x18\xd4\t\x20\x01(\x0b2\x0c.AKFEELJJOCIH\0R\x0bMPGFEGDIDDC\x121\n\x0bNN\
    AMFMIPNKN\x18\xbc\x0f\x20\x01(\x0b2\x0c.HCOHPHFOHAEH\0R\x0bNNAMFMIPNKN\
    \x121\n\x0bDHMJFKPPOGI\x18\xd1\x02\x20\x01(\x0b2\x0c.NGBCAJHPIGKH\0R\x0b\
    DHMJFKPPOGIB\r\n\x0bPGOMJOKHFCNb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::AKFEELJJOCI::file_descriptor().clone());
            deps.push(super::FPFAFNFPJDK::file_descriptor().clone());
            deps.push(super::HCOHPHFOHAE::file_descriptor().clone());
            deps.push(super::LDPBKMPNJFG::file_descriptor().clone());
            deps.push(super::NGBCAJHPIGK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrainPartyHandlePendingActionScRsp::generated_message_descriptor_data());
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
