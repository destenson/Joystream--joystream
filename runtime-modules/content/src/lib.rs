// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
// Internal Substrate warning (decl_event).
#![allow(clippy::unused_unit)]

#[cfg(test)]
mod tests;

mod errors;
mod nft;
mod permissions;
mod types;

pub use errors::*;
pub use nft::*;
pub use permissions::*;
pub use types::*;

use core::hash::Hash;

use codec::Codec;
use codec::{Decode, Encode};

use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, traits::Get, Parameter,
};
use frame_system::ensure_signed;
#[cfg(feature = "std")]
pub use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::{BaseArithmetic, One, Zero};
use sp_runtime::traits::{MaybeSerializeDeserialize, Member};
pub use sp_runtime::Perbill;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::vec;
use sp_std::vec::Vec;

pub use common::storage::{
    ContentParameters as ContentParametersRecord, StorageObjectOwner as StorageObjectOwnerRecord,
    StorageSystem,
};

pub use common::{
    currency::{BalanceOf, GovernanceCurrency},
    working_group::WorkingGroup,
    MembershipTypes, StorageOwnership, Url,
};
use frame_support::traits::{Currency, ReservableCurrency};

/// A numeric identifier trait
pub trait NumericIdentifier:
    Parameter
    + Member
    + BaseArithmetic
    + Codec
    + Default
    + Copy
    + Clone
    + Hash
    + MaybeSerializeDeserialize
    + Eq
    + PartialEq
    + Ord
    + Zero
{
}

impl NumericIdentifier for u64 {}

