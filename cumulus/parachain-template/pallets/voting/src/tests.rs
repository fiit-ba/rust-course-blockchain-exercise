use crate::{mock::*, Error, Event, ProposalStatus, Vote};
use codec::Encode;
use frame_support::{assert_noop, assert_ok};
use sp_io::hashing::blake2_256;


#[test]
fn register_voter() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		assert_ok!(Voting::register_voter(RuntimeOrigin::root(), 1));
		System::assert_last_event(Event::VoterRegistered { who: 1 }.into());
	});
}

#[test]
fn register_same_voter_twice() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);
		assert_noop!(
			Voting::register_voter(RuntimeOrigin::root(), 1),
			Error::<Test>::AlreadyRegistered
		);
	});
}

#[test]
fn create_proposal() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		assert_ok!(Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into()));
		System::assert_last_event(Event::ProposalCreated { who: 1, proposal_id: 1 }.into());
	});
}

#[test]
fn create_proposal_too_far_into_future() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Try to add proposal with end block number that is too far into the future (1000
		// configured currently, configurable in mock.rs)
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		assert_noop!(
			Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 1001.into()),
			Error::<Test>::TooFarInTheFuture
		);
	});
}

#[test]
fn deposit_assets() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Deposit assets
		assert_ok!(Voting::deposit_token(RuntimeOrigin::root(), 1, 100));
		System::assert_last_event(Event::TokensDeposited { who: 1, amount: 100 }.into());
	});
}

#[test]
fn cast_vote() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);

		//Vote on proposal
		assert_ok!(Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 10u128));
		System::assert_last_event(
			Event::ProposalVoted { who: 1, proposal_id: 1, vote: Vote::Aye, vote_power: 10 }.into(),
		);
	});
}

#[test]
fn cast_vote_on_not_existing_id() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);

		//Vote on proposal with non existing id
		assert_noop!(
			Voting::create_vote(RuntimeOrigin::signed(1), 2, Vote::Aye, 10u128),
			Error::<Test>::NonExistentProposal
		);
	});
}

#[test]
fn cast_vote_with_insufficient_balance() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 3u128);

		//Vote on proposal with too inefecient funds
		assert_noop!(
			Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 8u128),
			Error::<Test>::NotEnoughTokenToReserve
		);
	});
}

#[test]
fn cast_two_votes() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 145);

		//Vote on proposal with non existing id
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 2u128);
		assert_ok!(Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 10u128));
		System::assert_last_event(
			Event::ProposalVoted { who: 1, proposal_id: 1, vote: Vote::Aye, vote_power: 12 }.into(),
		);
	});
}

#[test]
fn cast_two_votes_two_accounts() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);
		let _ = Voting::register_voter(RuntimeOrigin::root(), 2);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 2, 130);

		//Vote on proposal with non existing id
		let _ = Voting::create_vote(RuntimeOrigin::signed(2), 1, Vote::Nay, 1u128);
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 2u128);
		assert_ok!(Voting::create_vote(RuntimeOrigin::signed(2), 1, Vote::Nay, 10u128));
		System::assert_last_event(
			Event::ProposalVoted { who: 2, proposal_id: 1, vote: Vote::Nay, vote_power: 11 }.into(),
		);
	});
}

#[test]
fn cast_second_vote_different() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);

		//Vote on proposal with non existing id
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 2u128);
		assert_noop!(
			Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Nay, 1u128),
			Error::<Test>::NotMatchingVote
		);
	});
}

#[test]
fn close_empty_proposal() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//We set block number to 902
		System::set_block_number(902);

		//We close proposal
		assert_ok!(Voting::close_proposal(RuntimeOrigin::signed(1), 1));
		System::assert_last_event(
			Event::ProposalClosed { who: 1, proposal_id: 1, status: ProposalStatus::Failed }.into(),
		);
	});
}

#[test]
fn close_proposal_with_votes() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);
		let _ = Voting::register_voter(RuntimeOrigin::root(), 2);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 2, 110);

		//Vote on proposal with non existing id
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 2u128);
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 2u128);
		let _ = Voting::create_vote(RuntimeOrigin::signed(2), 1, Vote::Nay, 10u128);

		//We set block number to 902
		System::set_block_number(902);

		//We close proposal
		assert_ok!(Voting::close_proposal(RuntimeOrigin::signed(1), 1));
	});
}

#[test]
fn close_proposal_that_does_not_exist() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//We set block number to 90
		System::set_block_number(90);

		//We close proposal
		assert_noop!(
			Voting::close_proposal(RuntimeOrigin::signed(1), 2),
			Error::<Test>::NonExistentProposal
		);
	});
}

#[test]
fn close_proposal_that_is_not_over() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//We set block number to 90
		System::set_block_number(90);

		//We close proposal
		assert_noop!(
			Voting::close_proposal(RuntimeOrigin::signed(1), 1),
			Error::<Test>::ProposalNotOverYet
		);
	});
}

#[test]
fn unregister_voter() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Unregister user as identity
		assert_ok!(Voting::unregister_voter(RuntimeOrigin::root(), 1));
		System::assert_last_event(Event::VoterDeregistered { who: 1 }.into());
	});
}

#[test]
fn unregister_voter_with_votes() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Vote on proposal with existing id
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 10u128);
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 40u128);

		//Unregister user as identity
		assert_ok!(Voting::unregister_voter(RuntimeOrigin::root(), 1));
		System::assert_last_event(Event::VoterDeregistered { who: 1 }.into());
	});
}

#[test]
fn unregister_voter_with_votes_after_proposal_is_over() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Deposit assets
		let _ = Voting::deposit_token(RuntimeOrigin::root(), 1, 110);

		//Create proposal
		let proposal_hash = blake2_256(("Ahoj").encode().as_slice()).into();
		let _ = Voting::create_proposal(RuntimeOrigin::signed(1), proposal_hash, 900.into());

		//Vote on proposal with existing id
		let _ = Voting::create_vote(RuntimeOrigin::signed(1), 1, Vote::Aye, 10u128);

		//We set block number to 902
		System::set_block_number(902);

		//We close proposal
		let _ = Voting::close_proposal(RuntimeOrigin::signed(1), 1);

		//Unregister user as identity
		assert_ok!(Voting::unregister_voter(RuntimeOrigin::root(), 1));
		System::assert_last_event(Event::VoterDeregistered { who: 1 }.into());
	});
}

#[test]
fn unregister_voter_without_identity() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Unregister user as identity
		assert_noop!(
			Voting::unregister_voter(RuntimeOrigin::root(), 1),
			Error::<Test>::NonExistentIdentity
		);
	});
}

#[test]
fn unregister_voter_two_times() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		//Register user as identity
		let _ = Voting::register_voter(RuntimeOrigin::root(), 1);

		//Unregister user as identity
		let _ = Voting::unregister_voter(RuntimeOrigin::root(), 1);

		//Unregister user as identity
		assert_noop!(
			Voting::unregister_voter(RuntimeOrigin::root(), 1),
			Error::<Test>::NonExistentIdentity
		);
	});
}
