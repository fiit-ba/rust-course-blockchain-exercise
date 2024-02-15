#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, LockableCurrency, ReservableCurrency},
	};
	use scale_info::prelude::vec;
	use frame_system::{
		pallet_prelude::{OriginFor, *},
	};
use sp_core::U256;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type to access the Balances Pallet.
		type Currency: Currency<Self::AccountId>
			+ ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId>;

		/// A configurable maximum number of users for this pallet.
		type MaxVoters: Get<u32>;

		/// Configurable maximum future block amount for voting
		type MaxFutureBlock: Get<u32>;

		/// Configurable maximum votes per account
		type MaxVotes: Get<u32>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Returns users registered for voting
	#[pallet::storage]
	pub type RegisteredVoters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

	//Returns proposal for specific id
	#[pallet::storage]
	pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, u32, Proposal<T>>;

	//Returns user ids in vector for specific proposal
	#[pallet::storage]
	pub type ProposalVotes<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, BoundedVec<T::AccountId, T::MaxVoters>>;

	//Returns id of last proposal
	#[pallet::storage]
	pub type LastProposalId<T: Config> = StorageValue<_, u32>;

	//Map accountid onto user_vote structure
	#[pallet::storage]
	pub type UserVotes<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<UserVote<T>, T::MaxVotes>>;

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug)]
	#[scale_info(skip_type_params(T))]
	pub struct Proposal<T: Config> {
		id: u32,
		text: T::Hash,
		vote_count_aye: BalanceOf<T>,
		vote_count_nay: BalanceOf<T>,
		end_block: U256,
		status: ProposalStatus,
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, PartialEq, Eq, Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct UserVote<T: Config> {
		proposal_id: u32,
		vote_amount: BalanceOf<T>,
		vote_power: BalanceOf<T>,
		vote: Vote,
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Clone, PartialEq, Eq)]
	pub enum Vote {
		Aye,
		Nay,
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, PartialEq, Eq, Debug, Clone)]
	pub enum ProposalStatus {
		InProgress,
		Passed,
		Failed,
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event for deposited tokens //USED IN DEPOSIT TOKENS
		/// parameters [who, amount]
		TokensDeposited { who: T::AccountId, amount: BalanceOf<T> },

		/// Event for registered voter //USED IN REGISTER VOTER
		/// parameters [who]
		VoterRegistered { who: T::AccountId },

		/// Event for voter deregistered //USED IN DEREGISTER VOTER
		/// parameters [who]
		VoterDeregistered { who: T::AccountId },

		/// Event for proposal created //USED IN CREATE PROPOSAL
		/// parameters [who, proposal_id]
		ProposalCreated { who: T::AccountId, proposal_id: u32 },

		/// Event for proposal voted //USED IN CREATE VOTE
		/// parameters [who, proposal_id, vote, vote_power (Total vote power of user)]
		ProposalVoted { who: T::AccountId, proposal_id: u32, vote: Vote, vote_power: BalanceOf<T> },

		/// Event for proposal closed //USED IN CLOSE PROPOSAL
		/// parameters [who, proposal_id, status]
		ProposalClosed { who: T::AccountId, proposal_id: u32, status: ProposalStatus },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Not enough token to reserve
		NotEnoughTokenToReserve,
		/// Non existent proposal
		NonExistentProposal,
		/// Non existent identity
		NonExistentIdentity,
		/// Too far in the future
		TooFarInTheFuture,
		/// Proposal is not able to  close
		ProposalNotOverYet,
		/// Proposal is either over or does not exist
		ProposalNotInProgress,
		/// Identity already registered
		AlreadyRegistered,
		/// Not matching vote
		NotMatchingVote,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Function to register identity and de-sybil users
		/// Has parameters provided by user:
		/// Account
		///
		/// Edge cases: none necessary except for making sure only ROOT can execute this.
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn register_voter(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(!Self::is_registered(&who), Error::<T>::AlreadyRegistered);
			RegisteredVoters::<T>::insert(who.clone(), ());

			//Emit event voter registered
			Self::deposit_event(Event::VoterRegistered { who });
			Ok(())
		}

		/// Function to deregister identity
		/// Has parameters provided by user:
		/// Account
		///
		/// Edge cases: not existing autority, proposal does not exist
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn unregister_voter(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			//We check if user has identity
			ensure!(Self::is_registered(&who), Error::<T>::NonExistentIdentity);

			//We query all proposals and remove voting power of account id from them
			let user_votes = UserVotes::<T>::get(&who).unwrap_or_default();

			//We iterate through all proposals user voted on and remove voting power and return
			// tokens
			for vote in user_votes {
				//We check if proposal exists
				ensure!(Self::exists(vote.proposal_id), Error::<T>::NonExistentProposal);
				let mut proposal = Proposals::<T>::get(vote.proposal_id).unwrap();

				//If user voted aye we remove aye voting power
				if vote.vote == Vote::Aye {
					proposal.vote_count_aye = proposal.vote_count_aye - vote.vote_power;
				}
				//If user voted nay we remove nay voting power
				else if vote.vote == Vote::Nay {
					proposal.vote_count_nay = proposal.vote_count_nay - vote.vote_power;
				}

				//We update proposal
				Proposals::<T>::insert(vote.proposal_id, proposal);
			}

			//We remove user votes
			UserVotes::<T>::remove(who.clone());

			//Remove user from registered voters
			RegisteredVoters::<T>::remove(who.clone());

			//Emit event voter deregistered
			Self::deposit_event(Event::VoterDeregistered { who });

			Ok(())
		}

		///Function to allow user create proposal
		/// Has parameters provided by user:
		/// Text
		/// End block
		///
		///
		/// Edge cases: too long text and end block too far in the future so it will take storage
		/// space. Too far into future is cured, allowing only x block long proposals
		/// Too long text, I think hash takes care of that
		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_proposal(
			origin: OriginFor<T>,
			text: T::Hash,
			end_block: U256,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//We check if block is not too far in the future
			//We get current block and add x blocks and check if end block is not bigger than that


			//Get max future block configurable max future block


			//Check if block is not too far in the future


			//Query latest proposal id

			// Insert new proposal into storage

			//Update last proposal id


			//Emit event proposal created


			Ok(())
		}

		///Function to allow autorized user create vote
		/// Has parameters provided by user:
		/// Id of proposal
		/// Vote (aye || nay)
		/// vote power user want to vote with
		///
		/// Edge cases: id not existent, proposal not in progress, user not registered.
		#[pallet::call_index(4)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_vote(
			origin: OriginFor<T>,
			id: u32,
			vote: Vote,
			vote_powers: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//We check if user is registered
			ensure!(Self::is_registered(&who), Error::<T>::NonExistentIdentity);

			//We check if proposal exists
			ensure!(Self::exists(id), Error::<T>::NonExistentProposal);

			let mut proposal = Proposals::<T>::get(id).ok_or(Error::<T>::NonExistentProposal)?;

			//We check if proposal is in progress
			ensure!(
				proposal.status == ProposalStatus::InProgress,
				Error::<T>::ProposalNotInProgress
			);

			//We get the amount of tokens user has
			let tokens = T::Currency::free_balance(&who);

			//We check if user has enough to add certain voting power by querying his current vote
			// power, adding it together with voting power He wants to add and substracting the
			// total amount with amount he has reserved already
			let user_vote = UserVotes::<T>::get(&who).unwrap_or_default();

			//to check if we already voted
			let mut voted = false;

			//We iterate through all votes and check if we find proposal id
			let mut new_power = 0u32.into();

			for voting in user_vote {
				if voting.proposal_id == id {
					//We chceck if user is not trying to vote different vote
					ensure!(voting.vote == vote, Error::<T>::NotMatchingVote);

					//We calculate new voting power
					new_power = voting.vote_power + vote_powers;

					//We need to save old voting amount
					let old_amount = voting.vote_amount;

					//We create new final sum and substract what is already reserved
					ensure!(
						tokens > (new_power * new_power) - old_amount,
						Error::<T>::NotEnoughTokenToReserve
					);
					voted = true;
				}
			}
			//We also check if he have not yet got any votes
			ensure!(tokens > (vote_powers * vote_powers), Error::<T>::NotEnoughTokenToReserve);

			//If we did not receive error not enough tokens to reserve, we can continue
			//if we did not vote we create new vote
			if !voted {
				//We create new vote
				let new_vote: UserVote<T> = UserVote {
					proposal_id: id,
					vote: vote.clone(),
					vote_power: vote_powers,
					vote_amount: vote_powers * vote_powers,
				};

				//We add new vote to user votes
				//mutate
				let _ = UserVotes::<T>::mutate(&who, |x| -> Result<(), ()> {
					*x = Some(vec![new_vote].try_into().map_err(|_| ())?);
					Ok(())
				});

				//We add account id into voters for this proposal id
				let _ = ProposalVotes::<T>::mutate(id, |x| -> Result<(), ()> {
					if let Some(x) = x {
						let _ = x.try_push(who.clone());
						Ok(())
					} else {
						*x = Some(vec![who.clone()].try_into().map_err(|_| ())?);
						Ok(())
					}
				});

				//retrieve proposal votes to check
				let _proposal_votes = ProposalVotes::<T>::get(id).unwrap_or_default();

				//We update proposal vote count
				if vote == Vote::Aye {
					proposal.vote_count_aye = proposal.vote_count_aye + vote_powers;
				} else {
					proposal.vote_count_nay = proposal.vote_count_nay + vote_powers;
				}

				//We update proposal
				Proposals::<T>::insert(id, proposal);

				//We reserve tokens
				let _ = T::Currency::reserve(&who, vote_powers * vote_powers)?;

				//We emit event vote created
				Self::deposit_event(Event::ProposalVoted {
					who,
					proposal_id: id,
					vote,
					vote_power: vote_powers,
				});
			} else {
				//We save previous voting amount so we can substract
				let mut prev_vote_amnt = 0u32.into();

				//We save new voting amount so we can add
				let mut vote_amnt = 0u32.into();

				//We get votes
				let mut user_votee = UserVotes::<T>::get(&who).unwrap_or_default();

				//We iterate through votes and update vote
				for voting in &mut user_votee {
					if voting.proposal_id == id {
						//We update vote
						voting.vote = vote.clone();
						prev_vote_amnt = voting.vote_amount;
						voting.vote_power = voting.vote_power + vote_powers;
						voting.vote_amount =
							(voting.vote_power * voting.vote_power) - voting.vote_amount;
						vote_amnt = voting.vote_amount;
					}
				}

				//We update user votes
				UserVotes::<T>::insert(&who, user_votee);

				//We update proposal vote count
				if vote == Vote::Aye {
					proposal.vote_count_aye = proposal.vote_count_aye + vote_powers;
				} else {
					proposal.vote_count_nay = proposal.vote_count_nay + vote_powers;
				}

				//We update proposal
				Proposals::<T>::insert(id, proposal);

				//We reserve tokens
				let _ = T::Currency::unreserve(&who, prev_vote_amnt);
				let _ = T::Currency::reserve(&who, vote_amnt);

				//We emit event vote created
				Self::deposit_event(Event::ProposalVoted {
					who,
					proposal_id: id,
					vote,
					vote_power: new_power,
				});
			}

			Ok(())
		}

		///Function to allow sudo add balance // FOR TESTING PURPOSE ONLY
		/// Has parameters provided by user:
		/// account
		/// amount
		///
		///
		/// Edge cases: none
		#[pallet::call_index(5)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn deposit_token(
			origin: OriginFor<T>,
			who: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;

			//We deposit test tokens
			let _ = T::Currency::deposit_creating(&who, amount);

			//We emit an event about succesful deposit
			Self::deposit_event(Event::TokensDeposited { who, amount });

			Ok(())
		}

		///Function to close proposal
		/// Has parameters provided by user:
		/// Id of proposal
		///
		/// Edge cases: proposal does not exist, proposal not over yet
		#[pallet::call_index(6)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn close_proposal(origin: OriginFor<T>, id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//We check if proposal exists
			ensure!(Self::exists(id), Error::<T>::NonExistentProposal);

			//We get the proposal
			let mut proposal = Proposals::<T>::get(id).ok_or(Error::<T>::NonExistentProposal)?;

			//We check if proposal is in progress
			ensure!(
				proposal.status == ProposalStatus::InProgress,
				Error::<T>::ProposalNotInProgress
			);

			let current_block = frame_system::Pallet::<T>::block_number();

			//We check if current block is bigger than end block
			ensure!(current_block.into() > proposal.end_block, Error::<T>::ProposalNotOverYet);

			//We update proposal status based on aye and naye count
			match proposal.vote_count_aye > proposal.vote_count_nay {
				true => {
					//We update the proposal status
					proposal.status = ProposalStatus::Passed;
				},
				false => {
					//We update the proposal status
					proposal.status = ProposalStatus::Failed;
				},
			}

			//We get all users that voted on this proposal
			let proposal_votes = ProposalVotes::<T>::get(id).unwrap_or_default();

			//We iterate through all users that voted on this proposal and release their tokens
			// based on vote_amount
			for user in proposal_votes {
				//We get user vote
				let user_vote = UserVotes::<T>::get(&user).unwrap_or_default();

				//We go through user votes and check if proposal id is equal to proposal id then
				// release tokens
				for vote in user_vote {
					if vote.proposal_id == id {
						//We release tokens
						T::Currency::unreserve(&user, vote.vote_amount.into());
					}
				}
			}

			//We free storage from proposal id
			Proposals::<T>::remove(&id);

			//We query account ids on proposal id
			let proposal_votes = ProposalVotes::<T>::get(id).unwrap_or_default();

			//We iterate through account ids and remove proposals in each account id containing
			// proposal id
			for user in proposal_votes {
				//We get user vote
				let mut user_vote = UserVotes::<T>::get(&user).unwrap_or_default();
				let us_vote = user_vote.clone();

				//We iterate through  user votes and remove one that matches proposal id
				let mut index = 0;
				for vote in us_vote {
					if vote.proposal_id == id {
						//We remove vote from user votes
						user_vote.remove(index);
					}
					index += 1;
				}

				//We update user votes vector
				UserVotes::<T>::insert(&user, user_vote);
			}
			//We remove ProposalVotes from storage
			ProposalVotes::<T>::remove(&id);

			//We emit an event about succesful proposal close
			Self::deposit_event(Event::ProposalClosed {
				who,
				proposal_id: id,
				status: proposal.status,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn is_registered(who: &T::AccountId) -> bool {
			RegisteredVoters::<T>::contains_key(who)
		}

		//Check if proposal exist
		pub fn exists(id: u32) -> bool {
			Proposals::<T>::contains_key(id)
		}
	}
}
