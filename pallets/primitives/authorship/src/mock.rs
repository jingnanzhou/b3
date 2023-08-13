
#![cfg(test)]

use crate as pallet_authorship;
use pallet_authorship::*;

use sp_core::H256;
use sp_runtime::{
	testing::{Header, DigestItem},
	traits::{BlakeTwo256, IdentityLookup},
};
use frame_support::{
	traits::{ConstU32, ConstU64},
	ConsensusEngineId,
};


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
	Block = Block,
	NodeBlock = Block,
	UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}


impl pallet::Config for Test {
	type FindAuthor = AuthorGiven;
	type UncleGenerations = ConstU64<5>;
	type FilterUncle = SealVerify<VerifyBlock>;
	type EventHandler = ();
}

const TEST_ID: ConsensusEngineId = [1, 2, 3, 4];
pub struct AuthorGiven;

impl FindAuthor<u64> for AuthorGiven {

	fn find_author<'a, I>(digests: I) -> Option<u64> 
		where I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
		{
			for (id, mut data) in digests {
				if id == TEST_ID {
					return u64::decode( &mut data).ok()
				}
			}
			None
		}
}

pub struct VerifyBlock;

impl VerifySeal<Header, u64> for VerifyBlock {

	fn verify_seal(header: &Header) -> Result<Option<u64>, &'static str> {
		let pre_runtime_digests = header.digest.logs.iter().filter_map(|d| d.as_pre_runtime());
		let seals = header.digest.logs.iter().filter_map(|d| d.as_seal());

		let author = AuthorGiven::find_author(pre_runtime_digests).ok_or_else(|| "no author")?;

		for (id, mut seal) in seals {
			if id == TEST_ID {

				match u64::decode( &mut seal) {
					Err(_) => return Err("wrong seal"),
					Ok(a) => {
						if a != author {
							return Err("wrong author in seal")
						}
						break
					},
				}
			}
		}
		Ok(Some(author))
	}
}


pub fn seal_header( mut header: Header, author: u64) ->Header {
	{
		let digest = header.digest_mut();
		digest.logs.push(DigestItem::PreRuntime(TEST_ID, author.encode()));
		digest.logs.push(DigestItem::Seal(TEST_ID, author.encode()));
	}
	header
}

pub fn create_header(number: u64, parent_hash: H256, state_root: H256) ->Header {

	Header::new(number, Default::default(), state_root, parent_hash, Default::default())
}

pub fn new_test_ext() ->sp_io::TestExternalities{
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	t.into()
}
