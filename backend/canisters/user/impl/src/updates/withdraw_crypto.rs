use crate::crypto::process_transaction;
use crate::guards::caller_is_owner;
use crate::run_regular_jobs;
use canister_tracing_macros::trace;
use ic_cdk_macros::update;
use user_canister::withdraw_crypto::{Response::*, *};

#[update(guard = "caller_is_owner")]
#[trace]
async fn withdraw_crypto(args: Args) -> Response {
    run_regular_jobs();

    match process_transaction(args.withdrawal).await {
        Ok(completed_withdrawal) => Success(completed_withdrawal),
        Err(failed_withdrawal) => TransactionFailed(failed_withdrawal),
    }
}
