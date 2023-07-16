#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod deposit {
    use frame_support::traits::ReservableCurrency;
    use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
    use scale_info::TypeInfo;
    use sp_runtime::{traits::Zero, DispatchError};
    /// An amount of balance reserved by the specified address.
    pub struct Deposit<Account, Balance> {
        pub owner: Account,
        pub amount: Balance,
    }
    #[automatically_derived]
    impl<
        Account: ::core::clone::Clone,
        Balance: ::core::clone::Clone,
    > ::core::clone::Clone for Deposit<Account, Balance> {
        #[inline]
        fn clone(&self) -> Deposit<Account, Balance> {
            Deposit {
                owner: ::core::clone::Clone::clone(&self.owner),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[automatically_derived]
    impl<Account: ::core::fmt::Debug, Balance: ::core::fmt::Debug> ::core::fmt::Debug
    for Deposit<Account, Balance> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Deposit",
                "owner",
                &&self.owner,
                "amount",
                &&self.amount,
            )
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<Account, Balance> ::parity_scale_codec::Encode for Deposit<Account, Balance>
        where
            Account: ::parity_scale_codec::Encode,
            Account: ::parity_scale_codec::Encode,
            Balance: ::parity_scale_codec::Encode,
            Balance: ::parity_scale_codec::Encode,
        {
            fn encode_to<
                __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
            >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                ::parity_scale_codec::Encode::encode_to(&self.owner, __codec_dest_edqy);
                ::parity_scale_codec::Encode::encode_to(&self.amount, __codec_dest_edqy);
            }
        }
        #[automatically_derived]
        impl<Account, Balance> ::parity_scale_codec::EncodeLike
        for Deposit<Account, Balance>
        where
            Account: ::parity_scale_codec::Encode,
            Account: ::parity_scale_codec::Encode,
            Balance: ::parity_scale_codec::Encode,
            Balance: ::parity_scale_codec::Encode,
        {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<Account, Balance> ::parity_scale_codec::Decode for Deposit<Account, Balance>
        where
            Account: ::parity_scale_codec::Decode,
            Account: ::parity_scale_codec::Decode,
            Balance: ::parity_scale_codec::Decode,
            Balance: ::parity_scale_codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                ::core::result::Result::Ok(Deposit::<Account, Balance> {
                    owner: {
                        let __codec_res_edqy = <Account as ::parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Deposit::owner`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                    amount: {
                        let __codec_res_edqy = <Balance as ::parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Deposit::amount`"),
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
    #[automatically_derived]
    impl<Account, Balance> ::core::marker::StructuralEq for Deposit<Account, Balance> {}
    #[automatically_derived]
    impl<Account: ::core::cmp::Eq, Balance: ::core::cmp::Eq> ::core::cmp::Eq
    for Deposit<Account, Balance> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Account>;
            let _: ::core::cmp::AssertParamIsEq<Balance>;
        }
    }
    #[automatically_derived]
    impl<Account, Balance> ::core::marker::StructuralPartialEq
    for Deposit<Account, Balance> {}
    #[automatically_derived]
    impl<
        Account: ::core::cmp::PartialEq,
        Balance: ::core::cmp::PartialEq,
    > ::core::cmp::PartialEq for Deposit<Account, Balance> {
        #[inline]
        fn eq(&self, other: &Deposit<Account, Balance>) -> bool {
            self.owner == other.owner && self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl<Account: ::core::cmp::Ord, Balance: ::core::cmp::Ord> ::core::cmp::Ord
    for Deposit<Account, Balance> {
        #[inline]
        fn cmp(&self, other: &Deposit<Account, Balance>) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.owner, &other.owner) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.amount, &other.amount)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl<
        Account: ::core::cmp::PartialOrd,
        Balance: ::core::cmp::PartialOrd,
    > ::core::cmp::PartialOrd for Deposit<Account, Balance> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Deposit<Account, Balance>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.owner, &other.owner) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.amount, &other.amount)
                }
                cmp => cmp,
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<Account, Balance> ::scale_info::TypeInfo for Deposit<Account, Balance>
        where
            Account: ::scale_info::TypeInfo + 'static,
            Balance: ::scale_info::TypeInfo + 'static,
            Account: ::scale_info::TypeInfo + 'static,
            Balance: ::scale_info::TypeInfo + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Deposit", "kilt_support::deposit"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "Account",
                                    ::core::option::Option::Some(
                                        ::scale_info::meta_type::<Account>(),
                                    ),
                                ),
                                ::scale_info::TypeParameter::new(
                                    "Balance",
                                    ::core::option::Option::Some(
                                        ::scale_info::meta_type::<Balance>(),
                                    ),
                                ),
                            ]),
                        ),
                    )
                    .docs(&["An amount of balance reserved by the specified address."])
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f.ty::<Account>().name("owner").type_name("Account")
                            })
                            .field(|f| {
                                f.ty::<Balance>().name("amount").type_name("Balance")
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl<Account, Balance> ::parity_scale_codec::MaxEncodedLen
        for Deposit<Account, Balance>
        where
            Account: ::parity_scale_codec::MaxEncodedLen,
            Account: ::parity_scale_codec::MaxEncodedLen,
            Balance: ::parity_scale_codec::MaxEncodedLen,
            Balance: ::parity_scale_codec::MaxEncodedLen,
        {
            fn max_encoded_len() -> ::core::primitive::usize {
                0_usize
                    .saturating_add(<Account>::max_encoded_len())
                    .saturating_add(<Balance>::max_encoded_len())
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<Account, Balance> _serde::Serialize for Deposit<Account, Balance>
        where
            Account: _serde::Serialize,
            Balance: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Deposit",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "owner",
                    &self.owner,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "amount",
                    &self.amount,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, Account, Balance> _serde::Deserialize<'de>
        for Deposit<Account, Balance>
        where
            Account: _serde::Deserialize<'de>,
            Balance: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "owner" => _serde::__private::Ok(__Field::__field0),
                            "amount" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"owner" => _serde::__private::Ok(__Field::__field0),
                            b"amount" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de, Account, Balance>
                where
                    Account: _serde::Deserialize<'de>,
                    Balance: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<Deposit<Account, Balance>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, Account, Balance> _serde::de::Visitor<'de>
                for __Visitor<'de, Account, Balance>
                where
                    Account: _serde::Deserialize<'de>,
                    Balance: _serde::Deserialize<'de>,
                {
                    type Value = Deposit<Account, Balance>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Deposit",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Account,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Deposit with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Balance,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Deposit with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Deposit {
                            owner: __field0,
                            amount: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Account> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Balance> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("owner"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Account,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Balance,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("owner") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("amount") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Deposit {
                            owner: __field0,
                            amount: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["owner", "amount"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Deposit",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            Deposit<Account, Balance>,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub fn reserve_deposit<Account, Currency: ReservableCurrency<Account>>(
        account: Account,
        deposit_amount: Currency::Balance,
    ) -> Result<Deposit<Account, Currency::Balance>, DispatchError> {
        Currency::reserve(&account, deposit_amount)?;
        Ok(Deposit {
            owner: account,
            amount: deposit_amount,
        })
    }
    pub fn free_deposit<Account, Currency: ReservableCurrency<Account>>(
        deposit: &Deposit<Account, Currency::Balance>,
    ) {
        let err_amount = Currency::unreserve(&deposit.owner, deposit.amount);
        if true {
            if !err_amount.is_zero() {
                ::core::panicking::panic("assertion failed: err_amount.is_zero()")
            }
        }
    }
}
pub use deposit::{free_deposit, reserve_deposit};
#[cfg(any(feature = "runtime-benchmarks", feature = "mock"))]
pub mod mock {
    ///! This module contains utilities for testing.
    use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
    use scale_info::TypeInfo;
    use sp_core::sr25519;
    use sp_runtime::AccountId32;
    /// This pallet only contains an origin which supports separated sender and
    /// subject.
    ///
    /// WARNING: This is only used for testing!
    #[allow(dead_code)]
    pub mod mock_origin {
        use sp_std::marker::PhantomData;
        use frame_support::{traits::EnsureOrigin, Parameter};
        use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
        use scale_info::TypeInfo;
        use sp_runtime::AccountId32;
        use crate::traits::CallSources;
        /**
			Configuration trait of this pallet.

			Implement this type for a runtime in order to customize this pallet.
			*/
        pub trait Config: frame_system::Config {
            type RuntimeOrigin: From<Origin<Self>>;
            type AccountId: Parameter;
            type SubjectId: Parameter;
        }
        /// A dummy pallet for adding an origin to the runtime that contains
        /// separate sender and subject accounts.
        ///
        /// WARNING: This is only used for testing!
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
        /// An origin that is split into sender and subject.
        ///
        /// WARNING: This is only used for testing!
        pub type Origin<T> = DoubleOrigin<
            <T as Config>::AccountId,
            <T as Config>::SubjectId,
        >;
        /// An origin that is split into sender and subject.
        ///
        /// WARNING: This is only used for testing!
        pub struct DoubleOrigin<AccountId, SubjectId>(pub AccountId, pub SubjectId);
        #[automatically_derived]
        impl<
            AccountId: ::core::fmt::Debug,
            SubjectId: ::core::fmt::Debug,
        > ::core::fmt::Debug for DoubleOrigin<AccountId, SubjectId> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "DoubleOrigin",
                    &&self.0,
                    &&self.1,
                )
            }
        }
        #[automatically_derived]
        impl<
            AccountId: ::core::clone::Clone,
            SubjectId: ::core::clone::Clone,
        > ::core::clone::Clone for DoubleOrigin<AccountId, SubjectId> {
            #[inline]
            fn clone(&self) -> DoubleOrigin<AccountId, SubjectId> {
                DoubleOrigin(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[automatically_derived]
        impl<AccountId, SubjectId> ::core::marker::StructuralPartialEq
        for DoubleOrigin<AccountId, SubjectId> {}
        #[automatically_derived]
        impl<
            AccountId: ::core::cmp::PartialEq,
            SubjectId: ::core::cmp::PartialEq,
        > ::core::cmp::PartialEq for DoubleOrigin<AccountId, SubjectId> {
            #[inline]
            fn eq(&self, other: &DoubleOrigin<AccountId, SubjectId>) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        #[automatically_derived]
        impl<AccountId, SubjectId> ::core::marker::StructuralEq
        for DoubleOrigin<AccountId, SubjectId> {}
        #[automatically_derived]
        impl<AccountId: ::core::cmp::Eq, SubjectId: ::core::cmp::Eq> ::core::cmp::Eq
        for DoubleOrigin<AccountId, SubjectId> {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<SubjectId>;
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl<AccountId, SubjectId> ::scale_info::TypeInfo
            for DoubleOrigin<AccountId, SubjectId>
            where
                AccountId: ::scale_info::TypeInfo + 'static,
                SubjectId: ::scale_info::TypeInfo + 'static,
                AccountId: ::scale_info::TypeInfo + 'static,
                SubjectId: ::scale_info::TypeInfo + 'static,
            {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "DoubleOrigin",
                                "kilt_support::mock::mock_origin",
                            ),
                        )
                        .type_params(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    ::scale_info::TypeParameter::new(
                                        "AccountId",
                                        ::core::option::Option::Some(
                                            ::scale_info::meta_type::<AccountId>(),
                                        ),
                                    ),
                                    ::scale_info::TypeParameter::new(
                                        "SubjectId",
                                        ::core::option::Option::Some(
                                            ::scale_info::meta_type::<SubjectId>(),
                                        ),
                                    ),
                                ]),
                            ),
                        )
                        .docs(
                            &[
                                "An origin that is split into sender and subject.",
                                "",
                                "WARNING: This is only used for testing!",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| f.ty::<AccountId>().type_name("AccountId"))
                                .field(|f| f.ty::<SubjectId>().type_name("SubjectId")),
                        )
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl<AccountId, SubjectId> ::parity_scale_codec::Encode
            for DoubleOrigin<AccountId, SubjectId>
            where
                AccountId: ::parity_scale_codec::Encode,
                AccountId: ::parity_scale_codec::Encode,
                SubjectId: ::parity_scale_codec::Encode,
                SubjectId: ::parity_scale_codec::Encode,
            {
                fn encode_to<
                    __CodecOutputEdqy: ::parity_scale_codec::Output
                        + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::parity_scale_codec::Encode::encode_to(&self.0, __codec_dest_edqy);
                    ::parity_scale_codec::Encode::encode_to(&self.1, __codec_dest_edqy);
                }
            }
            #[automatically_derived]
            impl<AccountId, SubjectId> ::parity_scale_codec::EncodeLike
            for DoubleOrigin<AccountId, SubjectId>
            where
                AccountId: ::parity_scale_codec::Encode,
                AccountId: ::parity_scale_codec::Encode,
                SubjectId: ::parity_scale_codec::Encode,
                SubjectId: ::parity_scale_codec::Encode,
            {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl<AccountId, SubjectId> ::parity_scale_codec::Decode
            for DoubleOrigin<AccountId, SubjectId>
            where
                AccountId: ::parity_scale_codec::Decode,
                AccountId: ::parity_scale_codec::Decode,
                SubjectId: ::parity_scale_codec::Decode,
                SubjectId: ::parity_scale_codec::Decode,
            {
                fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                    ::core::result::Result::Ok(
                        DoubleOrigin::<
                            AccountId,
                            SubjectId,
                        >(
                            {
                                let __codec_res_edqy = <AccountId as ::parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `DoubleOrigin.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy = <SubjectId as ::parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `DoubleOrigin.1`"),
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
            }
        };
        const _: () = {
            impl<AccountId, SubjectId> ::parity_scale_codec::MaxEncodedLen
            for DoubleOrigin<AccountId, SubjectId>
            where
                AccountId: ::parity_scale_codec::MaxEncodedLen,
                AccountId: ::parity_scale_codec::MaxEncodedLen,
                SubjectId: ::parity_scale_codec::MaxEncodedLen,
                SubjectId: ::parity_scale_codec::MaxEncodedLen,
            {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                        .saturating_add(<AccountId>::max_encoded_len())
                        .saturating_add(<SubjectId>::max_encoded_len())
                }
            }
        };
        impl<AccountId: Clone, SubjectId: Clone> CallSources<AccountId, SubjectId>
        for DoubleOrigin<AccountId, SubjectId> {
            fn sender(&self) -> AccountId {
                self.0.clone()
            }
            fn subject(&self) -> SubjectId {
                self.1.clone()
            }
        }
        /// Ensure that the call was made using the split origin.
        ///
        /// WARNING: This is only used for testing!
        pub struct EnsureDoubleOrigin<AccountId, SubjectId>(
            PhantomData<(AccountId, SubjectId)>,
        );
        impl<OuterOrigin, AccountId, SubjectId> EnsureOrigin<OuterOrigin>
        for EnsureDoubleOrigin<AccountId, SubjectId>
        where
            OuterOrigin: Into<Result<DoubleOrigin<AccountId, SubjectId>, OuterOrigin>>
                + From<DoubleOrigin<AccountId, SubjectId>>,
            AccountId: From<AccountId32>,
            SubjectId: From<AccountId32>,
        {
            type Success = DoubleOrigin<AccountId, SubjectId>;
            fn try_origin(o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
                o.into()
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
            pub fn error_metadata() -> Option<
                frame_support::metadata::PalletErrorMetadata,
            > {
                None
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
                frame_support::traits::StorageVersion::default()
            }
            fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
                frame_support::traits::StorageVersion::get::<Self>()
            }
        }
        impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
            fn on_genesis() {
                let storage_version = frame_support::traits::StorageVersion::default();
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
            __Ignore(
                frame_support::sp_std::marker::PhantomData<(T,)>,
                frame_support::Never,
            ),
        }
        const _: () = {
            impl<T: Config> core::fmt::Debug for Call<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                    match *self {
                        Self::__Ignore(ref _0, ref _1) => {
                            fmt
                                .debug_tuple("Call::__Ignore")
                                .field(&_0)
                                .field(&_1)
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
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<T: Config> ::parity_scale_codec::Encode for Call<T> {}
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
                            e
                                .chain(
                                    "Could not decode `Call`, failed to read variant byte",
                                )
                        })?
                    {
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
                T: Config + 'static,
            {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Call",
                                "kilt_support::mock::mock_origin",
                            ),
                        )
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
                        .variant(::scale_info::build::Variants::new())
                }
            }
        };
        impl<T: Config> Call<T> {}
        impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
            fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
                match *self {
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
                &[]
            }
        }
        impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
            type RuntimeOrigin = frame_system::pallet_prelude::OriginFor<T>;
            fn dispatch_bypass_filter(
                self,
                origin: Self::RuntimeOrigin,
            ) -> frame_support::dispatch::DispatchResultWithPostInfo {
                match self {
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
        pub use __tt_error_token_1 as tt_error_token;
        #[doc(hidden)]
        pub mod __substrate_event_check {
            #[doc(hidden)]
            pub use __is_event_part_defined_2 as is_event_part_defined;
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
                        entries
                    },
                }
            }
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
        > frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
        for Pallet<T> {}
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
                                "kilt_support::mock::mock_origin",
                                ::tracing::Level::TRACE,
                                Some("common/support/src/mock.rs"),
                                Some(29u32),
                                Some("kilt_support::mock::mock_origin"),
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
                                "kilt_support::mock::mock_origin",
                                ::tracing::Level::TRACE,
                                Some("common/support/src/mock.rs"),
                                Some(29u32),
                                Some("kilt_support::mock::mock_origin"),
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
                                "kilt_support::mock::mock_origin",
                                ::tracing::Level::TRACE,
                                Some("common/support/src/mock.rs"),
                                Some(29u32),
                                Some("kilt_support::mock::mock_origin"),
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
                                "kilt_support::mock::mock_origin",
                                "common/support/src/mock.rs",
                                29u32,
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
    pub struct SubjectId(pub AccountId32);
    #[automatically_derived]
    impl ::core::fmt::Debug for SubjectId {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "SubjectId", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SubjectId {
        #[inline]
        fn clone(&self) -> SubjectId {
            SubjectId(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SubjectId {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SubjectId {
        #[inline]
        fn eq(&self, other: &SubjectId) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SubjectId {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SubjectId {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<AccountId32>;
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::parity_scale_codec::Encode for SubjectId {
            fn encode_to<
                __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
            >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                ::parity_scale_codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(
                &self,
            ) -> ::parity_scale_codec::alloc::vec::Vec<::core::primitive::u8> {
                ::parity_scale_codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::parity_scale_codec::Encode::using_encoded(&&self.0, f)
            }
        }
        #[automatically_derived]
        impl ::parity_scale_codec::EncodeLike for SubjectId {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::parity_scale_codec::Decode for SubjectId {
            fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                ::core::result::Result::Ok(
                    SubjectId({
                        let __codec_res_edqy = <AccountId32 as ::parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `SubjectId.0`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    }),
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for SubjectId {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("SubjectId", "kilt_support::mock"))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::unnamed()
                            .field(|f| f.ty::<AccountId32>().type_name("AccountId32")),
                    )
            }
        }
    };
    const _: () = {
        impl ::parity_scale_codec::MaxEncodedLen for SubjectId {
            fn max_encoded_len() -> ::core::primitive::usize {
                0_usize.saturating_add(<AccountId32>::max_encoded_len())
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SubjectId {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "SubjectId",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SubjectId {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SubjectId>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SubjectId;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct SubjectId",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: AccountId32 = match <AccountId32 as _serde::Deserialize>::deserialize(
                            __e,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(SubjectId(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            AccountId32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct SubjectId with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(SubjectId(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "SubjectId",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SubjectId>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl From<AccountId32> for SubjectId {
        fn from(acc: AccountId32) -> Self {
            SubjectId(acc)
        }
    }
    impl From<sr25519::Public> for SubjectId {
        fn from(acc: sr25519::Public) -> Self {
            SubjectId(acc.into())
        }
    }
    impl AsRef<[u8]> for SubjectId {
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }
}
pub mod signature {
    use frame_support::dispatch::Weight;
    use scale_info::TypeInfo;
    #[cfg(any(test, feature = "mock", feature = "runtime-benchmarks"))]
    use sp_std::marker::PhantomData;
    /// The Result of the signature verification.
    pub type SignatureVerificationResult = Result<(), SignatureVerificationError>;
    /// The Errors that can occur during signature verification.
    pub enum SignatureVerificationError {
        /// The signers information is not present on chain.
        SignerInformationNotPresent,
        /// The signature is not valid for the given payload.
        SignatureInvalid,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SignatureVerificationError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SignatureVerificationError::SignerInformationNotPresent => {
                    ::core::fmt::Formatter::write_str(f, "SignerInformationNotPresent")
                }
                SignatureVerificationError::SignatureInvalid => {
                    ::core::fmt::Formatter::write_str(f, "SignatureInvalid")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SignatureVerificationError {
        #[inline]
        fn clone(&self) -> SignatureVerificationError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SignatureVerificationError {}
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for SignatureVerificationError {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "SignatureVerificationError",
                            "kilt_support::signature",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&["The Errors that can occur during signature verification."])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "SignerInformationNotPresent",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .docs(&["The signers information is not present on chain."])
                                },
                            )
                            .variant(
                                "SignatureInvalid",
                                |v| {
                                    v
                                        .index(1usize as ::core::primitive::u8)
                                        .docs(
                                            &["The signature is not valid for the given payload."],
                                        )
                                },
                            ),
                    )
            }
        }
    };
    /// A signature verification implementation.
    pub trait VerifySignature {
        /// The identifier of the signer.
        type SignerId;
        /// The type of the payload that can be verified with the implementation.
        type Payload;
        /// The type of the signature that is expected by the implementation.
        type Signature;
        /// Verifies that the signature matches the payload and has been generated
        /// by the signer.
        fn verify(
            signer: &Self::SignerId,
            payload: &Self::Payload,
            signature: &Self::Signature,
        ) -> SignatureVerificationResult;
        /// The weight if the signature verification.
        fn weight(payload_byte_length: usize) -> Weight;
    }
    #[cfg(any(test, feature = "mock", feature = "runtime-benchmarks"))]
    pub struct EqualVerify<A, B>(PhantomData<(A, B)>);
    #[cfg(any(test, feature = "mock", feature = "runtime-benchmarks"))]
    impl<Account, Payload> VerifySignature for EqualVerify<Account, Payload>
    where
        Account: PartialEq,
        Payload: PartialEq,
    {
        type SignerId = Account;
        type Payload = Payload;
        type Signature = (Account, Payload);
        fn verify(
            delegate: &Self::SignerId,
            payload: &Self::Payload,
            signature: &Self::Signature,
        ) -> SignatureVerificationResult {
            if (delegate, payload) == (&signature.0, &signature.1) {
                SignatureVerificationResult::Ok(())
            } else {
                SignatureVerificationResult::Err(
                    SignatureVerificationError::SignatureInvalid,
                )
            }
        }
        fn weight(_: usize) -> Weight {
            Weight::zero()
        }
    }
}
pub mod traits {
    use frame_support::traits::{Currency, ReservableCurrency};
    use sp_runtime::DispatchError;
    use crate::{deposit::Deposit, free_deposit};
    /// The sources of a call struct.
    ///
    /// This trait allows to differentiate between the sender of a call and the
    /// subject of the call. The sender account submitted the call to the chain and
    /// might pay all fees and deposits that are required by the call.
    pub trait CallSources<S, P> {
        /// The sender of the call who will pay for all deposits and fees.
        fn sender(&self) -> S;
        /// The subject of the call.
        fn subject(&self) -> P;
    }
    impl<S: Clone> CallSources<S, S> for S {
        fn sender(&self) -> S {
            self.clone()
        }
        fn subject(&self) -> S {
            self.clone()
        }
    }
    impl<S: Clone, P: Clone> CallSources<S, P> for (S, P) {
        fn sender(&self) -> S {
            self.0.clone()
        }
        fn subject(&self) -> P {
            self.1.clone()
        }
    }
    /// A trait that allows version migrators to access the underlying pallet's
    /// context, e.g., its Config trait.
    ///
    /// In this way, the migrator can access the pallet's storage and the pallet's
    /// types directly.
    pub trait VersionMigratorTrait<T>: Sized {
        fn migrate(&self) -> frame_support::weights::Weight;
    }
    /// Generic filter.
    pub trait ItemFilter<Item> {
        fn should_include(&self, credential: &Item) -> bool;
    }
    pub trait StorageDepositCollector<AccountId, Key> {
        type Currency: ReservableCurrency<AccountId>;
        /// Returns the deposit of the storage entry that is stored behind the key.
        fn deposit(
            key: &Key,
        ) -> Result<
            Deposit<AccountId, <Self::Currency as Currency<AccountId>>::Balance>,
            DispatchError,
        >;
        /// Returns the deposit amount that should be reserved for the storage entry
        /// behind the key.
        ///
        /// This value can differ from the actual deposit that is reserved at the
        /// time, since the deposit can be changed.
        fn deposit_amount(key: &Key) -> <Self::Currency as Currency<AccountId>>::Balance;
        /// Store the new deposit information in the storage entry behind the key.
        fn store_deposit(
            key: &Key,
            deposit: Deposit<AccountId, <Self::Currency as Currency<AccountId>>::Balance>,
        ) -> Result<(), DispatchError>;
        /// Change the deposit owner.
        ///
        /// The deposit balance of the current owner will be freed, while the
        /// deposit balance of the new owner will get reserved. The deposit amount
        /// will not change even if the required byte and item fees were updated.
        fn change_deposit_owner(
            key: &Key,
            new_owner: AccountId,
        ) -> Result<(), DispatchError> {
            let deposit = Self::deposit(key)?;
            free_deposit::<AccountId, Self::Currency>(&deposit);
            let deposit = Deposit {
                owner: new_owner,
                ..deposit
            };
            Self::Currency::reserve(&deposit.owner, deposit.amount)?;
            Self::store_deposit(key, deposit)?;
            Ok(())
        }
        /// Update the deposit amount.
        ///
        /// In case the required deposit per item and byte changed, this function
        /// updates the deposit amount. It either frees parts of the reserved
        /// balance in case the deposit was lowered or reserves more balance when
        /// the deposit was raised.
        fn update_deposit(key: &Key) -> Result<(), DispatchError> {
            let deposit = Self::deposit(key)?;
            free_deposit::<AccountId, Self::Currency>(&deposit);
            let deposit = Deposit {
                amount: Self::deposit_amount(key),
                ..deposit
            };
            Self::Currency::reserve(&deposit.owner, deposit.amount)?;
            Self::store_deposit(key, deposit)?;
            Ok(())
        }
    }
}
