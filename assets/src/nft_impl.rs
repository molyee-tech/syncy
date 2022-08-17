use frame_support::traits::tokens::nonfungibles::{self, *};
use frame_support::storage::{StorageNMap, StorageValue, StorageMap, StorageDoubleMap};
use frame_support::pallet_prelude::*;
use sp_runtime::traits::{Hash, AtLeast32BitUnsigned, One, Zero, CheckedAdd, Saturating, CheckedSub};
use codec::{Encode, Decode};
use scale_info::TypeInfo;

use crate::{error::Error, FTImplT, Seal};


pub trait NFTImplT
{
    type Fungibles: FTImplT<
        Account=Self::Account,
        FTokenId=Self::FTokenId,
        FTokenAmount=Self::FractionAmount
    >;

    type Fingerprint: Copy + Parameter + 'static;
    type CollectionId: From<sp_core::H160> + Copy + Parameter;

    type Hasher: Hash<Output = Self::Fingerprint>;

    type InternalCollectionId: AtLeast32BitUnsigned + Copy + Parameter;
    type ItemId: AtLeast32BitUnsigned + Copy + Parameter + 'static;
    type FTokenId: AtLeast32BitUnsigned + Copy + Parameter;

    type FractionAmount: AtLeast32BitUnsigned + Copy + Parameter;

    type Account: Clone + Parameter + 'static;

    type Fractional: FractionalT<Self> + Copy + Parameter; // (Self::FTokenId, Self::FractionAmount)

    type CollectionRecord: CollectionRecordT<Self> + Parameter;
    type ItemRecord: ItemRecordT<Self> + Parameter;
    type FractionRecord: FractionRecordT<Self> + Parameter;

    type CollectionRepo: StorageMap<
        Self::CollectionId,
        Self::CollectionRecord
    >;

    type ItemRepo: StorageMap<
        Self::Fingerprint,
        Self::ItemRecord
    >;

    /// Storage with item id - fingerprint mapping.
    type FingerprintByFractionTokenId: StorageMap<Self::FTokenId, Self::Fingerprint>;

    type FractionRepo: StorageDoubleMap<
        Self::Fingerprint,
        Self::Account,
        Self::FractionRecord
    >;

    type FractionalRepo: StorageMap<
        Self::Fingerprint,
        Self::Fractional
    >;

    type FractionHolderId: From<sp_core::H160> +  Copy + Parameter + 'static;
    type FractionHoldGuard: AtLeast32BitUnsigned + Copy + Parameter + 'static;
    type FractionHolds: StorageNMap<
        (
            NMapKey<Blake2_128Concat, Self::Fingerprint>,
            NMapKey<Blake2_128Concat, Self::Account>,
            NMapKey<Blake2_128Concat, Self::FractionHolderId>,
            NMapKey<Blake2_128Concat, Self::FractionHoldGuard>,
        ),
        (Self::FractionHolderId, Self::FractionHoldGuard)
    >;

    type NextCollectionId: StorageValue<Self::InternalCollectionId>;

    type Nonfungibles:
        nonfungibles::Inspect<
            Self::Account,
            ClassId = Self::InternalCollectionId,
            InstanceId = Self::ItemId,
        > +
        nonfungibles::Transfer<Self::Account> +
        nonfungibles::Create<Self::Account> +
        nonfungibles::Mutate<Self::Account>;

    type Error: Error + Into<DispatchError>;

    fn get_fingerprint_by_fraction_token_id(ft_id: &Self::FTokenId) -> Result<Self::Fingerprint, Self::Error> {
        Self::FingerprintByFractionTokenId::try_get(&ft_id)
            .map_err(|_| Self::Error::unknown_item())
    }

    fn _update_fractions_amount(
        mut item: Self::ItemRecord,
        ft_id: Self::FTokenId,
        new_amount: Self::FractionAmount,
        seal: Seal,
    ) {
        let fingerprint = *item.fingerprint();
        let admin = item.account();

        let new_item_fractional_part = Self::Fractional::new(ft_id, new_amount);

        let fraction = Self::FractionRecord::new(
            admin,
            fingerprint,
            new_item_fractional_part,
            new_amount,
            Self::FractionHoldGuard::zero(),
        );
        
        Self::_insert_fractional(&fingerprint, &new_item_fractional_part, seal);

        Self::_insert_fraction(fraction, seal);

        item.set_fractional(new_item_fractional_part);

        Self::_insert_item(item, seal);
    }

