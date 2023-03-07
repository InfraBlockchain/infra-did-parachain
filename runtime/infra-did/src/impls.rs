use crate::{Authorship, Balances, NegativeImbalance};
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

pub struct Author;
impl OnUnbalanced<NegativeImbalance> for Author {
    fn on_nonzero_unbalanced(amount: NegativeImbalance) {
        if let Some(author) = Authorship::author() {
            Balances::resolve_creating(&author, amount);
        }
    }
}

pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
        if let Some(fees) = fees_then_tips.next() {
            let mut to_author = fees;
            if let Some(tips) = fees_then_tips.next() {
                tips.merge_into(&mut to_author);
            }
            Author::on_unbalanced(to_author);
        }
    }
}
