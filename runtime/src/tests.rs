/// tests for this module

// Test cases:
//  1. record_price if called store item in storage
//  2. record_price can only be called from unsigned tx
//  3. with multiple record_price of same sym inserted. On next cycle, the average of the price is calculated
//  4. can fetch for BTC, parse the JSON blob and get a price > 0 out

use crate::price_fetch::{self, Module, Trait};
use primitives::{H256};
use support::{dispatch, impl_outer_origin, impl_outer_dispatch, assert_ok,
  parameter_types, weights::Weight};
use sp_runtime::{
  traits::{BlakeTwo256, IdentityLookup},
  testing::{Header, UintAuthorityId, TestXt},
  Perbill
};

impl_outer_origin! {
  pub enum Origin for TestRuntime {}
}

impl_outer_dispatch! {
  pub enum Call for TestRuntime where origin: Origin {
    price_fetch::PriceFetchModule,
  }
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct TestRuntime;

parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const MaximumBlockWeight: Weight = 1024;
  pub const MaximumBlockLength: u32 = 2 * 1024;
  pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for TestRuntime {
  type Origin = Origin;
  type Call = Call;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = ();
  type BlockHashCount = BlockHashCount;
  type MaximumBlockWeight = MaximumBlockWeight;
  type MaximumBlockLength = MaximumBlockLength;
  type AvailableBlockRatio = AvailableBlockRatio;
  type Version = ();
  type ModuleToIndex = ();
}

impl timestamp::Trait for TestRuntime {
  type Moment = u64;
  type OnTimestampSet = ();
  type MinimumPeriod = ();
}

pub type Extrinsic = TestXt<Call, ()>;
type AccountId = u64;
type AccountIndex = u64;
type SubmitPFTransaction = system::offchain::TransactionSubmitter<
  price_fetch::crypto::Public, Call, Extrinsic>;

parameter_types! {
  pub const BlockFetchDur: u64 = 1;
}

pub type PriceFetchModule = Module<TestRuntime>;

impl system::offchain::CreateTransaction<TestRuntime, Extrinsic> for Call {
  type Public = ();
  type Signature = u64;

  // Pay close attention to how this implementation --drastically-- differs from the real one
  // in the top level runtime aggregator file, and how it creates a mock signature (which is
  // actually the account id itself).
  fn create_transaction<F: system::offchain::Signer<AccountId, Self::Signature>>(
    call: Call,
    public: Self::Public,
    account: AccountId,
    _index: AccountIndex,
  ) -> Option<(Call, <Extrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
    let extra = ();
    Some((call, (account, extra)))
  }
}

impl Trait for TestRuntime {
  type Event = ();
  type Call = Call;
  type SubmitSignedTransaction = SubmitPFTransaction;
  type SubmitUnsignedTransaction = SubmitPFTransaction;

  // Wait period between automated fetches. Set to 0 disable this feature.
  //   Then you need to manucally kickoff pricefetch
  type BlockFetchDur = BlockFetchDur;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> runtime_io::TestExternalities {
  system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap().into()
}

#[test]
fn it_works_for_default_value() {
  new_test_ext().execute_with(|| {
    assert_eq!(1, 1);
  });
}