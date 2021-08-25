mod types;
pub use types::*;

use crate::*;

impl<T: Trait> Module<T> {
    /// Authorize auctioneer
    pub(crate) fn authorize_auctioneer(
        origin: T::Origin,
        actor: &ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
        owner: &ChannelOwner<T::MemberId, T::CuratorGroupId, T::DAOId>,
        video: &Video<T>,
    ) -> Result<T::AccountId, DispatchError> {
        let account_id = ensure_signed(origin.clone())?;

        if video.is_vnft_issued() {
            // Only members are supposed to start auctions for already existing nfts
            if let ChannelOwner::Member(member_id) = owner {
                ensure_member_auth_success::<T>(member_id, &account_id)?;

                video.ensure_vnft_ownership::<T>(owner)?;
            } else {
                return Err(Error::<T>::ActorNotAuthorizedToManageAuction.into());
            }
        } else {
            ensure_actor_authorized_to_update_channel::<T>(origin, actor, owner)?;
        }
        Ok(account_id)
    }

    /// Ensure auction participant has sufficient balance to make bid
    pub(crate) fn ensure_has_sufficient_balance(
        participant: &T::AccountId,
        bid: BalanceOf<T>,
    ) -> DispatchResult {
        ensure!(
            T::Currency::can_reserve(participant, bid),
            Error::<T>::InsufficientBalance
        );
        Ok(())
    }

    /// Safety/bound checks for auction parameters
    pub(crate) fn validate_auction_params(
        auction_params: &AuctionParams<T::VideoId, T::BlockNumber, BalanceOf<T>, MemberId<T>>,
        video: &Video<T>,
    ) -> DispatchResult {
        if video.is_vnft_issued() {
            video.ensure_nft_transactional_status_is_idle::<T>()?;
        } else {
            video.ensure_none_issued::<T>()?;
            if let Some(creator_royalty) = auction_params.creator_royalty {
                Self::ensure_reward_account_is_set(video.in_channel)?;
                Self::ensure_royalty_bounds_satisfied(creator_royalty)?;
            }
        }

        Self::ensure_auction_duration_bounds_satisfied(auction_params.auction_duration)?;
        Self::ensure_starting_price_bounds_satisfied(auction_params.starting_price)?;
        Self::ensure_bid_step_bounds_satisfied(auction_params.minimal_bid_step)?;

        if let Some(starts_at) = auction_params.starts_at {
            Self::ensure_starts_at_delta_bounds_satisfied(starts_at)?;
        }

        Ok(())
    }

    /// Ensure starts at bounds satisfied
    pub(crate) fn ensure_starts_at_delta_bounds_satisfied(
        starts_at: T::BlockNumber,
    ) -> DispatchResult {
        ensure!(
            starts_at > <frame_system::Module<T>>::block_number(),
            Error::<T>::StartsAtLowerBoundExceeded
        );

        ensure!(
            starts_at
                <= <frame_system::Module<T>>::block_number() + Self::auction_starts_at_max_delta(),
            Error::<T>::StartsAtUpperBoundExceeded
        );

        Ok(())
    }

    /// Ensure channel reward_account account is set
    pub(crate) fn ensure_reward_account_is_set(channel_id: T::ChannelId) -> DispatchResult {
        Self::channel_by_id(channel_id)
            .reward_account
            .ok_or(Error::<T>::RewardAccountIsNotSet)?;
        Ok(())
    }

    /// Ensure royalty bounds satisfied
    pub(crate) fn ensure_royalty_bounds_satisfied(royalty: Perbill) -> DispatchResult {
        ensure!(
            royalty <= Self::max_creator_royalty(),
            Error::<T>::RoyaltyUpperBoundExceeded
        );
        ensure!(
            royalty >= Self::min_creator_royalty(),
            Error::<T>::RoyaltyLowerBoundExceeded
        );
        Ok(())
    }

    /// Ensure bid step bounds satisfied
    pub(crate) fn ensure_bid_step_bounds_satisfied(bid_step: BalanceOf<T>) -> DispatchResult {
        ensure!(
            bid_step <= Self::max_bid_step(),
            Error::<T>::AuctionBidStepUpperBoundExceeded
        );
        ensure!(
            bid_step >= Self::min_bid_step(),
            Error::<T>::AuctionBidStepLowerBoundExceeded
        );
        Ok(())
    }

