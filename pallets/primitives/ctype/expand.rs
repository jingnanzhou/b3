#![feature(prelude_import)]
//! # CType Pallet
//!
//! A simple pallet which enables users to store their CType hash (blake2b as
//! hex string) on chain and associate it with their account id.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ### Terminology
//!
//! - **CType:**: CTypes are claim types. In everyday language, they are
//!   standardised structures for credentials. For example, a company may need a
//!   standard identification credential to identify workers that includes their
//!   full name, date of birth, access level and id number. Each of these are
//!   referred to as an attribute of a credential.
//!
//! ## Assumptions
//!
//! - The CType hash was created using our KILT JS-SDK.
//! - The underlying CType includes only the following required fields for the
//!   JSON-Schema we use in the SDK: Identifier, KILT specific JSON-Schema,
//!   Title and Properties.
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod ctype_entry {
    use frame_support::RuntimeDebug;
    use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
    use scale_info::TypeInfo;
    /// Creation details of a CType.
    pub struct CtypeEntry<Creator, BlockNumber> {
        /// Identifier of the creator.
        pub creator: Creator,
        /// Block number in which the creation tx was dispatched.
        pub created_at: BlockNumber,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<Creator, BlockNumber> ::parity_scale_codec::Encode
        for CtypeEntry<Creator, BlockNumber>
        where
            Creator: ::parity_scale_codec::Encode,
            Creator: ::parity_scale_codec::Encode,
            BlockNumber: ::parity_scale_codec::Encode,
            BlockNumber: ::parity_scale_codec::Encode,
        {
            fn encode_to<
                __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
            >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                ::parity_scale_codec::Encode::encode_to(
                    &self.creator,
                    __codec_dest_edqy,
                );
                ::parity_scale_codec::Encode::encode_to(
                    &self.created_at,
                    __codec_dest_edqy,
                );
            }
        }
        #[automatically_derived]
        impl<Creator, BlockNumber> ::parity_scale_codec::EncodeLike
        for CtypeEntry<Creator, BlockNumber>
        where
            Creator: ::parity_scale_codec::Encode,
            Creator: ::parity_scale_codec::Encode,
            BlockNumber: ::parity_scale_codec::Encode,
            BlockNumber: ::parity_scale_codec::Encode,
        {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<Creator, BlockNumber> ::parity_scale_codec::Decode
        for CtypeEntry<Creator, BlockNumber>
        where
            Creator: ::parity_scale_codec::Decode,
            Creator: ::parity_scale_codec::Decode,
            BlockNumber: ::parity_scale_codec::Decode,
            BlockNumber: ::parity_scale_codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                ::core::result::Result::Ok(CtypeEntry::<Creator, BlockNumber> {
                    creator: {
                        let __codec_res_edqy = <Creator as ::parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `CtypeEntry::creator`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                    created_at: {
                        let __codec_res_edqy = <BlockNumber as ::parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `CtypeEntry::created_at`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    impl<Creator, BlockNumber> core::fmt::Debug for CtypeEntry<Creator, BlockNumber>
    where
        Creator: core::fmt::Debug,
        BlockNumber: core::fmt::Debug,
    {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("CtypeEntry")
                .field("creator", &self.creator)
                .field("created_at", &self.created_at)
                .finish()
        }
    }
    const _: () = {
        impl<Creator, BlockNumber> ::parity_scale_codec::MaxEncodedLen
        for CtypeEntry<Creator, BlockNumber>
        where
            Creator: ::parity_scale_codec::MaxEncodedLen,
            Creator: ::parity_scale_codec::MaxEncodedLen,
            BlockNumber: ::parity_scale_codec::MaxEncodedLen,
            BlockNumber: ::parity_scale_codec::MaxEncodedLen,
        {
            fn max_encoded_len() -> ::core::primitive::usize {
                0_usize
                    .saturating_add(<Creator>::max_encoded_len())
                    .saturating_add(<BlockNumber>::max_encoded_len())
            }
        }
    };
    #[automatically_derived]
    impl<Creator, BlockNumber> ::core::marker::StructuralEq
    for CtypeEntry<Creator, BlockNumber> {}
    #[automatically_derived]
    impl<Creator: ::core::cmp::Eq, BlockNumber: ::core::cmp::Eq> ::core::cmp::Eq
    for CtypeEntry<Creator, BlockNumber> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Creator>;
            let _: ::core::cmp::AssertParamIsEq<BlockNumber>;
        }
    }
    #[automatically_derived]
    impl<Creator, BlockNumber> ::core::marker::StructuralPartialEq
    for CtypeEntry<Creator, BlockNumber> {}
    #[automatically_derived]
    impl<
        Creator: ::core::cmp::PartialEq,
        BlockNumber: ::core::cmp::PartialEq,
    > ::core::cmp::PartialEq for CtypeEntry<Creator, BlockNumber> {
        #[inline]
        fn eq(&self, other: &CtypeEntry<Creator, BlockNumber>) -> bool {
            self.creator == other.creator && self.created_at == other.created_at
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<Creator, BlockNumber> ::scale_info::TypeInfo
        for CtypeEntry<Creator, BlockNumber>
        where
            Creator: ::scale_info::TypeInfo + 'static,
            BlockNumber: ::scale_info::TypeInfo + 'static,
            Creator: ::scale_info::TypeInfo + 'static,
            BlockNumber: ::scale_info::TypeInfo + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("CtypeEntry", "ctype::ctype_entry"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "Creator",
                                    ::core::option::Option::Some(
                                        ::scale_info::meta_type::<Creator>(),
                                    ),
                                ),
                                ::scale_info::TypeParameter::new(
                                    "BlockNumber",
                                    ::core::option::Option::Some(
                                        ::scale_info::meta_type::<BlockNumber>(),
                                    ),
                                ),
                            ]),
                        ),
                    )
                    .docs(&["Creation details of a CType."])
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<Creator>()
                                    .name("creator")
                                    .type_name("Creator")
                                    .docs(&["Identifier of the creator."])
                            })
                            .field(|f| {
                                f
                                    .ty::<BlockNumber>()
                                    .name("created_at")
                                    .type_name("BlockNumber")
                                    .docs(
                                        &["Block number in which the creation tx was dispatched."],
                                    )
                            }),
                    )
            }
        }
    };
}
pub mod default_weights {
    //! Autogenerated weights for ctype
    //!
    //! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
    //! DATE: 2023-05-18
    //! STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
    //! WORST CASE MAP SIZE: `1000000`
    //! HOSTNAME: `rust-2`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
    //! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024
    #![allow(unused_parens)]
    #![allow(unused_imports)]
    use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
    use sp_std::marker::PhantomData;
    /// Weight functions needed for ctype.
    pub trait WeightInfo {
        fn add(l: u32) -> Weight;
        fn set_block_number() -> Weight;
    }
    /// Weights for ctype using the Substrate node and recommended hardware.
    pub struct SubstrateWeight<T>(PhantomData<T>);
    impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
        /// Storage: System Account (r:2 w:2)
        /// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
        /// Storage: Ctype Ctypes (r:1 w:1)
        /// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
        /// The range of component `l` is `[1, 5242880]`.
        fn add(l: u32) -> Weight {
            Weight::from_parts(48_883_000, 7777)
                .saturating_add(Weight::from_parts(1_195, 0).saturating_mul(l.into()))
                .saturating_add(T::DbWeight::get().reads(3_u64))
                .saturating_add(T::DbWeight::get().writes(3_u64))
        }
        /// Storage: Ctype Ctypes (r:1 w:1)
        /// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
        fn set_block_number() -> Weight {
            Weight::from_parts(13_231_000, 2563)
                .saturating_add(T::DbWeight::get().reads(1_u64))
                .saturating_add(T::DbWeight::get().writes(1_u64))
        }
    }
    impl WeightInfo for () {
        /// Storage: System Account (r:2 w:2)
        /// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
        /// Storage: Ctype Ctypes (r:1 w:1)
        /// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
        /// The range of component `l` is `[1, 5242880]`.
        fn add(l: u32) -> Weight {
            Weight::from_parts(48_883_000, 7777)
                .saturating_add(Weight::from_parts(1_195, 0).saturating_mul(l.into()))
                .saturating_add(RocksDbWeight::get().reads(3_u64))
                .saturating_add(RocksDbWeight::get().writes(3_u64))
        }
        /// Storage: Ctype Ctypes (r:1 w:1)
        /// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
        fn set_block_number() -> Weight {
            Weight::from_parts(13_231_000, 2563)
                .saturating_add(RocksDbWeight::get().reads(1_u64))
                .saturating_add(RocksDbWeight::get().writes(1_u64))
        }
    }
}
pub use crate::{default_weights::WeightInfo, pallet::*};
/**
			The module that hosts all the
			[FRAME](https://docs.substrate.io/main-docs/build/events-errors/)
			types needed to add this pallet to a
			runtime.
			*/
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*, sp_runtime::traits::Hash,
        traits::{
            Currency, ExistenceRequirement, OnUnbalanced, StorageVersion, WithdrawReasons,
        },
    };
    use frame_system::pallet_prelude::*;
    use kilt_support::traits::CallSources;
    use sp_runtime::{traits::Saturating, SaturatedConversion};
    use sp_std::vec::Vec;
    use crate::ctype_entry::CtypeEntry;
    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);
    /// Type of a CType hash.
    pub type CtypeHashOf<T> = <T as frame_system::Config>::Hash;
    pub type CtypeEntryOf<T> = CtypeEntry<
        <T as Config>::CtypeCreatorId,
        BlockNumberFor<T>,
    >;
    /// Type of a CType creator.
    pub type CtypeCreatorOf<T> = <T as Config>::CtypeCreatorId;
    pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<
        AccountIdOf<T>,
    >>::Balance;
    type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        AccountIdOf<T>,
    >>::NegativeImbalance;
    /**
			Configuration trait of this pallet.

			Implement this type for a runtime in order to customize this pallet.
			*/
    pub trait Config: frame_system::Config {
        type EnsureOrigin: EnsureOrigin<
                <Self as frame_system::Config>::RuntimeOrigin,
                Success = Self::OriginSuccess,
            >;
        type OverarchingOrigin: EnsureOrigin<
                <Self as frame_system::Config>::RuntimeOrigin,
            >;
        type OriginSuccess: CallSources<AccountIdOf<Self>, CtypeCreatorOf<Self>>;
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<AccountIdOf<Self>>;
        type WeightInfo: WeightInfo;
        type CtypeCreatorId: Parameter + MaxEncodedLen;
        type Fee: Get<BalanceOf<Self>>;
        type FeeCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;
    }
    /**
			The [pallet](https://docs.substrate.io/reference/frame-pallets/#pallets) implementing
			the on-chain logic.
			*/
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    /// CTypes stored on chain.
    ///
    /// It maps from a CType hash to its creator and block number in which it
    /// was created.
    #[allow(type_alias_bounds)]
    pub type Ctypes<T> = StorageMap<
        _GeneratedPrefixForStorageCtypes<T>,
        Blake2_128Concat,
        CtypeHashOf<T>,
        CtypeEntryOf<T>,
    >;
    /**
			The [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted
			by this pallet.
			*/
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        /// A new CType has been created.
        /// \[creator identifier, CType hash\]
        CTypeCreated(CtypeCreatorOf<T>, CtypeHashOf<T>),
        /// Information about a CType has been updated.
        /// \[CType hash\]
        CTypeUpdated(CtypeHashOf<T>),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>, frame_support::Never),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::CTypeCreated(ref _0, ref _1) => {
                        Self::CTypeCreated(
                            core::clone::Clone::clone(_0),
                            core::clone::Clone::clone(_1),
                        )
                    }
                    Self::CTypeUpdated(ref _0) => {
                        Self::CTypeUpdated(core::clone::Clone::clone(_0))
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            core::clone::Clone::clone(_0),
                            core::clone::Clone::clone(_1),
                        )
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (
                        Self::CTypeCreated(_0, _1),
                        Self::CTypeCreated(_0_other, _1_other),
                    ) => true && _0 == _0_other && _1 == _1_other,
                    (Self::CTypeUpdated(_0), Self::CTypeUpdated(_0_other)) => {
                        true && _0 == _0_other
                    }
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::CTypeCreated { .. }, Self::CTypeUpdated { .. }) => false,
                    (Self::CTypeCreated { .. }, Self::__Ignore { .. }) => false,
                    (Self::CTypeUpdated { .. }, Self::CTypeCreated { .. }) => false,
                    (Self::CTypeUpdated { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::CTypeCreated { .. }) => false,
                    (Self::__Ignore { .. }, Self::CTypeUpdated { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::CTypeCreated(ref _0, ref _1) => {
                        fmt
                            .debug_tuple("Event::CTypeCreated")
                            .field(&_0)
                            .field(&_1)
                            .finish()
                    }
                    Self::CTypeUpdated(ref _0) => {
                        fmt.debug_tuple("Event::CTypeUpdated").field(&_0).finish()
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Event::__Ignore").field(&_0).field(&_1).finish()
                    }
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::parity_scale_codec::Encode for Event<T>
        where
            CtypeCreatorOf<T>: ::parity_scale_codec::Encode,
            CtypeCreatorOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
        {
            fn encode_to<
                __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
            >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                match *self {
                    Event::CTypeCreated(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::CTypeUpdated(ref aa) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::parity_scale_codec::EncodeLike for Event<T>
        where
            CtypeCreatorOf<T>: ::parity_scale_codec::Encode,
            CtypeCreatorOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
            CtypeHashOf<T>: ::parity_scale_codec::Encode,
        {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::parity_scale_codec::Decode for Event<T>
        where
            CtypeCreatorOf<T>: ::parity_scale_codec::Decode,
            CtypeCreatorOf<T>: ::parity_scale_codec::Decode,
            CtypeHashOf<T>: ::parity_scale_codec::Decode,
            CtypeHashOf<T>: ::parity_scale_codec::Decode,
            CtypeHashOf<T>: ::parity_scale_codec::Decode,
            CtypeHashOf<T>: ::parity_scale_codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Event`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Event::<
                                T,
                            >::CTypeCreated(
                                {
                                    let __codec_res_edqy = <CtypeCreatorOf<
                                        T,
                                    > as ::parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::CTypeCreated.0`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                                {
                                    let __codec_res_edqy = <CtypeHashOf<
                                        T,
                                    > as ::parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::CTypeCreated.1`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                            ),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy
                        == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Event::<
                                T,
                            >::CTypeUpdated({
                                let __codec_res_edqy = <CtypeHashOf<
                                    T,
                                > as ::parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::CTypeUpdated.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Event`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Event<T>
        where
            CtypeCreatorOf<T>: ::scale_info::TypeInfo + 'static,
            CtypeHashOf<T>: ::scale_info::TypeInfo + 'static,
            CtypeHashOf<T>: ::scale_info::TypeInfo + 'static,
            frame_support::sp_std::marker::PhantomData<
                (T),
            >: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Event", "ctype::pallet"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "CTypeCreated",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f.ty::<CtypeCreatorOf<T>>().type_name("CtypeCreatorOf<T>")
                                                })
                                                .field(|f| {
                                                    f.ty::<CtypeHashOf<T>>().type_name("CtypeHashOf<T>")
                                                }),
                                        )
                                        .docs_always(
                                            &[
                                                "A new CType has been created.",
                                                "\\[creator identifier, CType hash\\]",
                                            ],
                                        )
                                },
                            )
                            .variant(
                                "CTypeUpdated",
                                |v| {
                                    v
                                        .index(1usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f.ty::<CtypeHashOf<T>>().type_name("CtypeHashOf<T>")
                                                }),
                                        )
                                        .docs_always(
                                            &[
                                                "Information about a CType has been updated.",
                                                "\\[CType hash\\]",
                                            ],
                                        )
                                },
                            ),
                    )
            }
        }
    };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    /**
			Custom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)
			of this pallet.
			*/
    pub enum Error<T> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>, frame_support::Never),
        /// There is no CType with the given hash.
        NotFound,
        /// The CType already exists.
        AlreadyExists,
        /// The paying account was unable to pay the fees for creating a ctype.
        UnableToPayFees,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::parity_scale_codec::Encode for Error<T> {
            fn encode_to<
                __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
            >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                match *self {
                    Error::NotFound => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    Error::AlreadyExists => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    }
                    Error::UnableToPayFees => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T> ::parity_scale_codec::EncodeLike for Error<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::parity_scale_codec::Decode for Error<T> {
            fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Error`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::NotFound)
                    }
                    __codec_x_edqy if __codec_x_edqy
                        == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::AlreadyExists)
                    }
                    __codec_x_edqy if __codec_x_edqy
                        == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::UnableToPayFees)
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Error`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T> ::scale_info::TypeInfo for Error<T>
        where
            frame_support::sp_std::marker::PhantomData<
                (T),
            >: ::scale_info::TypeInfo + 'static,
            T: 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Error", "ctype::pallet"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "NotFound",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .docs_always(&["There is no CType with the given hash."])
                                },
                            )
                            .variant(
                                "AlreadyExists",
                                |v| {
                                    v
                                        .index(1usize as ::core::primitive::u8)
                                        .docs_always(&["The CType already exists."])
                                },
                            )
                            .variant(
                                "UnableToPayFees",
                                |v| {
                                    v
                                        .index(2usize as ::core::primitive::u8)
                                        .docs_always(
                                            &[
                                                "The paying account was unable to pay the fees for creating a ctype.",
                                            ],
                                        )
                                },
                            ),
                    )
            }
        }
    };
    const _: () = {
        impl<T> frame_support::traits::PalletError for Error<T> {
            const MAX_ENCODED_SIZE: usize = 1;
        }
    };
    impl<T: Config> Pallet<T> {
        /// Create a new CType from the given unique CType hash and associates
        /// it with its creator.
        ///
        /// A CType with the same hash must not be stored on chain.
        ///
        /// Emits `CTypeCreated`.
        ///
        /// # <weight>
        /// Weight: O(1)
        /// - Reads: Ctypes, Balance
        /// - Writes: Ctypes, Balance
        /// # </weight>
        pub fn add(origin: OriginFor<T>, ctype: Vec<u8>) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let source = <T as Config>::EnsureOrigin::ensure_origin(origin)?;
                let creator = source.subject();
                let payer = source.sender();
                let balance = <T::Currency as Currency<
                    AccountIdOf<T>,
                >>::free_balance(&payer);
                <T::Currency as Currency<
                    AccountIdOf<T>,
                >>::ensure_can_withdraw(
                    &payer,
                    T::Fee::get(),
                    WithdrawReasons::FEE,
                    balance.saturating_sub(T::Fee::get()),
                )?;
                let hash = <T as frame_system::Config>::Hashing::hash(&ctype[..]);
                {
                    if !!Ctypes::<T>::contains_key(hash) {
                        { return Err(Error::<T>::AlreadyExists.into()) };
                    }
                };
                let imbalance = <T::Currency as Currency<
                    AccountIdOf<T>,
                >>::withdraw(
                        &payer,
                        T::Fee::get(),
                        WithdrawReasons::FEE,
                        ExistenceRequirement::AllowDeath,
                    )
                    .map_err(|_| Error::<T>::UnableToPayFees)?;
                T::FeeCollector::on_unbalanced(imbalance);
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["Creating CType with hash ", " and creator "],
                                &[
                                    ::core::fmt::ArgumentV1::new_debug(&hash),
                                    ::core::fmt::ArgumentV1::new_debug(&creator),
                                ],
                            ),
                            lvl,
                            &(
                                "ctype::pallet",
                                "ctype::pallet",
                                "pallets/primitives/ctype/src/lib.rs",
                                186u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                Ctypes::<
                    T,
                >::insert(
                    hash,
                    CtypeEntryOf::<T> {
                        creator: creator.clone(),
                        created_at: frame_system::Pallet::<T>::block_number(),
                    },
                );
                Self::deposit_event(Event::CTypeCreated(creator, hash));
                Ok(())
            })
        }
        /// Set the creation block number for a given CType, if found.
        ///
        /// Emits `CTypeUpdated`.
        pub fn set_block_number(
            origin: OriginFor<T>,
            ctype_hash: CtypeHashOf<T>,
            block_number: BlockNumberFor<T>,
        ) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                T::OverarchingOrigin::ensure_origin(origin)?;
                Ctypes::<
                    T,
                >::try_mutate(
                    ctype_hash,
                    |ctype_entry| {
                        if let Some(ctype_entry) = ctype_entry {
                            ctype_entry.created_at = block_number;
                            Ok(())
                        } else {
                            Err(Error::<T>::NotFound)
                        }
                    },
                )?;
                Self::deposit_event(Event::CTypeUpdated(ctype_hash));
                Ok(())
            })
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata() -> frame_support::sp_std::vec::Vec<
            frame_support::metadata::PalletConstantMetadata,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<frame_support::metadata::PalletErrorMetadata> {
            Some(frame_support::metadata::PalletErrorMetadata {
                ty: frame_support::scale_info::meta_type::<Error<T>>(),
            })
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            STORAGE_VERSION
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = STORAGE_VERSION;
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn module_name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::module_name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 9u8,
                patch: 37u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn infos() -> frame_support::sp_std::vec::Vec<
            frame_support::traits::PalletInfoData,
        > {
            use frame_support::traits::PalletInfoAccess;
            let item = frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([item]))
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<
            frame_support::traits::StorageInfo,
        > {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info = <Ctypes<
                    T,
                > as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    use frame_support::traits::{
        StorageInfoTrait, TrackedStorageKey, WhitelistedStorageKeys,
    };
    impl<T: Config> WhitelistedStorageKeys for Pallet<T> {
        fn whitelisted_storage_keys() -> frame_support::sp_std::vec::Vec<
            TrackedStorageKey,
        > {
            use frame_support::sp_std::vec;
            ::alloc::vec::Vec::new()
        }
    }
    mod warnings {}
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T,)>, frame_support::Never),
        /// Create a new CType from the given unique CType hash and associates
        /// it with its creator.
        ///
        /// A CType with the same hash must not be stored on chain.
        ///
        /// Emits `CTypeCreated`.
        ///
        /// # <weight>
        /// Weight: O(1)
        /// - Reads: Ctypes, Balance
        /// - Writes: Ctypes, Balance
        /// # </weight>
        #[codec(index = 0u8)]
        add { #[allow(missing_docs)] ctype: Vec<u8> },
        /// Set the creation block number for a given CType, if found.
        ///
        /// Emits `CTypeUpdated`.
        #[codec(index = 1u8)]
        set_block_number {
            #[allow(missing_docs)]
            ctype_hash: CtypeHashOf<T>,
            #[allow(missing_docs)]
            block_number: BlockNumberFor<T>,
        },
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Call::__Ignore").field(&_0).field(&_1).finish()
                    }
                    Self::add { ref ctype } => {
                        fmt.debug_struct("Call::add").field("ctype", &ctype).finish()
                    }
                    Self::set_block_number { ref ctype_hash, ref block_number } => {
                        fmt
                            .debug_struct("Call::set_block_number")
                            .field("ctype_hash", &ctype_hash)
                            .field("block_number", &block_number)
                            .finish()
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            core::clone::Clone::clone(_0),
                            core::clone::Clone::clone(_1),
                        )
                    }
                    Self::add { ref ctype } => {
                        Self::add {
                            ctype: core::clone::Clone::clone(ctype),
                        }
                    }
                    Self::set_block_number { ref ctype_hash, ref block_number } => {
                        Self::set_block_number {
                            ctype_hash: core::clone::Clone::clone(ctype_hash),
                            block_number: core::clone::Clone::clone(block_number),
                        }
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::add { ctype }, Self::add { ctype: _0 }) => true && ctype == _0,
                    (
                        Self::set_block_number { ctype_hash, block_number },
                        Self::set_block_number { ctype_hash: _0, block_number: _1 },
                    ) => true && ctype_hash == _0 && block_number == _1,
                    (Self::__Ignore { .. }, Self::add { .. }) => false,
                    (Self::__Ignore { .. }, Self::set_block_number { .. }) => false,
                    (Self::add { .. }, Self::__Ignore { .. }) => false,
                    (Self::add { .. }, Self::set_block_number { .. }) => false,
                    (Self::set_block_number { .. }, Self::__Ignore { .. }) => false,
                    (Self::set_block_number { .. }, Self::add { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::parity_scale_codec::Encode for Call<T> {
            fn encode_to<
                __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
            >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                match *self {
                    Call::add { ref ctype } => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::parity_scale_codec::Encode::encode_to(
                            ctype,
                            __codec_dest_edqy,
                        );
                    }
                    Call::set_block_number { ref ctype_hash, ref block_number } => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::parity_scale_codec::Encode::encode_to(
                            ctype_hash,
                            __codec_dest_edqy,
                        );
                        ::parity_scale_codec::Encode::encode_to(
                            block_number,
                            __codec_dest_edqy,
                        );
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::parity_scale_codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::parity_scale_codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Call`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::add {
                            ctype: {
                                let __codec_res_edqy = <Vec<
                                    u8,
                                > as ::parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::add::ctype`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::set_block_number {
                            ctype_hash: {
                                let __codec_res_edqy = <CtypeHashOf<
                                    T,
                                > as ::parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `Call::set_block_number::ctype_hash`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            block_number: {
                                let __codec_res_edqy = <BlockNumberFor<
                                    T,
                                > as ::parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `Call::set_block_number::block_number`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Call`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            frame_support::sp_std::marker::PhantomData<
                (T,),
            >: ::scale_info::TypeInfo + 'static,
            CtypeHashOf<T>: ::scale_info::TypeInfo + 'static,
            BlockNumberFor<T>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Call", "ctype::pallet"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "Contains one variant per dispatchable that can be called by an extrinsic.",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "add",
                                |v| {
                                    v
                                        .index(0u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| {
                                                    f.ty::<Vec<u8>>().name("ctype").type_name("Vec<u8>")
                                                }),
                                        )
                                        .docs_always(
                                            &[
                                                "Create a new CType from the given unique CType hash and associates",
                                                "it with its creator.",
                                                "",
                                                "A CType with the same hash must not be stored on chain.",
                                                "",
                                                "Emits `CTypeCreated`.",
                                                "",
                                                "# <weight>",
                                                "Weight: O(1)",
                                                "- Reads: Ctypes, Balance",
                                                "- Writes: Ctypes, Balance",
                                                "# </weight>",
                                            ],
                                        )
                                },
                            )
                            .variant(
                                "set_block_number",
                                |v| {
                                    v
                                        .index(1u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| {
                                                    f
                                                        .ty::<CtypeHashOf<T>>()
                                                        .name("ctype_hash")
                                                        .type_name("CtypeHashOf<T>")
                                                })
                                                .field(|f| {
                                                    f
                                                        .ty::<BlockNumberFor<T>>()
                                                        .name("block_number")
                                                        .type_name("BlockNumberFor<T>")
                                                }),
                                        )
                                        .docs_always(
                                            &[
                                                "Set the creation block number for a given CType, if found.",
                                                "",
                                                "Emits `CTypeUpdated`.",
                                            ],
                                        )
                                },
                            ),
                    )
            }
        }
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `add`.
        pub fn new_call_variant_add(ctype: Vec<u8>) -> Self {
            Self::add { ctype }
        }
        ///Create a call with the variant `set_block_number`.
        pub fn new_call_variant_set_block_number(
            ctype_hash: CtypeHashOf<T>,
            block_number: BlockNumberFor<T>,
        ) -> Self {
            Self::set_block_number {
                ctype_hash,
                block_number,
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::add { ref ctype } => {
                    let __pallet_base_weight = <T as pallet::Config>::WeightInfo::add(
                        ctype.len().saturated_into(),
                    );
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<
                        (&Vec<u8>,),
                    >>::weigh_data(&__pallet_base_weight, (ctype,));
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<
                        (&Vec<u8>,),
                    >>::classify_dispatch(&__pallet_base_weight, (ctype,));
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<
                        (&Vec<u8>,),
                    >>::pays_fee(&__pallet_base_weight, (ctype,));
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::set_block_number { ref ctype_hash, ref block_number } => {
                    let __pallet_base_weight = <T as pallet::Config>::WeightInfo::set_block_number();
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<
                        (&CtypeHashOf<T>, &BlockNumberFor<T>),
                    >>::weigh_data(&__pallet_base_weight, (ctype_hash, block_number));
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<
                        (&CtypeHashOf<T>, &BlockNumberFor<T>),
                    >>::classify_dispatch(
                        &__pallet_base_weight,
                        (ctype_hash, block_number),
                    );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<
                        (&CtypeHashOf<T>, &BlockNumberFor<T>),
                    >>::pays_fee(&__pallet_base_weight, (ctype_hash, block_number));
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["internal error: entered unreachable code: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &["__Ignore cannot be used"],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    )
                }
            }
        }
    }
    #[allow(deprecated)]
    impl<T: Config> frame_support::weights::GetDispatchInfo for Call<T> {}
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::add { .. } => "add",
                Self::set_block_number { .. } => "set_block_number",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["internal error: entered unreachable code: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &["__PhantomItem cannot be used."],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    )
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["add", "set_block_number"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type RuntimeOrigin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::RuntimeOrigin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::add { ctype } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "add",
                                    "ctype::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/primitives/ctype/src/lib.rs"),
                                    Some(60u32),
                                    Some("ctype::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = ::tracing::__macro_support::__disabled_span(
                                CALLSITE.metadata(),
                            );
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::add(origin, ctype).map(Into::into).map_err(Into::into)
                }
                Self::set_block_number { ctype_hash, block_number } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "set_block_number",
                                    "ctype::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/primitives/ctype/src/lib.rs"),
                                    Some(60u32),
                                    Some("ctype::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = ::tracing::__macro_support::__disabled_span(
                                CALLSITE.metadata(),
                            );
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::set_block_number(origin, ctype_hash, block_number)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["internal error: entered unreachable code: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &["__PhantomItem cannot be used."],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type RuntimeCall = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["internal error: entered unreachable code: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &["`__Ignore` can never be constructed"],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    )
                }
                Self::NotFound => "NotFound",
                Self::AlreadyExists => "AlreadyExists",
                Self::UnableToPayFees => "UnableToPayFees",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use frame_support::codec::Encode;
            let index = <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Pallet<T>,
            >()
                .expect("Every active module has an index in the runtime; qed") as u8;
            let mut encoded = err.encode();
            encoded.resize(frame_support::MAX_MODULE_ERROR_ENCODED_SIZE, 0);
            frame_support::sp_runtime::DispatchError::Module(frame_support::sp_runtime::ModuleError {
                index,
                error: TryInto::try_into(encoded)
                    .expect(
                        "encoded error is resized to be equal to the maximum encoded error size; qed",
                    ),
                message: Some(err.as_str()),
            })
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
            let event = <<T as Config>::RuntimeEvent as Into<
                <T as frame_system::Config>::RuntimeEvent,
            >>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
            frame_support::metadata::PalletStorageMetadata {
                prefix: <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                    Pallet<T>,
                >()
                    .expect(
                        "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                    ),
                entries: {
                    #[allow(unused_mut)]
                    let mut entries = ::alloc::vec::Vec::new();
                    {
                        <Ctypes<
                            T,
                        > as frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    " CTypes stored on chain.",
                                    "",
                                    " It maps from a CType hash to its creator and block number in which it",
                                    " was created.",
                                ]),
                            ),
                            &mut entries,
                        );
                    }
                    entries
                },
            }
        }
    }
    impl<T: Config> Pallet<T> {
        /// CTypes stored on chain.
        ///
        /// It maps from a CType hash to its creator and block number in which it
        /// was created.
        pub fn ctypes<KArg>(k: KArg) -> Option<CtypeEntryOf<T>>
        where
            KArg: frame_support::codec::EncodeLike<CtypeHashOf<T>>,
        {
            <Ctypes<
                T,
            > as frame_support::storage::StorageMap<
                CtypeHashOf<T>,
                CtypeEntryOf<T>,
            >>::get(k)
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageCtypes<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
    for _GeneratedPrefixForStorageCtypes<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
                .expect(
                    "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
        const STORAGE_PREFIX: &'static str = "Ctypes";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<
        T: Config,
    > frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "ctype::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/primitives/ctype/src/lib.rs"),
                            Some(60u32),
                            Some("ctype::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_finalize(n)
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_idle(n, remaining_weight)
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "ctype::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/primitives/ctype/src/lib.rs"),
                            Some(60u32),
                            Some("ctype::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_initialize(n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "ctype::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/primitives/ctype/src/lib.rs"),
                            Some(60u32),
                            Some("ctype::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
                .unwrap_or("<unknown pallet name>");
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["\u{2705} no migration for "],
                            &[::core::fmt::ArgumentV1::new_display(&pallet_name)],
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "ctype::pallet",
                            "pallets/primitives/ctype/src/lib.rs",
                            60u32,
                        ),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_runtime_upgrade()
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::offchain_worker(n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::integrity_test()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
}
