use pinocchio::AccountView;
fn test(a: &AccountView) {
    let _ = a.key();
    let _ = a.address();
    let _ = a.pubkey();
}
