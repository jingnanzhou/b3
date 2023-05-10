#![cfg(test)]
use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn trade_finance_works() {
	new_test_ext().execute_with(|| {
        let lc= crate::LetterOfCredit {
            id: 1,
            trade_id: 1,
            amount: 1,
            beneficiary: 1,
            issuing_bank: 1,
            status: 1,
        };
		assert_ok!(TradeFinance::send_lc(RuntimeOrigin::signed(1), lc));
	});
}