    fn _obtain_collection_id(_: Seal) -> Option<Self::InternalCollectionId> {
        let id = Self::NextCollectionId::try_get()
            .unwrap_or_else(|_| Self::InternalCollectionId::zero());
        Self::NextCollectionId::put(id.checked_add(&Self::InternalCollectionId::one())?);
        Some(id)
    }

    fn find_collection(
        id: Self::CollectionId,
        _: Seal
    ) -> Option<Self::CollectionRecord>
    {
        Self::CollectionRepo::try_get(id).ok()
    }

    fn find_item(
        fingerprint: Self::Fingerprint,
        _: Seal
    ) -> Option<Self::ItemRecord>
    {
        Self::ItemRepo::try_get(fingerprint).ok()
    }

    fn find_fraction(
        fingerprint: Self::Fingerprint,
        account: &Self::Account,
        _: Seal
    ) -> Option<Self::FractionRecord>
    {
        Self::FractionRepo::try_get(fingerprint, account).ok()
    }

    fn find_fractional(
        fingerprint: Self::Fingerprint,
        _: Seal
    ) -> Option<Self::Fractional>
    {
        Self::FractionalRepo::try_get(fingerprint).ok()
    }

    fn _insert_collection(
        collection: Self::CollectionRecord,
        _: Seal
    ) {
        Self::CollectionRepo::insert(
            *collection.collection_id(),
            collection
        );
    }

    fn _insert_item(
        item: Self::ItemRecord,
        _: Seal
    ) {
        Self::ItemRepo::insert(
            *item.fingerprint(),
            item
        );
    }

    fn _insert_fraction(
        fraction: Self::FractionRecord,
        _: Seal
    ) {
        Self::FractionRepo::insert(
            *fraction.fingerprint(),
            fraction.account().clone(),
            fraction
        );
    }

    fn _insert_fractional(fingerprint: &Self::Fingerprint, fractional: &Self::Fractional, _: Seal) {
        Self::FractionalRepo::insert(fingerprint, fractional);
    }

    fn _remove_fraction(
        fraction: &Self::FractionRecord,
        _: Seal
    ) {
        Self::FractionRepo::remove(
            fraction.fingerprint(),
            fraction.account()
        );
    }

    fn _remove_fractional(fraction: &Self::FractionRecord, _: Seal) {
        Self::FractionalRepo::remove(fraction.fingerprint());
    }

    fn _fraction_hold_key(
        fraction: &Self::FractionRecord,
        holder_id: Self::FractionHolderId,
        guard: Self::FractionHoldGuard,
        _: Seal
    ) -> (Self::Fingerprint,
          Self::Account,
          Self::FractionHolderId,
          Self::FractionHoldGuard)
    {
        (
            *fraction.fingerprint(),
            fraction.account().clone(),
            holder_id,
            guard
        )
    }

    fn create_collection(
        account: &Self::Account,
        id: Self::CollectionId,
        max_items: Self::ItemId,
        _: Seal
    ) -> Result<(), DispatchError>
    {
        ensure!(!max_items.is_zero(), Self::Error::bad_value());

        let internal_id = Self::_obtain_collection_id(Seal(()))
            .ok_or_else(|| Self::Error::unknown_collection().into())?;

        Self::Nonfungibles::create_class(
            &internal_id,
            account,
            account
        )?;

        let collection = Self::CollectionRecord::new(
            account,
            id,
            internal_id,
            max_items,
            Self::ItemId::zero(),
        );

        Self::_insert_collection(collection, Seal(()));

        Ok(())
    }

