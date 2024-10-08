use crate::{activity_notifications::handle_activity_notification, mutate_state, run_regular_jobs, RuntimeState};
use canister_tracing_macros::trace;
use chat_events::{EditMessageArgs, EditMessageResult};
use community_canister::edit_message::{Response::*, *};
use ic_cdk::update;
use types::Achievement;

#[update]
#[trace]
fn edit_message(args: Args) -> Response {
    run_regular_jobs();

    mutate_state(|state| edit_message_impl(args, state))
}

fn edit_message_impl(args: Args, state: &mut RuntimeState) -> Response {
    if state.data.is_frozen() {
        return CommunityFrozen;
    }

    let caller = state.env.caller();
    if let Some(member) = state.data.members.get(caller) {
        if member.suspended.value {
            return UserSuspended;
        }

        let now = state.env.now();

        if let Some(channel) = state.data.channels.get_mut(&args.channel_id) {
            let sender = member.user_id;
            if let Some(channel_member) = channel.chat.members.get(&sender) {
                match channel.chat.events.edit_message(
                    EditMessageArgs {
                        sender,
                        min_visible_event_index: channel_member.min_visible_event_index(),
                        thread_root_message_index: args.thread_root_message_index,
                        message_id: args.message_id,
                        content: args.content,
                        block_level_markdown: args.block_level_markdown,
                        now,
                    },
                    Some(&mut state.data.event_store_client),
                ) {
                    EditMessageResult::Success => {
                        handle_activity_notification(state);

                        if args.new_achievement {
                            state.data.achievements.notify_user(
                                sender,
                                vec![Achievement::EditedMessage],
                                &mut state.data.fire_and_forget_handler,
                            );
                        }

                        Success
                    }
                    EditMessageResult::NotAuthorized => MessageNotFound,
                    EditMessageResult::NotFound => MessageNotFound,
                }
            } else {
                UserNotInChannel
            }
        } else {
            ChannelNotFound
        }
    } else {
        UserNotInCommunity
    }
}