    /// Ensure auction duration bounds satisfied
    pub(crate) fn ensure_auction_duration_bounds_satisfied(
        auction_duration: T::BlockNumber,
    ) -> DispatchResult {
        ensure!(
            auction_duration <= Self::max_auction_duration(),
            Error::<T>::RoundTimeUpperBoundExceeded
        );
        ensure!(
            auction_duration >= Self::min_auction_duration(),
            Error::<T>::RoundTimeLowerBoundExceeded
        );
        Ok(())
    }

    /// Ensure royalty bounds satisfied
    pub(crate) fn ensure_starting_price_bounds_satisfied(
        starting_price: BalanceOf<T>,
    ) -> DispatchResult {
        ensure!(
            starting_price >= Self::max_starting_price(),
            Error::<T>::StartingPriceUpperBoundExceeded
        );
        ensure!(
            starting_price <= Self::min_starting_price(),
            Error::<T>::StartingPriceLowerBoundExceeded
        );
        Ok(())
    }

    /// Issue vnft
    pub(crate) fn issue_vnft(
        video: &mut Video<T>,
        video_id: T::VideoId,
        owner: T::MemberId,
        creator_royalty: Option<Royalty>,
        metadata: Metadata,
    ) {
        if let NFTStatus::Owned(OwnedNFT {
            is_issued,
            transactional_status,
            ..
        }) = &mut video.nft_status
        {
            *transactional_status = TransactionalStatus::Idle;
            *is_issued = true;
        } else {
            video.nft_status = NFTStatus::Owned(OwnedNFT {
                is_issued: true,
                transactional_status: TransactionalStatus::Idle,
                owner: NFTOwner::Member(owner),
                creator_royalty,
            })
        }

        Self::deposit_event(RawEvent::NftIssued(
            video_id,
            video.nft_status.clone(),
            metadata,
        ));
    }

    /// Ensure given participant have sufficient free balance
    pub fn ensure_sufficient_free_balance(
        participant_account_id: &T::AccountId,
        balance: BalanceOf<T>,
    ) -> DispatchResult {
        ensure!(
            T::Currency::can_slash(participant_account_id, balance),
            Error::<T>::InsufficientBalance
        );
        Ok(())
    }

    /// Ensure given participant can buy vnft now
    pub fn ensure_can_buy_now(
        video: &Video<T>,
        participant_account_id: &T::AccountId,
    ) -> DispatchResult {
        if let NFTStatus::Owned(OwnedNFT {
            transactional_status: TransactionalStatus::BuyNow(order_details),
            ..
        }) = &video.nft_status
        {
            Self::ensure_sufficient_free_balance(participant_account_id, order_details.price)
        } else {
            Err(Error::<T>::VNFTNotInBuyNowState.into())
        }
    }

    /// Ensure new pending offer for given participant is available to proceed
    pub fn ensure_new_pending_offer_available_to_proceed(
        video: &Video<T>,
        participant: T::MemberId,
        participant_account_id: &T::AccountId,
    ) -> DispatchResult {
        match &video.nft_status {
            NFTStatus::Owned(OwnedNFT {
                transactional_status: TransactionalStatus::InitiatedOfferToMember(to, order_details),
                ..
            }) if participant == *to => {
                if let Some(order_details) = order_details {
                    Self::ensure_sufficient_free_balance(
                        participant_account_id,
                        order_details.price,
                    )?;
                }
            }
            _ => return Err(Error::<T>::NoIncomingTransfers.into()),
        }
        Ok(())
    }

    /// Buy vnft
    pub fn buy_now(
        mut video: Video<T>,
        new_owner_account_id: T::AccountId,
        new_owner: T::MemberId,
    ) -> Video<T> {
        if let NFTStatus::Owned(OwnedNFT {
            transactional_status: TransactionalStatus::BuyNow(order_details),
            ref mut owner,
            ..
        }) = &mut video.nft_status
        {
            T::Currency::slash(&new_owner_account_id, order_details.price);

            T::Currency::deposit_creating(&order_details.account_id, order_details.price);

            *owner = NFTOwner::Member(new_owner);
        }

        video.set_idle_transactional_status()
    }