    fn mint_item(
        mut collection: Self::CollectionRecord,
        fingerprint: Self::Fingerprint,
        _: Seal
    ) -> Result<(), ()>
    {
        if Self::ItemRepo::contains_key(fingerprint) { return Err(()) }

        let id = collection.obtain_item_id().ok_or(())?;

        let item = Self::ItemRecord::new(
            collection.account(),
            fingerprint,
            id,
            *collection.internal_id(),
            None
        );

        Self::_insert_collection(collection, Seal(()));

        Self::Nonfungibles::mint_into(
            item.collection_id(),
            item.item_id(),
            item.account()
        ).map_err(|_| ())?;

        Self::_insert_item(item, Seal(()));

        Ok(())
    }

    fn fractionalize(
        mut item: Self::ItemRecord,
        total: Self::FractionAmount,
        limited: bool,
        _: Seal
    ) -> DispatchResult {
        ensure!(!total.is_zero(), Self::Error::bad_value());

        ensure!(!item.is_fractional(), Self::Error::no_permission());

        let minimum_balance = One::one();

        let account = item.account().clone();
        let fingerprint = *item.fingerprint();

        let ft_id = Self::Fungibles::create_ft(
            account.clone(),
            minimum_balance,
            Seal(())
        )?;

        // mint_fraction checks if fractional part of the item is set.
        // So it needs to be initialized beforehand.
        item.set_fractional(Self::Fractional::new(ft_id, Self::FractionAmount::zero()));

        Self::mint_fraction(item, &account, total, Seal(()))?;

        if limited {
            Self::Fungibles::lock_minting(ft_id, &account, Seal(()))?;
        }

        Self::FingerprintByFractionTokenId::insert(ft_id, fingerprint);

        Ok(())
    }

    fn mint_fraction(
        item: Self::ItemRecord,
        who: &Self::Account,
        amount: Self::FractionAmount,
        seal: Seal,
    ) -> DispatchResult {
        ensure!(!amount.is_zero(), Self::Error::bad_value());

        let fractional = item.fractional().ok_or_else(|| Self::Error::not_fractionalized().into())?;
        let ft_id = *fractional.ft_id();

        ensure!(Self::Fungibles::can_mint(ft_id, who, seal), Self::Error::no_permission());

        Self::Fungibles::mint_ft(ft_id, who, amount, seal)?;

        let total_amount = *fractional.total() + amount;

        Self::_update_fractions_amount(item, ft_id, total_amount, seal);

        Ok(())
    }

    fn burn_fraction(
        item: Self::ItemRecord,
        who: &Self::Account,
        amount: Self::FractionAmount,
        seal: Seal,
    ) -> Result<Self::FractionAmount, DispatchError> {
        ensure!(item.account() == who, Self::Error::wrong_owner());
        ensure!(!amount.is_zero(), Self::Error::bad_value());

        let item_fractional_part = item
            .fractional()
            .ok_or_else(|| Self::Error::not_fractionalized().into())?;
        let total_amount = *item_fractional_part.total();
        ensure!(amount <= total_amount, Self::Error::insufficient_balance());

        let ft_id = *item_fractional_part.ft_id();

        // @TODO belongs to FTImpl trait
        ensure!(Self::Fungibles::can_burn(ft_id, who, seal), Self::Error::no_permission());

        let withdrawn_amount = Self::Fungibles::burn_ft(ft_id, who, amount, seal)?;

        let after_burn_amount = total_amount
            .checked_sub(&withdrawn_amount)
            .ok_or_else(|| Self::Error::overflow().into())?;

        Self::_update_fractions_amount(item, ft_id, after_burn_amount, seal);

        Ok(withdrawn_amount)
    }

    fn transfer_collection(
        mut collection: Self::CollectionRecord,
        to: &Self::Account,
        _: Seal
    ) -> Result<(), ()>
    {
        collection.transfer_collection(to);
        Self::_insert_collection(collection, Seal(()));
        Ok(())
    }

    fn transfer_item(
        mut item: Self::ItemRecord,
        to: &Self::Account,
        _: Seal
    ) -> DispatchResult
    {
        ensure!(!item.is_fractional(), Self::Error::no_permission());

        Self::Nonfungibles::transfer(
            item.collection_id(),
            item.item_id(),
            to
        )?;

        // @TODO transfer_ownership AssetId

        item.transfer_item(to);

        Self::_insert_item(item, Seal(()));

        Ok(())
    }

