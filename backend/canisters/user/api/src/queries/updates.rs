use crate::{initial_state::PinNumberSettings, WalletConfig};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{
    Chat, ChatId, ChitEarned, CommunityId, DirectChatSummary, DirectChatSummaryUpdates, OptionUpdate, TimestampMillis, UserId,
};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub updates_since: TimestampMillis,
}

#[allow(clippy::large_enum_variant)]
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    SuccessNoUpdates,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub timestamp: TimestampMillis,
    pub username: Option<String>,
    pub display_name: OptionUpdate<String>,
    pub direct_chats: DirectChatsUpdates,
    pub group_chats: GroupChatsUpdates,
    pub favourite_chats: FavouriteChatsUpdates,
    pub communities: CommunitiesUpdates,
    pub avatar_id: OptionUpdate<u128>,
    pub blocked_users: Option<Vec<UserId>>,
    pub suspended: Option<bool>,
    pub pin_number_settings: OptionUpdate<PinNumberSettings>,
    pub achievements: Vec<ChitEarned>,
    pub achievements_last_seen: Option<TimestampMillis>,
    pub total_chit_earned: i32,
    pub chit_balance: i32,
    pub streak: u16,
    pub streak_ends: TimestampMillis,
    pub next_daily_claim: TimestampMillis,
    pub is_unique_person: Option<bool>,
    pub wallet_config: Option<WalletConfig>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DirectChatsUpdates {
    pub added: Vec<DirectChatSummary>,
    pub updated: Vec<DirectChatSummaryUpdates>,
    pub removed: Vec<ChatId>,
    pub pinned: Option<Vec<ChatId>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GroupChatsUpdates {
    pub added: Vec<crate::GroupChatSummary>,
    pub updated: Vec<crate::GroupChatSummaryUpdates>,
    pub removed: Vec<ChatId>,
    pub pinned: Option<Vec<ChatId>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CommunitiesUpdates {
    pub added: Vec<crate::CommunitySummary>,
    pub updated: Vec<crate::CommunitySummaryUpdates>,
    pub removed: Vec<CommunityId>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FavouriteChatsUpdates {
    pub chats: Option<Vec<Chat>>,
    pub pinned: Option<Vec<Chat>>,
}
