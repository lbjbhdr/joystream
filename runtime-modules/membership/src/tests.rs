#![cfg(test)]

use super::genesis;
use super::mock::*;

use srml_support::*;

fn assert_ok_unwrap<T>(value: Option<T>, err: &'static str) -> T {
    match value {
        None => {
            assert!(false, err);
            // although this code path is not reached, we need to return correct type
            // in match arm. Using panic! would remove this need, but assert! gives us better error in
            // console output
            value.unwrap()
        }
        Some(v) => v,
    }
}

fn get_membership_by_id(member_id: u32) -> crate::MembershipOf<Test> {
    if <crate::MembershipById<Test>>::exists(member_id) {
        Members::membership(member_id)
    } else {
        panic!("member profile not created");
    }
}

fn assert_dispatch_error_message(result: Result<(), &'static str>, expected_message: &'static str) {
    assert!(result.is_err());
    let message = result.err().unwrap();
    assert_eq!(message, expected_message);
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestUserInfo {
    pub handle: Option<Vec<u8>>,
    pub avatar_uri: Option<Vec<u8>>,
    pub about: Option<Vec<u8>>,
}

fn get_alice_info() -> TestUserInfo {
    TestUserInfo {
        handle: Some(String::from("alice").as_bytes().to_vec()),
        avatar_uri: Some(
            String::from("http://avatar-url.com/alice")
                .as_bytes()
                .to_vec(),
        ),
        about: Some(String::from("my name is alice").as_bytes().to_vec()),
    }
}

fn get_bob_info() -> TestUserInfo {
    TestUserInfo {
        handle: Some(String::from("bobby").as_bytes().to_vec()),
        avatar_uri: Some(
            String::from("http://avatar-url.com/bob")
                .as_bytes()
                .to_vec(),
        ),
        about: Some(String::from("my name is bob").as_bytes().to_vec()),
    }
}

const ALICE_ACCOUNT_ID: u64 = 1;

fn buy_default_membership_as_alice() -> dispatch::Result {
    let info = get_alice_info();
    Members::buy_membership(
        Origin::signed(ALICE_ACCOUNT_ID),
        DEFAULT_PAID_TERM_ID,
        info.handle,
        info.avatar_uri,
        info.about,
    )
}

fn set_alice_free_balance(balance: u64) {
    let _ = Balances::deposit_creating(&ALICE_ACCOUNT_ID, balance);
}

#[test]
fn initial_state() {
    const DEFAULT_FEE: u64 = 500;
    let initial_members = [1, 2, 3];

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .default_paid_membership_fee(DEFAULT_FEE)
                .members(initial_members.to_vec())
                .build(),
        )
        .build()
        .execute_with(|| {
            let default_terms = assert_ok_unwrap(
                Members::paid_membership_terms_by_id(DEFAULT_PAID_TERM_ID),
                "default terms not initialized",
            );

            assert_eq!(default_terms.fee, DEFAULT_FEE);

            // initial balance
            assert_eq!(
                Balances::free_balance(initial_members[0]),
                <Test as crate::Trait>::InitialMembersBalance::get()
            );
        });
}

#[test]
fn buy_membership() {
    const DEFAULT_FEE: u64 = 500;
    const SURPLUS_BALANCE: u64 = 500;

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .default_paid_membership_fee(DEFAULT_FEE)
                .build(),
        )
        .build()
        .execute_with(|| {
            let initial_balance = DEFAULT_FEE + SURPLUS_BALANCE;
            set_alice_free_balance(initial_balance);

            let next_member_id = Members::members_created();

            assert_ok!(buy_default_membership_as_alice());

            let member_ids = vec![0];
            assert_eq!(member_ids, vec![next_member_id]);

            let profile = get_membership_by_id(next_member_id);

            assert_eq!(Some(profile.handle), get_alice_info().handle);
            assert_eq!(Some(profile.avatar_uri), get_alice_info().avatar_uri);
            assert_eq!(Some(profile.about), get_alice_info().about);

            assert_eq!(Balances::free_balance(&ALICE_ACCOUNT_ID), SURPLUS_BALANCE);

            // controller account initially set to primary account
            assert_eq!(profile.controller_account, ALICE_ACCOUNT_ID);
            assert_eq!(
                <crate::MemberIdsByControllerAccountId<Test>>::get(ALICE_ACCOUNT_ID),
                vec![next_member_id]
            );
        });
}

#[test]
fn buy_membership_fails_without_enough_balance() {
    const DEFAULT_FEE: u64 = 500;

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .default_paid_membership_fee(DEFAULT_FEE)
                .build(),
        )
        .build()
        .execute_with(|| {
            let initial_balance = DEFAULT_FEE - 1;
            set_alice_free_balance(initial_balance);

            assert_dispatch_error_message(
                buy_default_membership_as_alice(),
                "not enough balance to buy membership",
            );
        });
}

#[test]
fn new_memberships_allowed_flag() {
    const DEFAULT_FEE: u64 = 500;

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .default_paid_membership_fee(DEFAULT_FEE)
                .build(),
        )
        .build()
        .execute_with(|| {
            let initial_balance = DEFAULT_FEE + 1;
            set_alice_free_balance(initial_balance);

            crate::NewMembershipsAllowed::put(false);

            assert_dispatch_error_message(
                buy_default_membership_as_alice(),
                "new members not allowed",
            );
        });
}

#[test]
fn unique_handles() {
    const DEFAULT_FEE: u64 = 500;
    const SURPLUS_BALANCE: u64 = 500;

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .default_paid_membership_fee(DEFAULT_FEE)
                .build(),
        )
        .build()
        .execute_with(|| {
            let initial_balance = DEFAULT_FEE + SURPLUS_BALANCE;
            set_alice_free_balance(initial_balance);

            // alice's handle already taken
            <crate::MemberIdByHandle<Test>>::insert(get_alice_info().handle.unwrap(), 1);

            // should not be allowed to buy membership with that handle
            assert_dispatch_error_message(
                buy_default_membership_as_alice(),
                "handle already registered",
            );
        });
}