/// Module configuration trait for Content Directory Module
pub trait Trait:
    membership::Trait + ContentActorAuthenticator + Clone + StorageOwnership + GovernanceCurrency
{
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Channel Transfer Payments Escrow Account seed for ModuleId to compute deterministic AccountId
    type ChannelOwnershipPaymentEscrowId: Get<[u8; 8]>;

    /// Type of identifier for Videos
    type VideoId: NumericIdentifier;

    /// Type of identifier for Video Categories
    type VideoCategoryId: NumericIdentifier;

    /// Type of identifier for Channel Categories
    type ChannelCategoryId: NumericIdentifier;

    /// Type of identifier for Playlists
    type PlaylistId: NumericIdentifier;

    /// Type of identifier for Persons
    type PersonId: NumericIdentifier;

    /// Type of identifier for Channels
    type SeriesId: NumericIdentifier;

    /// Type of identifier for Channel transfer requests
    type ChannelOwnershipTransferRequestId: NumericIdentifier;

    /// The maximum number of curators per group constraint
    type MaxNumberOfCuratorsPerGroup: Get<MaxNumber>;

    // Type that handles asset uploads to storage frame_system
    type StorageSystem: StorageSystem<Self, Self::MemberId>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Content {
        pub ChannelById get(fn channel_by_id): map hasher(blake2_128_concat) T::ChannelId => Channel<T>;

        pub ChannelCategoryById get(fn channel_category_by_id): map hasher(blake2_128_concat) T::ChannelCategoryId => ChannelCategory;

        pub VideoById get(fn video_by_id): map hasher(blake2_128_concat) T::VideoId => Video<T>;

        pub VideoCategoryById get(fn video_category_by_id): map hasher(blake2_128_concat) T::VideoCategoryId => VideoCategory;

        pub PlaylistById get(fn playlist_by_id): map hasher(blake2_128_concat) T::PlaylistId => Playlist<T::ChannelId>;

        pub SeriesById get(fn series_by_id): map hasher(blake2_128_concat) T::SeriesId => Series<T::ChannelId, T::VideoId>;

        pub PersonById get(fn person_by_id): map hasher(blake2_128_concat) T::PersonId => Person<T::MemberId>;

        pub ChannelOwnershipTransferRequestById get(fn channel_ownership_transfer_request_by_id):
            map hasher(blake2_128_concat) T::ChannelOwnershipTransferRequestId => ChannelOwnershipTransferRequest<T>;

        pub NextChannelCategoryId get(fn next_channel_category_id) config(): T::ChannelCategoryId;

        pub NextChannelId get(fn next_channel_id) config(): T::ChannelId;

        pub NextVideoCategoryId get(fn next_video_category_id) config(): T::VideoCategoryId;

        pub NextVideoId get(fn next_video_id) config(): T::VideoId;

        pub NextPlaylistId get(fn next_playlist_id) config(): T::PlaylistId;

        pub NextPersonId get(fn next_person_id) config(): T::PersonId;

        pub NextSeriesId get(fn next_series_id) config(): T::SeriesId;

        pub NextChannelOwnershipTransferRequestId get(fn next_channel_transfer_request_id) config(): T::ChannelOwnershipTransferRequestId;

        pub NextCuratorGroupId get(fn next_curator_group_id) config(): T::CuratorGroupId;

        /// Map, representing  CuratorGroupId -> CuratorGroup relation
        pub CuratorGroupById get(fn curator_group_by_id): map hasher(blake2_128_concat) T::CuratorGroupId => CuratorGroup<T>;

        /// Min auction round time
        pub MinRoundTime get(fn min_round_duration) config(): T::BlockNumber;

        /// Max auction round time
        pub MaxRoundTime get(fn max_round_duration) config(): T::BlockNumber;

        /// Min bid lock duration
        pub MinBidLockDuration get(fn min_bid_lock_duration) config(): T::BlockNumber;

        /// Max bid lock duration
        pub MaxBidLockDuration get(fn max_bid_lock_duration) config(): T::BlockNumber;

        /// Min auction staring price
        pub MinStartingPrice get(fn min_starting_price) config(): BalanceOf<T>;

        /// Max auction staring price
        pub MaxStartingPrice get(fn max_starting_price) config(): BalanceOf<T>;

        /// Min creator royalty percentage
        pub MinCreatorRoyalty get(fn min_creator_royalty) config(): Perbill;

        /// Max creator royalty percentage
        pub MaxCreatorRoyalty get(fn max_creator_royalty) config(): Perbill;

        /// Min auction bid step
        pub MinBidStep get(fn min_bid_step) config(): BalanceOf<T>;

        /// Max auction bid step
        pub MaxBidStep get(fn max_bid_step) config(): BalanceOf<T>;

        /// Auction platform fee percentage
        pub AuctionFeePercentage get(fn auction_fee_percentage) config(): Perbill;

        /// Max delta between current block and starts at
        pub AuctionStartsAtMaxDelta get(fn auction_starts_at_max_delta) config(): T::BlockNumber;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Predefined errors
        type Error = Error<T>;

        /// Initializing events
        fn deposit_event() = default;

        /// Exports const -  max number of curators per group
        const MaxNumberOfCuratorsPerGroup: MaxNumber = T::MaxNumberOfCuratorsPerGroup::get();

        // ======
        // Next set of extrinsics can only be invoked by lead.
        // ======

        /// Add new curator group to runtime storage
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_curator_group(
            origin,
        ) {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            //
            // == MUTATION SAFE ==
            //

            let curator_group_id = Self::next_curator_group_id();

            // Insert empty curator group with `active` parameter set to false
            <CuratorGroupById<T>>::insert(curator_group_id, CuratorGroup::<T>::default());

            // Increment the next curator curator_group_id:
            <NextCuratorGroupId<T>>::mutate(|n| *n += T::CuratorGroupId::one());

            // Trigger event
            Self::deposit_event(RawEvent::CuratorGroupCreated(curator_group_id));
        }

        /// Set `is_active` status for curator group under given `curator_group_id`
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_curator_group_status(
            origin,
            curator_group_id: T::CuratorGroupId,
            is_active: bool,
        ) {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure curator group under provided curator_group_id already exist
            Self::ensure_curator_group_under_given_id_exists(&curator_group_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Set `is_active` status for curator group under given `curator_group_id`
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.set_status(is_active)
            });

            // Trigger event
            Self::deposit_event(RawEvent::CuratorGroupStatusSet(curator_group_id, is_active));
        }

        /// Add curator to curator group under given `curator_group_id`
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn add_curator_to_group(
            origin,
            curator_group_id: T::CuratorGroupId,
            curator_id: T::CuratorId,
        ) {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure curator group under provided curator_group_id already exist, retrieve corresponding one
            let curator_group = Self::ensure_curator_group_exists(&curator_group_id)?;

            // Ensure that curator_id is infact a worker in content working group
            ensure_is_valid_curator_id::<T>(&curator_id)?;

            // Ensure max number of curators per group limit not reached yet
            curator_group.ensure_max_number_of_curators_limit_not_reached()?;

            // Ensure curator under provided curator_id isn`t a CuratorGroup member yet
            curator_group.ensure_curator_in_group_does_not_exist(&curator_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Insert curator_id into curator_group under given curator_group_id
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.get_curators_mut().insert(curator_id);
            });

            // Trigger event
            Self::deposit_event(RawEvent::CuratorAdded(curator_group_id, curator_id));
        }

        /// Remove curator from a given curator group
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn remove_curator_from_group(
            origin,
            curator_group_id: T::CuratorGroupId,
            curator_id: T::CuratorId,
        ) {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure curator group under provided curator_group_id already exist, retrieve corresponding one
            let curator_group = Self::ensure_curator_group_exists(&curator_group_id)?;

            // Ensure curator under provided curator_id is CuratorGroup member
            curator_group.ensure_curator_in_group_exists(&curator_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Remove curator_id from curator_group under given curator_group_id
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.get_curators_mut().remove(&curator_id);
            });

            // Trigger event
            Self::deposit_event(RawEvent::CuratorRemoved(curator_group_id, curator_id));
        }

        // TODO: Add Option<reward_account> to ChannelCreationParameters ?
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_channel(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            params: ChannelCreationParameters<ContentParameters<T>, T::AccountId>,
        ) {
            ensure_actor_authorized_to_create_channel::<T>(
                origin,
                &actor,
            )?;

            // The channel owner will be..
            let channel_owner = Self::actor_to_content_owner(&actor)?;

            // Pick out the assets to be uploaded to storage frame_system
            let content_parameters: Vec<ContentParameters<T>> = Self::pick_content_parameters_from_assets(&params.assets);

            let channel_id = NextChannelId::<T>::get();

            let object_owner = StorageObjectOwner::<T>::Channel(channel_id);

            //
            // == MUTATION SAFE ==
            //

            // This should be first mutation
            // Try add assets to storage
            T::StorageSystem::atomically_add_content(
                object_owner,
                content_parameters,
            )?;

            // Only increment next channel id if adding content was successful
            NextChannelId::<T>::mutate(|id| *id += T::ChannelId::one());

            let channel: Channel<T> = ChannelRecord {
                owner: channel_owner,
                videos: vec![],
                playlists: vec![],
                series: vec![],
                is_censored: false,
                reward_account: params.reward_account.clone(),
            };
            ChannelById::<T>::insert(channel_id, channel.clone());

            Self::deposit_event(RawEvent::ChannelCreated(actor, channel_id, channel, params));
        }

        // Include Option<AccountId> in ChannelUpdateParameters to update reward_account
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_channel(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            channel_id: T::ChannelId,
            params: ChannelUpdateParameters<ContentParameters<T>, T::AccountId>,
        ) {
            // check that channel exists
            let channel = Self::ensure_channel_exists(&channel_id)?;

            ensure_actor_authorized_to_update_channel::<T>(
                origin,
                &actor,
                &channel.owner,
            )?;

            // Pick out the assets to be uploaded to storage frame_system
            let new_assets = if let Some(assets) = &params.assets {
                let upload_parameters: Vec<ContentParameters<T>> = Self::pick_content_parameters_from_assets(assets);

                let object_owner = StorageObjectOwner::<T>::Channel(channel_id);

                // check assets can be uploaded to storage.
                // update can_add_content() to only take &refrences
                T::StorageSystem::can_add_content(
                    object_owner.clone(),
                    upload_parameters.clone(),
                )?;

                Some((upload_parameters, object_owner))
            } else {
                None
            };

            //
            // == MUTATION SAFE ==
            //

            let mut channel = channel;

            // Maybe update the reward account
            if let Some(reward_account) = &params.reward_account {
                channel.reward_account = reward_account.clone();
            }

            // Update the channel
            ChannelById::<T>::insert(channel_id, channel.clone());

            // add assets to storage
            // This should not fail because of prior can_add_content() check!
            if let Some((upload_parameters, object_owner)) = new_assets {
                T::StorageSystem::atomically_add_content(
                    object_owner,
                    upload_parameters,
                )?;
            }

            Self::deposit_event(RawEvent::ChannelUpdated(actor, channel_id, channel, params));
        }

        /// Remove assets of a channel from storage
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn remove_channel_assets(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            channel_id: T::ChannelId,
            assets: Vec<ContentId<T>>,
        ) {
            // check that channel exists
            let channel = Self::ensure_channel_exists(&channel_id)?;

            ensure_actor_authorized_to_update_channel::<T>(
                origin,
                &actor,
                &channel.owner,
            )?;

            let object_owner = StorageObjectOwner::<T>::Channel(channel_id);

            //
            // == MUTATION SAFE ==
            //

            T::StorageSystem::atomically_remove_content(&object_owner, &assets)?;

            Self::deposit_event(RawEvent::ChannelAssetsRemoved(actor, channel_id, assets));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_channel_censorship_status(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            channel_id: T::ChannelId,
            is_censored: bool,
            rationale: Vec<u8>,
        ) {
            // check that channel exists
            let channel = Self::ensure_channel_exists(&channel_id)?;

            ensure_actor_authorized_to_censor::<T>(
                origin,
                &actor,
                &channel.owner,
            )?;

            // Ensure censorship status have been changed
            channel.ensure_censorship_status_changed::<T>(is_censored)?;

            //
            // == MUTATION SAFE ==
            //

            let mut channel = channel;

            channel.is_censored = is_censored;

            // TODO: unset the reward account ? so no revenue can be earned for censored channels?

            // Update the channel
            ChannelById::<T>::insert(channel_id, channel);

            Self::deposit_event(RawEvent::ChannelCensorshipStatusUpdated(actor, channel_id, is_censored, rationale));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_channel_category(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            params: ChannelCategoryCreationParameters,
        ) {
            ensure_actor_authorized_to_manage_categories::<T>(
                origin,
                &actor
            )?;

            //
            // == MUTATION SAFE ==
            //

            let category_id = Self::next_channel_category_id();
            NextChannelCategoryId::<T>::mutate(|id| *id += T::ChannelCategoryId::one());

            let category = ChannelCategory {};
            ChannelCategoryById::<T>::insert(category_id, category.clone());

            Self::deposit_event(RawEvent::ChannelCategoryCreated(category_id, category, params));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_channel_category(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            category_id: T::ChannelCategoryId,
            params: ChannelCategoryUpdateParameters,
        ) {
            ensure_actor_authorized_to_manage_categories::<T>(
                origin,
                &actor
            )?;

            Self::ensure_channel_category_exists(&category_id)?;

            Self::deposit_event(RawEvent::ChannelCategoryUpdated(actor, category_id, params));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn delete_channel_category(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            category_id: T::ChannelCategoryId,
        ) {
            ensure_actor_authorized_to_manage_categories::<T>(
                origin,
                &actor
            )?;

            Self::ensure_channel_category_exists(&category_id)?;

            ChannelCategoryById::<T>::remove(&category_id);

            Self::deposit_event(RawEvent::ChannelCategoryDeleted(actor, category_id));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn request_channel_transfer(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _request: ChannelOwnershipTransferRequest<T>,
        ) {
            // requester must be new_owner
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_channel_transfer_request(
            _origin,
            _request_id: T::ChannelOwnershipTransferRequestId,
        ) {
            // origin must be original requester (ie. proposed new channel owner)
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn accept_channel_transfer(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _request_id: T::ChannelOwnershipTransferRequestId,
        ) {
            // only current owner of channel can approve
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_video(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            channel_id: T::ChannelId,
            params: VideoCreationParameters<ContentParameters<T>>,
        ) {
            // check that channel exists
            let channel = Self::ensure_channel_exists(&channel_id)?;

            ensure_actor_authorized_to_update_channel::<T>(
                origin,
                &actor,
                &channel.owner,
            )?;

            // Pick out the assets to be uploaded to storage frame_system
            let content_parameters: Vec<ContentParameters<T>> = Self::pick_content_parameters_from_assets(&params.assets);

            let video_id = NextVideoId::<T>::get();

            let object_owner = StorageObjectOwner::<T>::Channel(channel_id);

            // This should be first mutation
            // Try add assets to storage
            T::StorageSystem::atomically_add_content(
                object_owner,
                content_parameters,
            )?;

            //
            // == MUTATION SAFE ==
            //

            let video: Video<T> = VideoRecord {
                in_channel: channel_id,
                // keep track of which season the video is in if it is an 'episode'
                // - prevent removing a video if it is in a season (because order is important)
                in_series: None,
                /// Whether the curators have censored the video or not.
                is_censored: false,
                /// Newly created video has no nft
                nft_status: None,
            };

            VideoById::<T>::insert(video_id, video);

            // Only increment next video id if adding content was successful
            NextVideoId::<T>::mutate(|id| *id += T::VideoId::one());

            // Add recently added video id to the channel
            ChannelById::<T>::mutate(channel_id, |channel| {
                channel.videos.push(video_id);
            });

            Self::deposit_event(RawEvent::VideoCreated(actor, channel_id, video_id, params));

        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_video(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            video_id: T::VideoId,
            params: VideoUpdateParameters<ContentParameters<T>>,
        ) {
            // check that video exists, retrieve corresponding channel id.
            let channel_id = Self::ensure_video_exists(&video_id)?.in_channel;

            ensure_actor_authorized_to_update_channel::<T>(
                origin,
                &actor,
                &Self::channel_by_id(channel_id).owner,
            )?;

            // Pick out the assets to be uploaded to storage frame_system
            let new_assets = if let Some(assets) = &params.assets {
                let upload_parameters: Vec<ContentParameters<T>> = Self::pick_content_parameters_from_assets(assets);

                let object_owner = StorageObjectOwner::<T>::Channel(channel_id);

                // check assets can be uploaded to storage.
                // update can_add_content() to only take &refrences
                T::StorageSystem::can_add_content(
                    object_owner.clone(),
                    upload_parameters.clone(),
                )?;

                Some((upload_parameters, object_owner))
            } else {
                None
            };

            //
            // == MUTATION SAFE ==
            //

            // add assets to storage
            // This should not fail because of prior can_add_content() check!
            if let Some((upload_parameters, object_owner)) = new_assets {
                T::StorageSystem::atomically_add_content(
                    object_owner,
                    upload_parameters,
                )?;
            }

            Self::deposit_event(RawEvent::VideoUpdated(actor, video_id, params));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn delete_video(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            video_id: T::VideoId,
        ) {

            // check that video exists
            let video = Self::ensure_video_exists(&video_id)?;

            let channel_id = video.in_channel;

            ensure_actor_authorized_to_update_channel::<T>(
                origin,
                &actor,
                // The channel owner will be..
                &Self::channel_by_id(channel_id).owner,
            )?;

            Self::ensure_video_can_be_removed(video)?;

            //
            // == MUTATION SAFE ==
            //

            // Remove video
            VideoById::<T>::remove(video_id);

            // Update corresponding channel
            // Remove recently deleted video from the channel
            ChannelById::<T>::mutate(channel_id, |channel| {
                if let Some(index) = channel.videos.iter().position(|x| *x == video_id) {
                    channel.videos.remove(index);
                }
            });

            Self::deposit_event(RawEvent::VideoDeleted(actor, video_id));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_playlist(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _channel_id: T::ChannelId,
            _params: PlaylistCreationParameters,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_playlist(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _playlist: T::PlaylistId,
            _params: PlaylistUpdateParameters,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn delete_playlist(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _channel_id: T::ChannelId,
            _playlist: T::PlaylistId,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_featured_videos(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            list: Vec<T::VideoId>
        ) {
            // can only be set by lead
            ensure_actor_authorized_to_set_featured_videos::<T>(
                origin,
                &actor,
            )?;

            //
            // == MUTATION SAFE ==
            //

            Self::deposit_event(RawEvent::FeaturedVideosSet(actor, list));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_video_category(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            params: VideoCategoryCreationParameters,
        ) {
            ensure_actor_authorized_to_manage_categories::<T>(
                origin,
                &actor
            )?;

            //
            // == MUTATION SAFE ==
            //

            let category_id = Self::next_video_category_id();
            NextVideoCategoryId::<T>::mutate(|id| *id += T::VideoCategoryId::one());

            let category = VideoCategory {};
            VideoCategoryById::<T>::insert(category_id, category);

            Self::deposit_event(RawEvent::VideoCategoryCreated(actor, category_id, params));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_video_category(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            category_id: T::VideoCategoryId,
            params: VideoCategoryUpdateParameters,
        ) {
            ensure_actor_authorized_to_manage_categories::<T>(
                origin,
                &actor
            )?;

            Self::ensure_video_category_exists(&category_id)?;

            Self::deposit_event(RawEvent::VideoCategoryUpdated(actor, category_id, params));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn delete_video_category(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            category_id: T::VideoCategoryId,
        ) {
            ensure_actor_authorized_to_manage_categories::<T>(
                origin,
                &actor
            )?;

            Self::ensure_video_category_exists(&category_id)?;

            VideoCategoryById::<T>::remove(&category_id);

            Self::deposit_event(RawEvent::VideoCategoryDeleted(actor, category_id));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_person(
            _origin,
            _actor: PersonActor<T::MemberId, T::CuratorId>,
            _params: PersonCreationParameters<ContentParameters<T>>,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_person(
            _origin,
            _actor: PersonActor<T::MemberId, T::CuratorId>,
            _person: T::PersonId,
            _params: PersonUpdateParameters<ContentParameters<T>>,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn delete_person(
            _origin,
            _actor: PersonActor<T::MemberId, T::CuratorId>,
            _person: T::PersonId,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn add_person_to_video(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _video_id: T::VideoId,
            _person: T::PersonId
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn remove_person_from_video(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _video_id: T::VideoId
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_video_censorship_status(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            video_id: T::VideoId,
            is_censored: bool,
            rationale: Vec<u8>,
        ) {
            // check that video exists
            let video = Self::ensure_video_exists(&video_id)?;

            ensure_actor_authorized_to_censor::<T>(
                origin,
                &actor,
                // The channel owner will be..
                &Self::channel_by_id(video.in_channel).owner,
            )?;

            // Ensure censorship status have been changed
            video.ensure_censorship_status_changed::<T>(is_censored)?;

            //
            // == MUTATION SAFE ==
            //

            let mut video = video;

            video.is_censored = is_censored;

            // Update the video
            VideoById::<T>::insert(video_id, video);

            Self::deposit_event(RawEvent::VideoCensorshipStatusUpdated(actor, video_id, is_censored, rationale));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_series(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _channel_id: T::ChannelId,
            _params: SeriesParameters<T::VideoId, ContentParameters<T>>,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_series(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _channel_id: T::ChannelId,
            _params: SeriesParameters<T::VideoId, ContentParameters<T>>,
        ) {
            Self::not_implemented()?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn delete_series(
            _origin,
            _actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            _series: T::SeriesId,
        ) {
            Self::not_implemented()?;
        }

        /// Start video nft auction
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn start_nft_auction(
            origin,
            auctioneer: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            auction_params: AuctionParams<T::VideoId, T::BlockNumber, BalanceOf<T>, T::MemberId>,
        ) {

            let video_id = auction_params.video_id;

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure nft is already issued
            video.ensure_nft_is_issued::<T>()?;

            // Ensure there nft transactional status is set to idle.
            video.ensure_nft_transactional_status_is_idle::<T>()?;

            // Authorize nft owner
            Self::authorize_nft_owner(
                origin,
                &auctioneer,
                &video
            )?;

            // Validate round_duration & starting_price
            Self::validate_auction_params(&auction_params)?;

            //
            // == MUTATION SAFE ==
            //

            // Create new auction
            let auction = AuctionRecord::new(auction_params.clone());
            let video = video.set_auction_transactional_status(auction);

            // Update the video
            VideoById::<T>::insert(video_id, video);

            // Trigger event
            Self::deposit_event(RawEvent::AuctionStarted(auctioneer, auction_params));
        }

        /// Cancel video auction
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_video_auction(
            origin,
            auctioneer: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            video_id: T::VideoId,
        ) {

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Authorize nft owner
            Self::authorize_nft_owner(
                origin,
                &auctioneer,
                &video
            )?;

            // Ensure auction for given video id exists
            let auction = video.ensure_nft_auction_state::<T>()?;

            // Ensure nft auction not expired
            Self::ensure_nft_auction_not_expired(&auction)?;

            // Ensure given auction can be canceled
            auction.ensure_auction_can_be_canceled::<T>()?;

            let last_bid_data = if let Some(last_bid) = auction.last_bid {
                let last_bidder_account_id = Self::ensure_member_controller_account_id(last_bid.bidder)?;
                Some((last_bidder_account_id, last_bid.amount))
            } else {
                None
            };

            //
            // == MUTATION SAFE ==
            //

            // Unreserve previous bidder balance
            if let Some((last_bidder_account_id, last_bid_amount)) = last_bid_data {
                T::Currency::unreserve(&last_bidder_account_id, last_bid_amount);
            }

            // Cancel auction
            let video = video.set_idle_transactional_status();

            VideoById::<T>::insert(video_id, video);

            // Trigger event
            Self::deposit_event(RawEvent::AuctionCancelled(auctioneer, video_id));
        }

        /// Make auction bid
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn make_bid(
            origin,
            participant_id: T::MemberId,
            video_id: T::VideoId,
            bid: BalanceOf<T>,
        ) {

            // Authorize participant under given member id
            let participant_account_id = ensure_signed(origin)?;
            ensure_member_auth_success::<T>(&participant_id, &participant_account_id)?;

            // Ensure bidder have sufficient balance amount to reserve for bid
            Self::ensure_has_sufficient_balance(&participant_account_id, bid)?;

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure auction for given video id exists
            let auction = video.ensure_nft_auction_state::<T>()?;

            // Ensure nft auction not expired
            Self::ensure_nft_auction_not_expired(&auction)?;

            let current_block = <frame_system::Module<T>>::block_number();

            // Ensure auction have been already started
            auction.ensure_auction_started::<T>(current_block)?;

            // Ensure participant have been already added to whitelist if set
            auction.ensure_whitelisted_participant::<T>(participant_id)?;

            // Ensure new bid is greater then last bid + minimal bid step
            auction.ensure_is_valid_bid::<T>(bid)?;

            let last_bid_data = if let Some(last_bid) = auction.last_bid {
                let last_bidder_account_id = Self::ensure_member_controller_account_id(last_bid.bidder)?;
                Some((last_bidder_account_id, last_bid.amount))
            } else {
                None
            };

            //
            // == MUTATION SAFE ==
            //

            // Unreserve previous bidder balance
            if let Some((last_bidder_account_id, last_bid_amount)) = last_bid_data {
                T::Currency::unreserve(&last_bidder_account_id, last_bid_amount);
            }

            // Do not charge more then buy now
            let bid = match auction.buy_now_price {
                Some(buy_now_price) if bid >= buy_now_price => buy_now_price,
                _ => bid,
            };

            // Reseve balance for current bid
            // Can not fail, needed check made
            T::Currency::reserve(&participant_account_id, bid)?;

            // Make auction bid & update auction data
            let mut video = video;

            if let Some(auction) = video.get_nft_auction_ref_mut() {
                auction.make_bid(participant_id, bid, current_block);

                VideoById::<T>::insert(video_id, video);

                // Trigger event
                Self::deposit_event(RawEvent::AuctionBidMade(participant_id, video_id, bid));
            }
        }

        /// Cancel open auction bid
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_open_auction_bid(
            origin,
            participant_id: T::MemberId,
            video_id: T::VideoId,
        ) {

            // Authorize participant under given member id
            let participant_account_id = ensure_signed(origin)?;
            ensure_member_auth_success::<T>(&participant_id, &participant_account_id)?;

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure auction for given video id exists
            let auction = video.ensure_nft_auction_state::<T>()?;

            // Ensure nft auction not expired
            Self::ensure_nft_auction_not_expired(&auction)?;

            let current_block = <frame_system::Module<T>>::block_number();

            // Ensure auction have been already started
            auction.ensure_auction_started::<T>(current_block)?;

            // Ensure participant can cancel last bid
            auction.ensure_bid_can_be_canceled::<T>(participant_id, current_block)?;

            //
            // == MUTATION SAFE ==
            //

            // Cancel last auction bid & update auction data
            let mut video = video;

            if let Some(auction) = video.get_nft_auction_ref_mut() {
                auction.cancel_bid();

                VideoById::<T>::insert(video_id, video);

                // Trigger event
                Self::deposit_event(RawEvent::AuctionBidCanceled(participant_id, video_id));
            }
        }

        /// Complete auction
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn complete_video_auction(
            origin,
            member_id: T::MemberId,
            video_id: T::VideoId,
            metadata: Metadata,
        ) {

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure auction for given video id exists, retrieve corresponding one
            let auction = video.ensure_nft_auction_state::<T>()?;

            let last_bid = auction.ensure_last_bid_exists::<T>()?;

            // Ensure actor authorized to complete auction.
            Self::ensure_actor_is_last_bidder(origin, member_id, &auction)?;

            // Ensure auction can be completed
            Self::ensure_auction_can_be_completed(&auction)?;

            if let Some(owned_nft) = &video.nft_status {

                let owner_account_id = Self::ensure_owner_account_id(&video, &owned_nft)?;

                let last_bidder_account_id = Self::ensure_member_controller_account_id(last_bid.bidder)?;

                //
                // == MUTATION SAFE ==
                //

                let video = Self::complete_auction(video, last_bidder_account_id, owner_account_id);

                // Update the video
                VideoById::<T>::insert(video_id, video);
            }

            // Trigger event
            Self::deposit_event(RawEvent::AuctionCompleted(member_id, video_id, metadata));
        }

        /// Accept open auction bid
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn settle_open_auction(
            origin,
            actor: ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
            video_id: T::VideoId,
            metadata: Metadata,
        ) {

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure auction for given video id exists, retrieve corresponding one
            let auction = video.ensure_nft_auction_state::<T>()?;

            // Ensure open type auction
            auction.ensure_is_open_auction::<T>()?;

            // Ensure there is a bid to accept
            let last_bid = auction.ensure_last_bid_exists::<T>()?;

            // Ensure actor is authorized to accept open auction bid
            Self::authorize_nft_owner(origin, &actor, &video)?;

            if let Some(owned_nft) = &video.nft_status {

                let owner_account_id = Self::ensure_owner_account_id(&video, &owned_nft)?;

                let last_bidder_account_id = Self::ensure_member_controller_account_id(last_bid.bidder)?;

                //
                // == MUTATION SAFE ==
                //

                let video = Self::complete_auction(video, last_bidder_account_id, owner_account_id);

                // Update the video
                VideoById::<T>::insert(video_id, video);
            }

            // Trigger event
            Self::deposit_event(RawEvent::OpenAuctionBidAccepted(actor, video_id, metadata));
        }

        /// Issue NFT
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn issue_nft(
            origin,
            actor: ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
            video_id: T::VideoId,
            royalty: Option<Royalty>,
            metadata: Metadata,
            to: Option<T::MemberId>,
        ) {

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure have not been issued yet
            video.ensure_nft_is_not_issued::<T>()?;

            // Ensure channel exists, retrieve channel owner
            let channel_owner = Self::ensure_channel_exists(&video.in_channel)?.owner;

            ensure_actor_authorized_to_update_channel::<T>(origin, &actor, &channel_owner)?;

            // The content owner will be..
            let content_owner = if let Some(to) = to {
                ChannelOwner::Member(to)
            } else {
                // if `to` set to None, actor issues to himself
                Self::actor_to_content_owner(&actor)?
            };

            // Enure royalty bounds satisfied, if provided
            if let Some(royalty) = royalty {
                Self::ensure_reward_account_is_set(video.in_channel)?;
                Self::ensure_royalty_bounds_satisfied(royalty)?;
            }

            //
            // == MUTATION SAFE ==
            //

            // Issue NFT
            let mut video = video;
            video.nft_status = Some(OwnedNFT {
                transactional_status: TransactionalStatus::Idle,
                owner: content_owner.clone(),
                creator_royalty: royalty,
            });

            // Update the video
            VideoById::<T>::insert(video_id, video);

            Self::deposit_event(RawEvent::NftIssued(
                actor,
                video_id,
                royalty,
                metadata,
                content_owner,
            ));
        }

        /// Offer NFT
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn offer_nft(
            origin,
            video_id: T::VideoId,
            actor: ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
            to: MemberId<T>,
            price: Option<BalanceOf<T>>,
        ) {

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure there is no pending offer or existing auction for given nft.
            video.ensure_nft_transactional_status_is_idle::<T>()?;

            // Authorize nft owner
            Self::authorize_nft_owner(
                origin,
                &actor,
                &video
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Set nft transactional status to InitiatedOfferToMember
            let video = video.set_pending_offer_transactional_status(to, price);

            VideoById::<T>::insert(video_id, video);

            // Trigger event
            Self::deposit_event(RawEvent::OfferStarted(video_id, actor, to, price));
        }

        /// Cancel NFT offer
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_offer(
            origin,
            actor: ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
            video_id: T::VideoId,
        ) {

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure given pending offer exists
            video.ensure_pending_offer_exists::<T>()?;

            // Authorize nft owner
            Self::authorize_nft_owner(
                origin,
                &actor,
                &video
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Cancel pending offer
            let video = video.set_idle_transactional_status();

            VideoById::<T>::insert(video_id, video);

            // Trigger event
            Self::deposit_event(RawEvent::OfferCancelled(video_id, actor));
        }

        /// Accept incoming NFT offer
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn accept_incoming_offer(
            origin,
            video_id: T::VideoId,
            participant_id: MemberId<T>,
        ) {

            // Authorize participant under given member id
            let receiver_account_id = ensure_signed(origin)?;
            ensure_member_auth_success::<T>(&participant_id, &receiver_account_id)?;

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure new pending offer is available to proceed
            Self::ensure_new_pending_offer_available_to_proceed(&video, participant_id, &receiver_account_id)?;

            if let Some(owned_nft) = &video.nft_status {

                let owner_account_id = Self::ensure_owner_account_id(&video, &owned_nft)?;

                //
                // == MUTATION SAFE ==
                //

                // Complete nft offer
                let video = Self::complete_nft_offer(video, owner_account_id, receiver_account_id);

                VideoById::<T>::insert(video_id, video);

                // Trigger event
                Self::deposit_event(RawEvent::OfferAccepted(video_id, participant_id));
            }
        }

        /// Sell NFT
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn sell_nft(
            origin,
            video_id: T::VideoId,
            participant_id: MemberId<T>,
            price: BalanceOf<T>,
        ) {

            // Authorize participant under given member id
            let participant_account_id = ensure_signed(origin)?;
            ensure_member_auth_success::<T>(&participant_id, &participant_account_id)?;

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure participant_id is nft owner
            video.ensure_nft_ownership::<T>(&ChannelOwner::Member(participant_id))?;

            // Ensure there is no pending transfer or existing auction for given nft.
            video.ensure_nft_transactional_status_is_idle::<T>()?;

            //
            // == MUTATION SAFE ==
            //

            // Place nft sell order
            let video = video.set_buy_now_transactionl_status(price);

            VideoById::<T>::insert(video_id, video);

            // Trigger event
            Self::deposit_event(RawEvent::NFTSellOrderMade(video_id, participant_id, price));
        }

        /// Buy NFT
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn buy_nft(
            origin,
            video_id: T::VideoId,
            participant_id: MemberId<T>,
            metadata: Metadata,
        ) {

            // Authorize participant under given member id
            let participant_account_id = ensure_signed(origin)?;
            ensure_member_auth_success::<T>(&participant_id, &participant_account_id)?;

            // Ensure given video exists
            let video = Self::ensure_video_exists(&video_id)?;

            // Ensure given participant can buy nft now
            Self::ensure_can_buy_now(&video, &participant_account_id)?;

            if let Some(owned_nft) = &video.nft_status {

                let owner_account_id = Self::ensure_owner_account_id(&video, &owned_nft)?;

                //
                // == MUTATION SAFE ==
                //

                // Buy nft
                let video = Self::buy_now(video, owner_account_id, participant_account_id, participant_id);

                VideoById::<T>::insert(video_id, video);

                // Trigger event
                Self::deposit_event(RawEvent::NFTBought(video_id, participant_id, metadata));
            }
        }
    }
}

impl<T: Trait> Module<T> {
    /// Ensure `CuratorGroup` under given id exists
    fn ensure_curator_group_under_given_id_exists(
        curator_group_id: &T::CuratorGroupId,
    ) -> Result<(), Error<T>> {
        ensure!(
            <CuratorGroupById<T>>::contains_key(curator_group_id),
            Error::<T>::CuratorGroupDoesNotExist
        );
        Ok(())
    }

    /// Ensure `CuratorGroup` under given id exists, return corresponding one
    fn ensure_curator_group_exists(
        curator_group_id: &T::CuratorGroupId,
    ) -> Result<CuratorGroup<T>, Error<T>> {
        Self::ensure_curator_group_under_given_id_exists(curator_group_id)?;
        Ok(Self::curator_group_by_id(curator_group_id))
    }

    fn ensure_channel_exists(channel_id: &T::ChannelId) -> Result<Channel<T>, Error<T>> {
        ensure!(
            ChannelById::<T>::contains_key(channel_id),
            Error::<T>::ChannelDoesNotExist
        );
        Ok(ChannelById::<T>::get(channel_id))
    }

    fn ensure_video_exists(video_id: &T::VideoId) -> Result<Video<T>, Error<T>> {
        ensure!(
            VideoById::<T>::contains_key(video_id),
            Error::<T>::VideoDoesNotExist
        );
        Ok(VideoById::<T>::get(video_id))
    }

    // Ensure given video is not in season
    fn ensure_video_can_be_removed(video: Video<T>) -> DispatchResult {
        ensure!(video.in_series.is_none(), Error::<T>::VideoInSeason);
        Ok(())
    }

    fn ensure_channel_category_exists(
        channel_category_id: &T::ChannelCategoryId,
    ) -> Result<ChannelCategory, Error<T>> {
        ensure!(
            ChannelCategoryById::<T>::contains_key(channel_category_id),
            Error::<T>::CategoryDoesNotExist
        );
        Ok(ChannelCategoryById::<T>::get(channel_category_id))
    }

    fn ensure_video_category_exists(
        video_category_id: &T::VideoCategoryId,
    ) -> Result<VideoCategory, Error<T>> {
        ensure!(
            VideoCategoryById::<T>::contains_key(video_category_id),
            Error::<T>::CategoryDoesNotExist
        );
        Ok(VideoCategoryById::<T>::get(video_category_id))
    }

    fn pick_content_parameters_from_assets(
        assets: &[NewAsset<ContentParameters<T>>],
    ) -> Vec<ContentParameters<T>> {
        assets
            .iter()
            .filter_map(|asset| match asset {
                NewAsset::Upload(content_parameters) => Some(content_parameters.clone()),
                _ => None,
            })
            .collect()
    }

    fn actor_to_content_owner(
        actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    ) -> ActorToChannelOwnerResult<T> {
        match actor {
            // Lead should use their member or curator role to authorize
            ContentActor::Lead => Err(Error::<T>::ActorCannotBeLead),
            ContentActor::Curator(
                curator_group_id,
                _curator_id
            ) => {
                Ok(ChannelOwner::CuratorGroup(*curator_group_id))
            }
            ContentActor::Member(member_id) => {
                Ok(ChannelOwner::Member(*member_id))
            }
            // TODO:
            // ContentActor::Dao(id) => Ok(ChannelOwner::Dao(id)),
        }
    }

    /// Ensure owner account id exists, retreive corresponding one.
    pub fn ensure_owner_account_id(
        video: &Video<T>,
        owned_nft: &Nft<T>,
    ) -> Result<T::AccountId, Error<T>> {
        match owned_nft.owner {
            ChannelOwner::Member(member_id) => Self::ensure_member_controller_account_id(member_id),
            _ => {
                if let Some(reward_account) = Self::channel_by_id(video.in_channel).reward_account {
                    Ok(reward_account)
                } else {
                    Err(Error::<T>::RewardAccountIsNotSet)
                }
            }
        }
    }

    /// Ensure member controller account id exists, retrieve corresponding one.
    pub fn ensure_member_controller_account_id(
        member_id: T::MemberId,
    ) -> Result<T::AccountId, Error<T>> {
        let membership = <membership::Module<T>>::ensure_membership(member_id)
            .map_err(|_| Error::<T>::MemberProfileNotFound)?;
        Ok(membership.controller_account)
    }

    fn not_implemented() -> DispatchResult {
        Err(Error::<T>::FeatureNotImplemented.into())
    }
}

decl_event!(
    pub enum Event<T>
    where
        ContentActor = ContentActor<
            <T as ContentActorAuthenticator>::CuratorGroupId,
            <T as ContentActorAuthenticator>::CuratorId,
            MemberId<T>,
        >,
        ChannelOwner =
            ChannelOwner<MemberId<T>, <T as ContentActorAuthenticator>::CuratorGroupId, DAOId<T>>,
        MemberId = MemberId<T>,
        CuratorGroupId = <T as ContentActorAuthenticator>::CuratorGroupId,
        CuratorId = <T as ContentActorAuthenticator>::CuratorId,
        VideoId = <T as Trait>::VideoId,
        VideoCategoryId = <T as Trait>::VideoCategoryId,
        ChannelId = <T as StorageOwnership>::ChannelId,
        NewAsset = NewAsset<ContentParameters<T>>,
        ChannelCategoryId = <T as Trait>::ChannelCategoryId,
        ChannelOwnershipTransferRequestId = <T as Trait>::ChannelOwnershipTransferRequestId,
        PlaylistId = <T as Trait>::PlaylistId,
        SeriesId = <T as Trait>::SeriesId,
        PersonId = <T as Trait>::PersonId,
        ChannelOwnershipTransferRequest = ChannelOwnershipTransferRequest<T>,
        Series = Series<<T as StorageOwnership>::ChannelId, <T as Trait>::VideoId>,
        Channel = Channel<T>,
        ContentParameters = ContentParameters<T>,
        AccountId = <T as frame_system::Trait>::AccountId,
        ContentId = ContentId<T>,
        IsCensored = bool,
        AuctionParams = AuctionParams<
            <T as Trait>::VideoId,
            <T as frame_system::Trait>::BlockNumber,
            BalanceOf<T>,
            MemberId<T>,
        >,
        Balance = BalanceOf<T>,
    {
        // Curators
        CuratorGroupCreated(CuratorGroupId),
        CuratorGroupStatusSet(CuratorGroupId, bool /* active status */),
        CuratorAdded(CuratorGroupId, CuratorId),
        CuratorRemoved(CuratorGroupId, CuratorId),

        // Channels
        ChannelCreated(
            ContentActor,
            ChannelId,
            Channel,
            ChannelCreationParameters<ContentParameters, AccountId>,
        ),
        ChannelUpdated(
            ContentActor,
            ChannelId,
            Channel,
            ChannelUpdateParameters<ContentParameters, AccountId>,
        ),
        ChannelAssetsRemoved(ContentActor, ChannelId, Vec<ContentId>),

        ChannelCensorshipStatusUpdated(
            ContentActor,
            ChannelId,
            IsCensored,
            Vec<u8>, /* rationale */
        ),

        // Channel Ownership Transfers
        ChannelOwnershipTransferRequested(
            ContentActor,
            ChannelOwnershipTransferRequestId,
            ChannelOwnershipTransferRequest,
        ),
        ChannelOwnershipTransferRequestWithdrawn(ContentActor, ChannelOwnershipTransferRequestId),
        ChannelOwnershipTransferred(ContentActor, ChannelOwnershipTransferRequestId),

        // Channel Categories
        ChannelCategoryCreated(
            ChannelCategoryId,
            ChannelCategory,
            ChannelCategoryCreationParameters,
        ),
        ChannelCategoryUpdated(
            ContentActor,
            ChannelCategoryId,
            ChannelCategoryUpdateParameters,
        ),
        ChannelCategoryDeleted(ContentActor, ChannelCategoryId),

        // Videos
        VideoCategoryCreated(
            ContentActor,
            VideoCategoryId,
            VideoCategoryCreationParameters,
        ),
        VideoCategoryUpdated(ContentActor, VideoCategoryId, VideoCategoryUpdateParameters),
        VideoCategoryDeleted(ContentActor, VideoCategoryId),

        VideoCreated(
            ContentActor,
            ChannelId,
            VideoId,
            VideoCreationParameters<ContentParameters>,
        ),
        VideoUpdated(
            ContentActor,
            VideoId,
            VideoUpdateParameters<ContentParameters>,
        ),
        VideoDeleted(ContentActor, VideoId),

        VideoCensorshipStatusUpdated(
            ContentActor,
            VideoId,
            IsCensored,
            Vec<u8>, /* rationale */
        ),

        // Featured Videos
        FeaturedVideosSet(ContentActor, Vec<VideoId>),

        // Video Playlists
        PlaylistCreated(ContentActor, PlaylistId, PlaylistCreationParameters),
        PlaylistUpdated(ContentActor, PlaylistId, PlaylistUpdateParameters),
        PlaylistDeleted(ContentActor, PlaylistId),

        // Series
        SeriesCreated(
            ContentActor,
            SeriesId,
            Vec<NewAsset>,
            SeriesParameters<VideoId, ContentParameters>,
            Series,
        ),
        SeriesUpdated(
            ContentActor,
            SeriesId,
            Vec<NewAsset>,
            SeriesParameters<VideoId, ContentParameters>,
            Series,
        ),
        SeriesDeleted(ContentActor, SeriesId),

        // Persons
        PersonCreated(
            ContentActor,
            PersonId,
            Vec<NewAsset>,
            PersonCreationParameters<ContentParameters>,
        ),
        PersonUpdated(
            ContentActor,
            PersonId,
            Vec<NewAsset>,
            PersonUpdateParameters<ContentParameters>,
        ),
        PersonDeleted(ContentActor, PersonId),

        // NFT auction
        AuctionStarted(ContentActor, AuctionParams),
        NftIssued(
            ContentActor,
            VideoId,
            Option<Royalty>,
            Metadata,
            ChannelOwner,
        ),
        AuctionBidMade(MemberId, VideoId, Balance),
        AuctionBidCanceled(MemberId, VideoId),
        AuctionCancelled(ContentActor, VideoId),
        AuctionCompleted(MemberId, VideoId, Metadata),
        OpenAuctionBidAccepted(ContentActor, VideoId, Metadata),
        OfferStarted(VideoId, ContentActor, MemberId, Option<Balance>),
        OfferCancelled(VideoId, ContentActor),
        OfferAccepted(VideoId, MemberId),
        NFTSellOrderMade(VideoId, MemberId, Balance),
        NFTBought(VideoId, MemberId, Metadata),
    }
);
