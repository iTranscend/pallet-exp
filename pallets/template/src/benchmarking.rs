//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn do_nothing() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		do_nothing(RawOrigin::Signed(caller));

		assert_eq!(true, true);
	}

	#[benchmark]
	fn cause_error() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));

		assert_eq!(Some(1011u32), Some(101u32));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