#[test]
fn update_profile() {
    const DEFAULT_FEE: u64 = 500;
    const SURPLUS_BALANCE: u64 = 500;

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .default_paid_membership_fee(DEFAULT_FEE)
                .build(),
        )
        .build()
        .execute_with(|| {
            let initial_balance = DEFAULT_FEE + SURPLUS_BALANCE;
            set_alice_free_balance(initial_balance);

            let next_member_id = Members::members_created();

            assert_ok!(buy_default_membership_as_alice());
            let info = get_bob_info();
            assert_ok!(Members::update_membership(
                Origin::signed(ALICE_ACCOUNT_ID),
                next_member_id,
                info.handle,
                info.avatar_uri,
                info.about,
            ));

            let profile = get_membership_by_id(next_member_id);

            assert_eq!(Some(profile.handle), get_bob_info().handle);
            assert_eq!(Some(profile.avatar_uri), get_bob_info().avatar_uri);
            assert_eq!(Some(profile.about), get_bob_info().about);
        });
}

#[test]
fn add_screened_member() {
    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(genesis::GenesisConfigBuilder::default().build())
        .build()
        .execute_with(|| {
            let screening_authority = 5;
            <crate::ScreeningAuthority<Test>>::put(&screening_authority);

            let next_member_id = Members::members_created();

            let info = get_alice_info();
            assert_ok!(Members::add_screened_member(
                Origin::signed(screening_authority),
                ALICE_ACCOUNT_ID,
                info.handle,
                info.avatar_uri,
                info.about
            ));

            let profile = get_membership_by_id(next_member_id);

            assert_eq!(Some(profile.handle), get_alice_info().handle);
            assert_eq!(Some(profile.avatar_uri), get_alice_info().avatar_uri);
            assert_eq!(Some(profile.about), get_alice_info().about);
            assert_eq!(
                crate::EntryMethod::Screening(screening_authority),
                profile.entry
            );
        });
}

#[test]
fn set_controller_key() {
    let initial_members = [ALICE_ACCOUNT_ID];
    const ALICE_CONTROLLER_ID: u64 = 2;

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .members(initial_members.to_vec())
                .build(),
        )
        .build()
        .execute_with(|| {
            let member_id = 0;

            assert_ok!(Members::set_controller_account(
                Origin::signed(ALICE_ACCOUNT_ID),
                member_id,
                ALICE_CONTROLLER_ID
            ));

            let profile = get_membership_by_id(member_id);

            assert_eq!(profile.controller_account, ALICE_CONTROLLER_ID);
            assert_eq!(
                <crate::MemberIdsByControllerAccountId<Test>>::get(&ALICE_CONTROLLER_ID),
                vec![member_id]
            );
            assert!(
                <crate::MemberIdsByControllerAccountId<Test>>::get(&ALICE_ACCOUNT_ID).is_empty()
            );
        });
}

#[test]
fn set_root_account() {
    let initial_members = [ALICE_ACCOUNT_ID];
    const ALICE_NEW_ROOT_ACCOUNT: u64 = 2;

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .members(initial_members.to_vec())
                .build(),
        )
        .build()
        .execute_with(|| {
            let member_id = 0;

            assert_ok!(Members::set_root_account(
                Origin::signed(ALICE_ACCOUNT_ID),
                member_id,
                ALICE_NEW_ROOT_ACCOUNT
            ));

            let membership = Members::membership(member_id);

            assert_eq!(ALICE_ACCOUNT_ID, membership.root_account);

            assert!(<crate::MemberIdsByRootAccountId<Test>>::get(&ALICE_ACCOUNT_ID).is_empty());
        });
}

#[test]
fn registering_and_unregistering_roles_on_member() {
    let initial_members = [1, 2];

    TestExternalitiesBuilder::<Test>::default()
        .set_membership_config(
            genesis::GenesisConfigBuilder::default()
                .members(initial_members.to_vec())
                .build(),
        )
        .build()
        .execute_with(|| {
            const DUMMY_ACTOR_ID: u32 = 100;
            let member_id_1 = 0;
            let member_id_2 = 0;

            // no initial roles for member
            assert!(!Members::member_is_in_role(
                member_id_1,
                crate::Role::ChannelOwner
            ));

            // REGISTERING

            // successful registration
            assert_ok!(Members::register_role_on_member(
                member_id_1,
                &crate::ActorInRole::new(crate::Role::ChannelOwner, DUMMY_ACTOR_ID)
            ));
            assert!(Members::member_is_in_role(
                member_id_1,
                crate::Role::ChannelOwner
            ));

            // registering another member in same role and actorid combination should fail
            assert_dispatch_error_message(
                Members::register_role_on_member(
                    member_id_2,
                    &crate::ActorInRole::new(crate::Role::ChannelOwner, DUMMY_ACTOR_ID),
                ),
                "ActorInRoleAlreadyExists",
            );

            // UNREGISTERING

            // trying to unregister non existent actor role should fail
            assert_dispatch_error_message(
                Members::unregister_role(crate::ActorInRole::new(crate::Role::Curator, 222)),
                "ActorInRoleNotFound",
            );

            // successfully unregister role
            assert_ok!(Members::unregister_role(crate::ActorInRole::new(
                crate::Role::ChannelOwner,
                DUMMY_ACTOR_ID
            )));
            assert!(!Members::member_is_in_role(
                member_id_1,
                crate::Role::ChannelOwner
            ));
        });
}
