# FIIT Programovanie v jazyku Rust cvičenie 12 - Rust v Blockchaine.

Vitajte na bonusovom cvičení zameranom na využitie jazyka Rust v Blockchaine

## Prerequizity
- Rust: ```curl –proto ’=https’ –tlsv1.2 -sSf https://sh.rustup.rs | sh```
- Rust: verzia nightly ```rustup default nightly```
- Rust wasm ```rustup target add wasm32-unknown-unknown```
- OS specific moduly ktoré vypíše pri kompilácii (Treba inštalovať dodatočne keďže každé OS má rôzne prerequizity)
- 60gb úložisko a 8gb ram

## Zadanie
Máte predpripravenú šablónu Parachainu. Doplnte paletu [voting](https://github.com/dudo50/polkadot-sdk/blob/master/cumulus/parachain-template/pallets/voting/src/lib.rs) o funkciu ktorá umožní vytvárať referendá pre kvadratické hlasovanie.
Riešenie otestujte v Runtime.

### Ako funguje zjednodušené kvadratické hlasovanie:

- Alice hlasuje aye s ```1000``` hlasovacou silou. Musí rezervovať ```1000000``` tokenov

- Bob hlasuje nay s hlasovacou silou ```30```. Musí rezervovať ```900``` tokenov

- Bob zahlasuje nay s ďalšou hlasovacou silou ```70```. Musí rezervovať ďalších ```9910``` tokenov (Dokopy má Bob hlasovaciu silu ```100``` a rezervovaných ```10000``` tokenov)

Celková hlasovacia sila na referendu je teda:

Aye - 1000

Nay - 100

Referendum úspešne ukončí v **prospech tvorcu**.

## Postup

- Krok 1. Naviguj do palety voting ```cd cumulus/parachain-template/pallets/voting```
- Krok 2. Doplň funkciu "create_proposal" podľa komentárov v kóde
- Krok 2.1 Otestovanie doplnenej funkcie cez ```cargo build```
- Krok 2.2 Otestovanie doplnenej funkcie cez testy ```cargo test --package pallet-voting --lib -- tests --nocapture```
- Krok 3. Skompilovanie Polkadotu - ```cd Polkadot``` a ```cargo build --release```
- Krok 4. Skompilovanie Parachain šablóny - ```cargo b -r -p parachain-template-node```
- Krok 5. Prejsť do zložky binaries ```cd binaries```
- Krok 6. Stiahnuť si zombienet binárku pre špecifický OS (Linux a Mac only) ```https://github.com/paritytech/zombienet/releases/tag/v1.3.90```
(Windows buď cez virtuálnu mašinu alebo manuálne spustiť sieť ```https://docs.substrate.io/tutorials/build-a-parachain/connect-a-local-parachain/```)
- Krok 7. Otvoriť PolkadotJS link ktorú poskytol Zombienet
- Krok 8. Extrinsics tab (Otestuj si voting paletku) vytvor proposal
 <img width="1440" alt="Screenshot" src="https://github.com/dudo50/polkadot-sdk/assets/55763425/e3f570a8-234b-4975-9263-cbca02772a22">
- Krok 9. Sudo tab (Vytvor si voting entitu cez sudo) pomocou sudo.sudo.register_voter
 <img width="1440" alt="Screenshot1" src="https://github.com/dudo50/polkadot-sdk/assets/55763425/476f745b-9ad3-4309-a11e-284faaeaa857">
- Krok 10. Chainstate tab (Querni si dáta o proposale)
<img width="1439" alt="Screenshot2" src="https://github.com/dudo50/polkadot-sdk/assets/55763425/33a7f160-3f56-4d84-ad14-27f1b3ae7a23">

Práve si sa naučil základy interakcie s Blockchainom ✅.

## Bonusové zadanie
- Všimol si si niektoré limitácie ktoré táto paleta obsahuje? Aké?
- Rozšír paletu o žiadosti a poplatky za registráciu
- Rozšír referendá o možnosť žiadať o prostriedky. Keď referendum skončí úspešne (AYE>NAY) prostriedky užívateľovi vyplať z Pokladnice (Vytvor špeciálny účet ktorý na to bude slúžiť a depozituj mu aktíva)

## Ak som stratený
Ak som stratený mám dostupných nasledovných žolíkov ktoré využívam postupne.
- ChatGPT?
- Ťahák?
- Cvičiaci?
- Riešenie?

### Ťahák

#### Opis funkcií v palete

-  ```register_voter (origin: OriginFor<T>, who: T::AccountId)```
Funkcia registruje hlasujúceho (Registruje jeho entitu - hlasovať môžu len registrovaný voliči). Potrebné **sudo**.

-  ```unregister_voter (origin: OriginFor<T>, who: T::AccountId)```
Funkcia na deregistráciu hlasujúceho. Potrebné **sudo**.

-  ```create_proposal (origin: OriginFor<T>, text: T::Hash, end_block: T::BlockNumber )```
Funkcia na vytvorenie referenda, môžu ho vytvoriť aj normálny užívatelia. (Text si zahashuj!)

-  ``` create_vote (origin: OriginFor<T>, id: u32, vote: Vote, vote_powers: BalanceOf<T>)```
Funkcia na hlasovanie - len registrovaný používateľ môže hlasovať

-  ```deposit_token (origin: OriginFor<T>, who: T::AccountId, amount: BalanceOf<T>)```
Testovacia funkcia na pridanie aktív. Potrebné **sudo**

-  ```close_proposal(origin: OriginFor<T>, id: u32)```
Funkcia na uzavretie referenda, referendum je možné uzavrieť po čase na ktorý bolo aktivované. Zavrieť ho môže ktokoľvek (Aby sa proposaly nekopili a nezahltili úložisko). 

#### Eventy ktoré sa v palete používajú

-  **TokensDeposited** {who: T::AccountId, amount: BalanceOf<T>}: Informuje používateľa o pridaných aktívach na účet
-  **VoterRegistered** {who: T::AccountId}: Informuje používateľa o registrácii entity
-  **VoterDeregistered** {who: T::AccountId}: Informuje používateľa o deregistrácii entity
-  **ProposalCreated** {who: T::AccountId, proposal_id: u32}: Informuje používateľa o úspešnom vytvorení referenda
-  **ProposalVoted** {who: T::AccountId, proposal_id: u32, vote: Vote, vote_power:  BalanceOf<T>}: Informuje používateľa o úspešnom hlasovaní
-  **ProposalClosed** {who: T::AccountId, proposal_id: u32, status: ProposalStatus}: Informuje používateľa o úspešnom uzavretí hlasovania

  

#### Paletou emitované chyby

-  **NotEnoughTokenToReserve**: Používateľ chce hlasovať s viac aktívami než disponuje
-  **NonExistentProposal**: Referendum neexistuje (Buď bolo uzavreté alebo nebolo nikdy vytvorené)
-  **NonExistentIdentity**: Používateľ nie je registrovaný
-  **TooFarInTheFuture**: Používateľ chce vytvoriť referendum na príliš ďalekú budúcnosť (Limit 1000blokov do budúcna - možné zmeniť v mock.rs)
-  **ProposalNotOverYet**: Používateľ chce uzatvoriť referendum keď ešte stále nie je rozhodnuté
-  **AlreadyRegistered**: Identita nového používateľa už existuje
-  **NotMatchingVote**: Používateľ chce na referendum hlasovať znovu s opozíciou proti jeho pôvodnému hlasu (nedovolené striedať AYE a NAY na rovnakom referende)

#### Úložisko

```
//Returns proposal for specific id
pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, u32, Proposal<T>>;
```
Úložisko vráti objekt referenda so špecifickou ID.

```
//Returns user ids in vector for specific proposal
pub type ProposalVotes<T: Config> = StorageMap<_, Blake2_128Concat, u32, BoundedVec<T::AccountId, T::MaxVoters>>;
```
Úložisko vráti vektor účtov ktoré hlasovali na špecifické referendum.

```
//Returns id of last proposal
pub type LastProposalId<T: Config> = StorageValue<_, u32>;
```
Úložisko ktoré informuje o poslednej ID referenda.

```
//Map accountid onto user_vote structure
pub type UserVotes<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<UserVote, T::MaxVotes>>;
```
Úložisko ktoré vráti hlasy ktoré účet použil.

#### Riešenie funkcie "create_proposal"
Ak sa ti nepodarilo vyriešiť funkciu tak si pozri riadok s ktorým sa pasuješ a potom sa vráť a skús zvyšok dokončiť sám.

```
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
			let current_block = frame_system::Pallet::<T>::block_number();

			//Get max future block configurable max future block
			let max_block = current_block + T::MaxFutureBlock::get().into();

			//Check if block is not too far in the future
			ensure!(end_block < max_block.into(), Error::<T>::TooFarInTheFuture);

			//Query latest proposal id
			let id = LastProposalId::<T>::get().unwrap_or(0);

			// Bounded vector for proposals and change storage map to storage value
			Proposals::<T>::insert(
				id + 1,
				Proposal {
					id: id + 1,
					text,
					vote_count_aye: 0u32.into(),
					vote_count_nay: 0u32.into(),
					end_block,
					status: ProposalStatus::InProgress,
				},
			);

			//Update last proposal id
			LastProposalId::<T>::put(id + 1);

			//Emit event proposal created
			Self::deposit_event(Event::ProposalCreated { who, proposal_id: id + 1 });

			Ok(())
		}
```
