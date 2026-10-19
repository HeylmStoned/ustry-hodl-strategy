#![cfg(test)]

//! Integration tests for the Prism USTRY hodl strategy.
//!
//! The vault is simulated by a plain address that holds USTRY on a test
//! Stellar Asset Contract. Every state-changing call goes through the
//! generated client so the auth tree is checked like on mainnet.

use super::{StrategyError, UstryHodlStrategy, UstryHodlStrategyClient};
use soroban_sdk::testutils::{Address as _, Events as _, IssuerFlags, Ledger as _, MockAuth, MockAuthInvoke};
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{vec, Address, Env, IntoVal, Val, Vec};

const DAY_LEDGERS: u32 = 17280;
/// Mainnet minimum TTL of a persistent or instance entry (2,073,600 ledgers, about 120 days at 5s).
const MAINNET_MIN_PERSISTENT_TTL: u32 = 2_073_600;

struct Setup<'a> {
    env: Env,
    strategy: UstryHodlStrategyClient<'a>,
    strategy_id: Address,
    token: TokenClient<'a>,
    token_admin: StellarAssetClient<'a>,
    token_id: Address,
    vault: Address,
}

fn setup() -> Setup<'static> {
    setup_in(Env::default())
}

fn setup_in(env: Env) -> Setup<'static> {
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    // USTRY's issuer has auth_revocable and auth_clawback_enabled on mainnet: mirror it.
    sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);
    let token_id = sac.address();
    let init_args: Vec<Val> = Vec::new(&env);
    let strategy_id = env.register(UstryHodlStrategy, (token_id.clone(), init_args));
    let strategy = UstryHodlStrategyClient::new(&env, &strategy_id);
    let token = TokenClient::new(&env, &token_id);
    let token_admin = StellarAssetClient::new(&env, &token_id);
    let vault = Address::generate(&env);
    Setup { env, strategy, strategy_id, token, token_admin, token_id, vault }
}

#[test]
fn asset_is_set_by_constructor() {
    let s = setup();
    assert_eq!(s.strategy.asset(), s.token_id);
}

#[test]
fn deposit_moves_tokens_and_credits_balance() {
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &1_000_0000000);

    let remaining = s.strategy.deposit(&400_0000000, &s.vault);

    assert_eq!(remaining, 400_0000000);
    assert_eq!(s.strategy.balance(&s.vault), 400_0000000);
    assert_eq!(s.token.balance(&s.strategy_id), 400_0000000);
    assert_eq!(s.token.balance(&s.vault), 600_0000000);
}

#[test]
fn deposit_emits_event() {
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &10_0000000);
    s.strategy.deposit(&10_0000000, &s.vault);
    let events = s.env.events().all();
    let last = events.last().unwrap();
    assert_eq!(last.0, s.strategy_id);
}

#[test]
fn withdraw_returns_tokens_to_recipient() {
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &500_0000000);
    s.strategy.deposit(&500_0000000, &s.vault);

    let to = Address::generate(&s.env);
    let remaining = s.strategy.withdraw(&200_0000000, &s.vault, &to);

    assert_eq!(remaining, 300_0000000);
    assert_eq!(s.strategy.balance(&s.vault), 300_0000000);
    assert_eq!(s.token.balance(&to), 200_0000000);
    assert_eq!(s.token.balance(&s.strategy_id), 300_0000000);
}

#[test]
fn withdraw_more_than_balance_fails() {
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &100_0000000);
    s.strategy.deposit(&100_0000000, &s.vault);

    let res = s.strategy.try_withdraw(&100_0000001, &s.vault, &s.vault);
    assert_eq!(res, Err(Ok(StrategyError::InsufficientBalance)));
    assert_eq!(s.strategy.balance(&s.vault), 100_0000000);
}

#[test]
fn negative_amounts_are_rejected() {
    let s = setup();
    s.env.mock_all_auths();
    assert_eq!(s.strategy.try_deposit(&-1, &s.vault), Err(Ok(StrategyError::NegativeNotAllowed)));
    assert_eq!(s.strategy.try_withdraw(&-1, &s.vault, &s.vault), Err(Ok(StrategyError::NegativeNotAllowed)));
}

#[test]
fn zero_amount_is_currently_accepted() {
    // Documents today's behaviour (STRIDE Tamper.4 / DoS.3): a zero deposit
    // succeeds, writes storage and emits an event. The treatment is to reject it.
    let s = setup();
    s.env.mock_all_auths();
    assert_eq!(s.strategy.deposit(&0, &s.vault), 0);
    assert_eq!(s.strategy.balance(&s.vault), 0);
}

#[test]
fn balances_are_isolated_per_depositor() {
    // A third party depositing under its own key cannot touch the vault's balance (STRIDE Spoof.1).
    let s = setup();
    s.env.mock_all_auths();
    let other = Address::generate(&s.env);
    s.token_admin.mint(&s.vault, &100_0000000);
    s.token_admin.mint(&other, &50_0000000);
    s.strategy.deposit(&100_0000000, &s.vault);
    s.strategy.deposit(&50_0000000, &other);

    assert_eq!(s.strategy.balance(&s.vault), 100_0000000);
    assert_eq!(s.strategy.balance(&other), 50_0000000);
    let res = s.strategy.try_withdraw(&60_0000000, &other, &other);
    assert_eq!(res, Err(Ok(StrategyError::InsufficientBalance)));
    assert_eq!(s.token.balance(&s.strategy_id), 150_0000000);
}

