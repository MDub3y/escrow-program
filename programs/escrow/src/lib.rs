use anchor_lang::prelude::*;

pub mod error;
pub mod handlers;
pub mod state;

use handlers::*;

declare_id!("B2fbWZXvAZGdjBV7bwt5LzsD5VPwwf6errc134sVHzdr");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amt: u64,
        token_b_wanted_amt: u64,
    ) -> Result<()> {
        handlers::make_offer::make_offer(context, id, token_a_offered_amt, token_b_wanted_amt)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        handlers::take_offer::take_offer(context)
    }

    pub fn refund_offer(context: Context<RefundOffer>) -> Result<()> {
        handlers::refund_offer::refund_offer(context)
    }
}
