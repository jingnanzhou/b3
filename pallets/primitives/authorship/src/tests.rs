

#![cfg(test)]

use crate as pallet_authorship;
use pallet_authorship::*;
use mock::*;
use sp_core::H256;
use sp_runtime::{

    testing::Header,
};
use frame_support::{
    traits::{OnFinalize, OnInitialize},
};

#[test]
fn prune_old_uncles_works() {

    use crate::UncleEntryItem::*;

    crate::mock::new_test_ext().execute_with(||{

        let hash = Default::default();
        let author = Default::default();
        let uncles = vec![
            InclusionHeight(1u64),
            Uncle(hash, Some(author)),
            Uncle(hash, None),
            Uncle(hash, None),
            InclusionHeight(2u64),
            Uncle(hash, None),
            InclusionHeight(3u64),
            Uncle(hash, None),
        ];
        let uncles = BoundedVec::try_from(uncles).unwrap();

        <Authorship as Store>::Uncles::put(uncles);
        Authorship::prune_old_uncles(3);

        let uncles = <Authorship as Store>::Uncles::get();
        assert_eq!(uncles, vec![InclusionHeight(3u64), Uncle(hash, None)]);
    })
}

#[test]
fn reject_bad_uncles(){
    new_test_ext().execute_with(||{

        let author_a =69;

        struct CanonChain {
            inner: Vec<Header>,
        }

        impl CanonChain {
            fn best_bash(&self) -> H256 {
                self.inner.last().unwrap().hash()
            }

            fn canon_hash(&self, index: usize) -> H256 {
                self.inner[index].hash()
            }
            fn header(&self, index: usize) -> &Header {
                &self.inner[index]
            }
            fn push( &mut self, header: Header) {

                self.inner.push(header)
            }

        }

        let mut canon_chain = CanonChain{
            inner: vec![seal_header(
                create_header(0, Default::default(), Default::default()), 999)],
        };

        let initialize_block = |number, hash: H256| {
            System::reset_events();
            System::initialize(&number, &hash, &Default::default())
        };

        for number in 1..8 {

            initialize_block(number, canon_chain.best_bash());
            let header = seal_header( System::finalize(), author_a);
            canon_chain.push(header);
        }
        initialize_block(8, canon_chain.best_bash());
        {
            let uncle_a = seal_header(
                create_header(3, canon_chain.canon_hash(2), [1; 32].into()),
                author_a,
            );
            assert_eq!(
                Authorship::verify_and_import_uncles(vec![uncle_a.clone(), uncle_a.clone()]),
                Err(Error::<Test>::UncleAlreadyIncluded.into()),
            );
        }       
			// 2 of the same uncle at different times.
        {
            let uncle_a = seal_header(
                create_header(3, canon_chain.canon_hash(2), [1; 32].into()),
                author_a,
            );

            assert!(Authorship::verify_and_import_uncles(vec![uncle_a.clone()]).is_ok());

            assert_eq!(
                Authorship::verify_and_import_uncles(vec![uncle_a.clone()]),
                Err(Error::<Test>::UncleAlreadyIncluded.into()),
            );
        }

			// same uncle as ancestor.
        {
            let uncle_clone = canon_chain.header(5).clone();

            assert_eq!(
                Authorship::verify_and_import_uncles(vec![uncle_clone]),
                Err(Error::<Test>::UncleAlreadyIncluded.into()),
            );
        }

			// uncle without valid seal.
        {
            let unsealed = create_header(3, canon_chain.canon_hash(2), [2; 32].into());
            assert_eq!(
                Authorship::verify_and_import_uncles(vec![unsealed]),
                Err("no author".into()),
            );
        }

			// old uncles can't get in.
        {
            assert_eq!(System::block_number(), 8);

            let gen_2 = seal_header(
                create_header(2, canon_chain.canon_hash(1), [3; 32].into()),
                author_a,
            );

            assert_eq!(
                Authorship::verify_and_import_uncles(vec![gen_2]),
                Err(Error::<Test>::OldUncle.into()),
            );
        }

			// siblings are also allowed
        {
            let other_8 = seal_header(
                create_header(8, canon_chain.canon_hash(7), [1; 32].into()),
                author_a,
            );

            assert!(Authorship::verify_and_import_uncles(vec![other_8]).is_ok());
        }

    });
}

#[test]
fn maximum_bound() {
    new_test_ext().execute_with( || {

        let mut max_item_count = 0;

        let mut author_counter = 0;
        let mut current_depth = 1;
        let mut parent_hash: H256 = [1; 32].into();
        let mut uncles = vec![];
        // We deliberately run this for more generations than the limit
        // so that we can get the `Uncles` to hit its cap.

        for _ in 0..<<Test as Config>::UncleGenerations as Get<u64>>::get()+3 {

            let new_uncles: Vec<_> = (0..MAX_UNCLES).map(|_| {
                System::reset_events();
                System::initialize(&current_depth, &parent_hash, &Default::default());

                author_counter += 1;
                seal_header(System::finalize(), author_counter)
                
            }).collect();



            author_counter += 1;
            System::reset_events();
            System::initialize(&current_depth, &parent_hash, &Default::default());
            Authorship::on_initialize(current_depth);

            Authorship::set_uncles(RuntimeOrigin::none(), uncles).unwrap();
            Authorship::on_finalize(current_depth);
            max_item_count =
                std::cmp::max(max_item_count, <Authorship as Store>::Uncles::get().len());

            let new_parent = seal_header(System::finalize(), author_counter);
            parent_hash = new_parent.hash();
            uncles = new_uncles;
            current_depth += 1;


        }
        assert_eq!(max_item_count, MaxUncleEntryItems::<Test>::get() as usize);

    });
}




#[test]
fn sets_author_lazily() {
    new_test_ext().execute_with(|| {
        let author = 42;
        let mut header =
            seal_header(create_header(1, Default::default(), [1; 32].into()), author);

        header.digest_mut().pop(); // pop the seal off.
        System::reset_events();
        System::initialize(&1, &Default::default(), header.digest());

        assert_eq!(Authorship::author(), Some(author));
    });
}

#[test]
fn one_uncle_per_author_per_number() {
    type Filter = OnePerAuthorPerHeight<VerifyBlock, u64>;

    let author_a = 42;
    let author_b = 43;

    let mut acc: <Filter as FilterUncle<Header, u64>>::Accumulator = Default::default();
    let header_a1 = seal_header(create_header(1, Default::default(), [1; 32].into()), author_a);
    let header_b1 = seal_header(create_header(1, Default::default(), [1; 32].into()), author_b);

    let header_a2_1 =
        seal_header(create_header(2, Default::default(), [1; 32].into()), author_a);
    let header_a2_2 =
        seal_header(create_header(2, Default::default(), [2; 32].into()), author_a);

    let mut check_filter = move |uncle| Filter::filter_uncle(uncle, &mut acc);

    // same height, different author is OK.
    assert_eq!(check_filter(&header_a1), Ok(Some(author_a)));
    assert_eq!(check_filter(&header_b1), Ok(Some(author_b)));

    // same author, different height.
    assert_eq!(check_filter(&header_a2_1), Ok(Some(author_a)));

    // same author, same height (author a, height 2)
    assert!(check_filter(&header_a2_2).is_err());
}