#[test]
fn deposit_requires_from_auth() {
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &10_0000000);
    // Now only authorise an unrelated address: the vault's auth is missing.
    let stranger = Address::generate(&s.env);
    s.env.mock_auths(&[MockAuth {
        address: &stranger,
        invoke: &MockAuthInvoke {
            contract: &s.strategy_id,
            fn_name: "deposit",
            args: vec![&s.env, 10_0000000i128.into_val(&s.env), s.vault.to_val()],
            sub_invokes: &[],
        },
    }]);
    let res = s.strategy.try_deposit(&10_0000000, &s.vault);
    assert!(res.is_err(), "deposit without the vault's auth must fail");
    assert_eq!(s.token.balance(&s.strategy_id), 0);
}

#[test]
fn withdraw_requires_from_auth() {
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &10_0000000);
    s.strategy.deposit(&10_0000000, &s.vault);

    let thief = Address::generate(&s.env);
    s.env.mock_auths(&[MockAuth {
        address: &thief,
        invoke: &MockAuthInvoke {
            contract: &s.strategy_id,
            fn_name: "withdraw",
            args: vec![&s.env, 10_0000000i128.into_val(&s.env), s.vault.to_val(), thief.to_val()],
            sub_invokes: &[],
        },
    }]);
    let res = s.strategy.try_withdraw(&10_0000000, &s.vault, &thief);
    assert!(res.is_err(), "withdraw without the vault's auth must fail");
    assert_eq!(s.token.balance(&s.strategy_id), 10_0000000);
    assert_eq!(s.token.balance(&thief), 0);
}

#[test]
fn harvest_is_a_no_op_and_needs_no_auth() {
    // Documents today's behaviour (STRIDE Spoof.2): anyone can emit a harvest event for any address.
    let s = setup();
    s.token_admin.mock_all_auths().mint(&s.vault, &10_0000000);
    s.env.mock_all_auths();
    s.strategy.deposit(&10_0000000, &s.vault);
    s.env.mock_auths(&[]);
    s.strategy.harvest(&s.vault, &None);
    // events().all() only holds the events of the last top-level invocation.
    let events = s.env.events().all();
    assert_eq!(events.len(), 1);
    assert_eq!(events.last().unwrap().0, s.strategy_id);
    assert_eq!(s.strategy.balance(&s.vault), 10_0000000);
}

#[test]
fn balance_reports_ledger_not_actual_holdings_after_clawback() {
    // STRIDE Tamper.1: an issuer clawback empties the contract's SAC balance while
    // balance() keeps reporting the internal ledger. The next withdraw then fails.
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &100_0000000);
    s.strategy.deposit(&100_0000000, &s.vault);

    s.token_admin.clawback(&s.strategy_id, &100_0000000);

    assert_eq!(s.token.balance(&s.strategy_id), 0);
    assert_eq!(s.strategy.balance(&s.vault), 100_0000000);
    let res = s.strategy.try_withdraw(&1_0000000, &s.vault, &s.vault);
    assert!(res.is_err(), "withdraw after clawback must fail at the SAC transfer");
}

#[test]
fn freeze_by_issuer_blocks_withdraw() {
    // STRIDE DoS.2: the issuer revokes the strategy's authorisation; withdrawals fail until it is restored.
    let s = setup();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &100_0000000);
    s.strategy.deposit(&100_0000000, &s.vault);

    s.token_admin.set_authorized(&s.strategy_id, &false);
    let res = s.strategy.try_withdraw(&1_0000000, &s.vault, &s.vault);
    assert!(res.is_err(), "withdraw while frozen must fail");

    s.token_admin.set_authorized(&s.strategy_id, &true);
    assert_eq!(s.strategy.withdraw(&1_0000000, &s.vault, &s.vault), 99_0000000);
}

fn setup_mainnet_ttl() -> Setup<'static> {
    // TTL parameters must be set before registration: entries take the minimum TTL at creation.
    let env = Env::default();
    env.ledger().with_mut(|l| {
        l.min_persistent_entry_ttl = MAINNET_MIN_PERSISTENT_TTL;
        l.max_entry_ttl = 6_312_000;
    });
    setup_in(env)
}

#[test]
fn storage_lives_120_days_on_mainnet_parameters() {
    // With mainnet minimums a fresh entry lives 120 days; the contract's 1-day threshold never triggers before that.
    let s = setup_mainnet_ttl();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &10_0000000);
    s.strategy.deposit(&10_0000000, &s.vault);
    s.env.ledger().with_mut(|l| l.sequence_number += DAY_LEDGERS * 119);
    assert_eq!(s.strategy.balance(&s.vault), 10_0000000);
}

#[test]
#[should_panic(expected = "Error(Storage, InternalError)")]
fn storage_is_archived_after_120_days_without_activity() {
    // STRIDE DoS.1: 121 days without any call archives the entries; every call then fails until a restore.
    let s = setup_mainnet_ttl();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &10_0000000);
    s.strategy.deposit(&10_0000000, &s.vault);
    s.env.ledger().with_mut(|l| l.sequence_number += DAY_LEDGERS * 121);
    let _ = s.strategy.balance(&s.vault);
}

#[test]
#[should_panic(expected = "Error(Storage, InternalError)")]
fn balance_read_does_not_extend_the_balance_entry() {
    // STRIDE DoS.1: only write_balance bumps Balance(from). A read at day 119 keeps the instance alive
    // (extend_instance runs) but not the Balance entry, which still archives at day 120.
    let s = setup_mainnet_ttl();
    s.env.mock_all_auths();
    s.token_admin.mint(&s.vault, &10_0000000);
    s.strategy.deposit(&10_0000000, &s.vault);
    s.env.ledger().with_mut(|l| l.sequence_number += DAY_LEDGERS * 119 + DAY_LEDGERS / 2);
    assert_eq!(s.strategy.balance(&s.vault), 10_0000000);
    s.env.ledger().with_mut(|l| l.sequence_number += DAY_LEDGERS * 2);
    let _ = s.strategy.balance(&s.vault);
}
