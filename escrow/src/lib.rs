// Inside approve_exit, after successful refund transfer
if let Some(pos) = payout_order.iter().position(|&addr| addr == exited_member) {
    compact_payout_order(env, pos);
    // Emit event
    env.events().publish(
        (Symbol::new(env, "PayoutOrderCompacted"),),
        PayoutOrderCompacted { removed_member: exited_member, new_length: payout_order.len() as u32 }
    );
}

// New helper function
fn compact_payout_order(env: &Env, removed_index: usize) {
    let mut order: Vec<Address> = env.storage().get(&DataKey::PayoutOrder).unwrap_or_else(|| vec![]);
    order.remove(removed_index);
    env.storage().set(&DataKey::PayoutOrder, &order);
    
    // Update TotalRounds if needed
    // ...
}
