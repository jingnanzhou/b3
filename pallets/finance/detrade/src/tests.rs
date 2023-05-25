#![cfg(test)]
use crate::mock::*;
use frame_support::assert_ok;

//const bank_name: &str = "test";
//const beneficiary: &str = "test_b";
#[test]
fn trade_finance_works() {
	new_test_ext().execute_with(|| {
 
        let lc= crate::LetterOfCredit {
            id: 1,
            trade_id: 1,
            amount: 1,
            beneficiary: vec![0].try_into().unwrap(),
            issuing_bank: vec![0].try_into().unwrap(),
            status: 1,
        };
        
		assert_ok!(TradeFinance::send_lc(RuntimeOrigin::signed(1), lc));
	});
}