    fn transfer_fraction(
        mut donor: Self::FractionRecord,
        to: &Self::Account,
        amount: Self::FractionAmount,
        _: Seal
    ) -> DispatchResult
    {
        ensure!(donor.account() != to, Self::Error::bad_target());

        ensure!(!amount.is_zero(), Self::Error::bad_value());

        ensure!(!donor.on_hold(), Self::Error::no_permission());

        ensure!(&amount <= donor.amount(), Self::Error::insufficient_balance());

        let maybe_fraction = Self::find_fraction(*donor.fingerprint(), to, Seal(()));

        let mut fraction = maybe_fraction.unwrap_or_else(|| {
            Self::FractionRecord::new(
                to,
                *donor.fingerprint(),
                *donor.fractional(),
                <Self::FractionAmount>::zero(),
                <Self::FractionHoldGuard>::zero(),
            )
        });

        ensure!(!fraction.on_hold(), Self::Error::no_permission());

        Self::Fungibles::transfer(
            *donor.fractional().ft_id(),
            donor.account(),
            to,
            amount,
            Seal(())
        )?;

        fraction.increase_amount(amount).map_err(|_| Self::Error::overflow().into())?;

        Self::_insert_fraction(fraction, Seal(()));

        donor.decrease_amount(amount).map_err(|_| Self::Error::overflow().into())?;

        if donor.amount().is_zero() {
            Self::_remove_fraction(&donor, Seal(()));
        } else {
            Self::_insert_fraction(donor, Seal(()));
        }

        Ok(())
    }

    fn hold_fraction(
        mut fraction: Self::FractionRecord,
        holder_id: Self::FractionHolderId,
        guard: Self::FractionHoldGuard,
        _: Seal
    ) -> DispatchResult
    {
        let key = Self::_fraction_hold_key(&fraction, holder_id, guard, Seal(()));

        ensure!(!Self::FractionHolds::contains_key(&key), Self::Error::no_permission());

        Self::FractionHolds::insert(key, (holder_id, guard));

        fraction.inc_holds().map_err(|_| Self::Error::overflow().into())?;

        Self::_insert_fraction(fraction, Seal(()));

        Ok(())
    }

    fn unhold_fraction(
        mut fraction: Self::FractionRecord,
        holder_id: Self::FractionHolderId,
        guard: Self::FractionHoldGuard,
        _: Seal
    ) -> DispatchResult
    {
        let key = Self::_fraction_hold_key(&fraction, holder_id, guard, Seal(()));

        ensure!(Self::FractionHolds::contains_key(&key), Self::Error::no_permission());

        Self::FractionHolds::remove(key);

        fraction.dec_holds().map_err(|_| Self::Error::overflow().into())?;

        Self::_insert_fraction(fraction, Seal(()));

        Ok(())
    }
}

//

pub trait CollectionRecordT<Impl: NFTImplT + ?Sized>: Sized
{
    fn account(&self) -> &Impl::Account;

    fn collection_id(&self) -> &Impl::CollectionId;

    fn internal_id(&self) -> &Impl::InternalCollectionId;

    fn max_items(&self) -> &Impl::ItemId;

    fn items(&self) -> &Impl::ItemId;

    fn new(
        account: &Impl::Account,
        collection_id: Impl::CollectionId,
        internal_id: Impl::InternalCollectionId,
        max_items: Impl::ItemId,
        items: Impl::ItemId,
    ) -> Self;

    fn _inc_items(&mut self);

    fn _mut_account(&mut self) -> &mut Impl::Account;

    fn obtain_item_id(&mut self) -> Option<Impl::ItemId>
    {
        if self.items() < self.max_items() {
            let id = *self.items();
            self._inc_items();
            return Some(id)
        }
        None
    }

    fn transfer_collection(&mut self, to: &Impl::Account) {
        *self._mut_account() = to.clone();
    }
}

//

pub trait ItemRecordT<Impl: NFTImplT + ?Sized>: Sized {
    fn account(&self) -> &Impl::Account;

    fn fingerprint(&self) -> &Impl::Fingerprint;