    /// Completes vnft offer
    pub fn complete_vnft_offer(
        mut video: Video<T>,
        new_owner_account_id: T::AccountId,
    ) -> Video<T> {
        if let NFTStatus::Owned(OwnedNFT {
            transactional_status: TransactionalStatus::InitiatedOfferToMember(to, order_details),
            ref mut owner,
            ..
        }) = &mut video.nft_status
        {
            if let Some(order_details) = order_details {
                T::Currency::slash(&new_owner_account_id, order_details.price);

                T::Currency::deposit_creating(&order_details.account_id, order_details.price);
            }

            *owner = NFTOwner::Member(*to);
        }

        video.set_idle_transactional_status()
    }

    /// Complete vnft transfer
    pub(crate) fn complete_vnft_auction_transfer(
        video: &mut Video<T>,
        auction_fee: BalanceOf<T>,
        last_bidder_account_id: T::AccountId,
        last_bidder: T::MemberId,
        owner_account_id: T::AccountId,
        last_bid_amount: BalanceOf<T>,
    ) {
        if let NFTStatus::Owned(OwnedNFT {
            owner,
            transactional_status,
            creator_royalty,
            ..
        }) = &mut video.nft_status
        {
            if let Some(creator_royalty) = creator_royalty {
                let royalty = *creator_royalty * last_bid_amount;

                // Slash last bidder bid
                T::Currency::slash_reserved(&last_bidder_account_id, last_bid_amount);

                // Deposit bid, exluding royalty amount and auction fee into auctioneer account
                if last_bid_amount > royalty + auction_fee {
                    T::Currency::deposit_creating(
                        &owner_account_id,
                        last_bid_amount - royalty - auction_fee,
                    );
                } else {
                    T::Currency::deposit_creating(&owner_account_id, last_bid_amount - auction_fee);
                }

                // Should always be Some(_) at this stage, because of previously made check.
                if let Some(creator_account_id) =
                    Self::channel_by_id(video.in_channel).reward_account
                {
                    // Deposit royalty into creator account
                    T::Currency::deposit_creating(&creator_account_id, royalty);
                }
            } else {
                // Slash last bidder bid and deposit it into auctioneer account
                T::Currency::slash_reserved(&last_bidder_account_id, last_bid_amount);

                // Deposit bid, exluding auction fee into auctioneer account
                T::Currency::deposit_creating(&owner_account_id, last_bid_amount - auction_fee);
            }

            *owner = NFTOwner::Member(last_bidder);
            *transactional_status = TransactionalStatus::Idle;
        }
    }

    /// Complete auction
    pub(crate) fn complete_auction(
        mut video: Video<T>,
        video_id: T::VideoId,
        last_bidder_account_id: T::AccountId,
        last_bidder: T::MemberId,
        owner_account_id: T::AccountId,
        auction_mode: AuctionMode,
    ) -> Video<T> {
        if let NFTStatus::Owned(OwnedNFT {
            transactional_status: TransactionalStatus::Auction(auction),
            ..
        }) = &video.nft_status
        {
            let auction = auction.to_owned();
            if let Some(last_bid) = auction.last_bid {
                let bid = last_bid.amount;
                let auction_fee = Self::auction_fee_percentage() * bid;

                match auction_mode {
                    AuctionMode::WithIssuance(metadata) => {
                        // Slash last bidder bid
                        T::Currency::slash_reserved(&last_bidder_account_id, bid);
                        // Deposit last bidder bid minus auction fee into auctioneer account
                        T::Currency::deposit_creating(&owner_account_id, bid - auction_fee);

                        // Issue vnft.
                        // We do not need to provide creator royalty here,
                        // because this data have been already provided at the auction start
                        Self::issue_vnft(&mut video, video_id, last_bidder, None, metadata);
                    }
                    AuctionMode::WithoutIsuance => {
                        Self::complete_vnft_auction_transfer(
                            &mut video,
                            auction_fee,
                            last_bidder_account_id,
                            last_bidder,
                            owner_account_id,
                            bid,
                        );
                    }
                }
            }
        }
        video
    }
}