    fn item_id(&self) -> &Impl::ItemId;

    fn collection_id(&self) -> &Impl::InternalCollectionId;

    fn fractional(&self) -> Option<&Impl::Fractional>;

    fn new(
        account: &Impl::Account,
        fingerprint: Impl::Fingerprint,
        item_id: Impl::ItemId,
        collection_id: Impl::InternalCollectionId,
        fractional: Option<Impl::Fractional>,
    ) -> Self;

    fn is_fractional(&self) -> bool {
        self.fractional().is_some()
    }

    fn _mut_account(&mut self) -> &mut Impl::Account;

    fn _mut_fractional(&mut self) -> &mut Option<Impl::Fractional>;

    fn transfer_item(&mut self, to: &Impl::Account) {
        *self._mut_account() = to.clone();
    }

    fn set_fractional(&mut self, fractional: Impl::Fractional) {
        self._mut_fractional().replace(fractional);
    }

    fn fuse(&mut self) {
        *self._mut_fractional() = None;
    }
}

//

pub trait FractionalT<Impl: NFTImplT + ?Sized>: Sized {
    fn ft_id(&self) -> &Impl::FTokenId;

    fn total(&self) -> &Impl::FractionAmount;

    fn new(
        ft_id: Impl::FTokenId,
        total: Impl::FractionAmount
    ) -> Self;
}

impl<Impl: NFTImplT + ?Sized> FractionalT<Impl> for (Impl::FTokenId, Impl::FractionAmount)
{
    fn ft_id(&self) -> &Impl::FTokenId {
        &self.0
    }

    fn total(&self) -> &Impl::FractionAmount {
        &self.1
    }

    fn new(
        ft_id: Impl::FTokenId,
        total: Impl::FractionAmount
    ) -> Self
    {
        (ft_id, total)
    }
}

//

pub trait FractionRecordT<Impl: NFTImplT + ?Sized>: Sized {
    fn account(&self) -> &Impl::Account;

    fn fingerprint(&self) -> &Impl::Fingerprint;

    fn fractional(&self) -> &Impl::Fractional;

    fn amount(&self) -> &Impl::FractionAmount;

    fn holds(&self) -> &Impl::FractionHoldGuard;

    fn new(
        account: &Impl::Account,
        fingerprint: Impl::Fingerprint,
        fractional: Impl::Fractional,
        amount: Impl::FractionAmount,
        holds: Impl::FractionHoldGuard
    ) -> Self;

    fn _mut_amount(&mut self) -> &mut Impl::FractionAmount;

    fn _mut_holds(&mut self) -> &mut Impl::FractionHoldGuard;

    fn can_fuse(&self) -> bool {
        self.amount() == self.fractional().total()
    }

    fn on_hold(&self) -> bool {
        !self.holds().is_zero()
    }

    fn increase_amount(&mut self, by: Impl::FractionAmount) -> Result<(), ()>
    {
        *self._mut_amount() = self.amount().checked_add(&by).ok_or(())?;
        Ok(())
    }

    fn decrease_amount(&mut self, by: Impl::FractionAmount) -> Result<(), ()>
    {
        *self._mut_amount() = self.amount().checked_sub(&by).ok_or(())?;
        Ok(())
    }

    fn inc_holds(&mut self) -> Result<(), ()> {
        *self._mut_holds() = self.holds().checked_add(&One::one()).ok_or(())?;
        Ok(())
    }

    fn dec_holds(&mut self) -> Result<(), ()> {
        *self._mut_holds() = self.holds().checked_sub(&One::one()).ok_or(())?;
        Ok(())
    }
}

//

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub struct NFTokenCollectionRecord<Account, CollectionId, InternalId, ItemId> {
    pub account: Account,
    pub collection_id: CollectionId,
    pub internal_id: InternalId,
    pub max_items: ItemId,
    pub items: ItemId,
}

impl<Impl: NFTImplT + ?Sized> CollectionRecordT<Impl> for
    NFTokenCollectionRecord<
        Impl::Account,
        Impl::CollectionId,
        Impl::InternalCollectionId,
        Impl::ItemId
    >
{
    fn account(&self) -> &Impl::Account {
        &self.account
    }

    fn collection_id(&self) -> &Impl::CollectionId {
        &self.collection_id
    }

    fn internal_id(&self) -> &Impl::InternalCollectionId {
        &self.internal_id
    }

    fn max_items(&self) -> &Impl::ItemId {
        &self.max_items
    }

    fn items(&self) -> &Impl::ItemId {
        &self.items
    }

    fn new(
        account: &Impl::Account,
        collection_id: Impl::CollectionId,
        internal_id: Impl::InternalCollectionId,
        max_items: Impl::ItemId,
        items: Impl::ItemId
    ) -> Self
    {
        Self {
            account: account.clone(),
            collection_id,
            internal_id,
            max_items,
            items
        }
    }

    fn _inc_items(&mut self) {
        self.items.saturating_inc();
    }

    fn _mut_account(&mut self) -> &mut Impl::Account {
        &mut self.account
    }
}

//

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub struct NFTokenItemRecord<Account, Fingerprint, ItemId, CollectionId, Fractional> {
    pub account: Account,
    pub fingerprint: Fingerprint,
    pub item_id: ItemId,
    pub collection_id: CollectionId,
    pub fractional: Option<Fractional>,
}

impl<Impl: NFTImplT + ?Sized> ItemRecordT<Impl> for
    NFTokenItemRecord<
        Impl::Account,
        Impl::Fingerprint,
        Impl::ItemId,
        Impl::InternalCollectionId,
        Impl::Fractional
    >
{
    fn account(&self) -> &Impl::Account {
        &self.account
    }

    fn fingerprint(&self) -> &Impl::Fingerprint {
        &self.fingerprint
    }

    fn item_id(&self) -> &Impl::ItemId {
        &self.item_id
    }

    fn collection_id(&self) -> &Impl::InternalCollectionId {
        &self.collection_id
    }

    fn fractional(&self) -> Option<&Impl::Fractional> {
        self.fractional.as_ref()
    }

    fn new(
        account: &Impl::Account,
        fingerprint: Impl::Fingerprint,
        item_id: Impl::ItemId,
        collection_id: Impl::InternalCollectionId,
        fractional: Option<Impl::Fractional>
    ) -> Self
    {
        Self {
            account: account.clone(),
            fingerprint,
            item_id,
            collection_id,
            fractional
        }
    }

    fn _mut_account(&mut self) -> &mut Impl::Account {
        &mut self.account
    }

    fn _mut_fractional(&mut self) -> &mut Option<Impl::Fractional> {
        &mut self.fractional
    }
}

//

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub struct NFTokenFractionRecord<Account, Fingerprint, Fractional, Amount, HoldGuard> {
    account: Account,
    fingerprint: Fingerprint,
    fractional: Fractional,
    amount: Amount,
    holds: HoldGuard,
}

impl<Impl: NFTImplT + ?Sized> FractionRecordT<Impl>
    for NFTokenFractionRecord<
        Impl::Account,
        Impl::Fingerprint,
        Impl::Fractional,
        Impl::FractionAmount,
        Impl::FractionHoldGuard
    >
{
    fn account(&self) -> &Impl::Account {
        &self.account
    }

    fn fingerprint(&self) -> &Impl::Fingerprint {
        &self.fingerprint
    }

    fn fractional(&self) -> &Impl::Fractional {
        &self.fractional
    }

    fn amount(&self) -> &Impl::FractionAmount {
        &self.amount
    }

    fn holds(&self) -> &Impl::FractionHoldGuard {
        &self.holds
    }

    fn new(
        account: &Impl::Account,
        fingerprint: Impl::Fingerprint,
        fractional: Impl::Fractional,
        amount: Impl::FractionAmount,
        holds: Impl::FractionHoldGuard
    ) -> Self
    {
        Self {
            account: account.clone(),
            fingerprint,
            fractional,
            amount,
            holds
        }
    }

    fn _mut_amount(&mut self) -> &mut Impl::FractionAmount {
        &mut self.amount
    }

    fn _mut_holds(&mut self) -> &mut Impl::FractionHoldGuard {
        &mut self.holds
    }
}
