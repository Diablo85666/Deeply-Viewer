//! Firestorm's catalogue of group abilities (indra/newview/skins/default/xui/
//! en/role_actions.xml, read by LLGroupMgr::parseRoleActions, originally
//! LGPL 2.1), generated: the sets in their order, each ability with its
//! power bit (roles_constants.h) and its texts (`tr!` keys).

/// One ability: its power bit, short and long texts.
pub struct Ability {
    pub bit: u32,
    pub name: &'static str,
    pub help: &'static str,
}

/// A set of abilities, its icon and its texts.
pub struct AbilitySet {
    pub icon: &'static str,
    pub name: &'static str,
    pub help: &'static str,
    pub abilities: Vec<Ability>,
}

pub fn catalogue() -> Vec<AbilitySet> {
    vec![
        AbilitySet {
            icon: "users",
            name: deeply_i18n::tr!("group-abilities-membership"),
            help: deeply_i18n::tr!("group-abilities-membership-help"),
            abilities: vec![
                Ability {
                    bit: 1,
                    name: deeply_i18n::tr!("group-ability-member-invite"),
                    help: deeply_i18n::tr!("group-ability-member-invite-help"),
                },
                Ability {
                    bit: 2,
                    name: deeply_i18n::tr!("group-ability-member-eject"),
                    help: deeply_i18n::tr!("group-ability-member-eject-help"),
                },
                Ability {
                    bit: 51,
                    name: deeply_i18n::tr!("group-ability-allow-ban"),
                    help: deeply_i18n::tr!("group-ability-allow-ban-help"),
                },
                Ability {
                    bit: 3,
                    name: deeply_i18n::tr!("group-ability-member-options"),
                    help: deeply_i18n::tr!("group-ability-member-options-help"),
                },
            ],
        },
        AbilitySet {
            icon: "crown",
            name: deeply_i18n::tr!("group-abilities-roles"),
            help: deeply_i18n::tr!("group-abilities-roles-help"),
            abilities: vec![
                Ability {
                    bit: 4,
                    name: deeply_i18n::tr!("group-ability-role-create"),
                    help: deeply_i18n::tr!("group-ability-role-create-help"),
                },
                Ability {
                    bit: 5,
                    name: deeply_i18n::tr!("group-ability-role-delete"),
                    help: deeply_i18n::tr!("group-ability-role-delete-help"),
                },
                Ability {
                    bit: 6,
                    name: deeply_i18n::tr!("group-ability-role-properties"),
                    help: deeply_i18n::tr!("group-ability-role-properties-help"),
                },
                Ability {
                    bit: 7,
                    name: deeply_i18n::tr!("group-ability-role-assign-member-limited"),
                    help: deeply_i18n::tr!("group-ability-role-assign-member-limited-help"),
                },
                Ability {
                    bit: 8,
                    name: deeply_i18n::tr!("group-ability-role-assign-member"),
                    help: deeply_i18n::tr!("group-ability-role-assign-member-help"),
                },
                Ability {
                    bit: 9,
                    name: deeply_i18n::tr!("group-ability-role-remove-member"),
                    help: deeply_i18n::tr!("group-ability-role-remove-member-help"),
                },
                Ability {
                    bit: 10,
                    name: deeply_i18n::tr!("group-ability-role-change-actions"),
                    help: deeply_i18n::tr!("group-ability-role-change-actions-help"),
                },
            ],
        },
        AbilitySet {
            icon: "id-filled",
            name: deeply_i18n::tr!("group-abilities-group-identity"),
            help: deeply_i18n::tr!("group-abilities-group-identity-help"),
            abilities: vec![Ability {
                bit: 11,
                name: deeply_i18n::tr!("group-ability-group-change-identity"),
                help: deeply_i18n::tr!("group-ability-group-change-identity-help"),
            }],
        },
        AbilitySet {
            icon: "map-2",
            name: deeply_i18n::tr!("group-abilities-parcel-management"),
            help: deeply_i18n::tr!("group-abilities-parcel-management-help"),
            abilities: vec![
                Ability {
                    bit: 12,
                    name: deeply_i18n::tr!("group-ability-land-deed"),
                    help: deeply_i18n::tr!("group-ability-land-deed-help"),
                },
                Ability {
                    bit: 13,
                    name: deeply_i18n::tr!("group-ability-land-release"),
                    help: deeply_i18n::tr!("group-ability-land-release-help"),
                },
                Ability {
                    bit: 14,
                    name: deeply_i18n::tr!("group-ability-land-set-sale-info"),
                    help: deeply_i18n::tr!("group-ability-land-set-sale-info-help"),
                },
                Ability {
                    bit: 15,
                    name: deeply_i18n::tr!("group-ability-land-divide-join"),
                    help: deeply_i18n::tr!("group-ability-land-divide-join-help"),
                },
            ],
        },
        AbilitySet {
            icon: "map-pin-filled",
            name: deeply_i18n::tr!("group-abilities-parcel-identity"),
            help: deeply_i18n::tr!("group-abilities-parcel-identity-help"),
            abilities: vec![
                Ability {
                    bit: 17,
                    name: deeply_i18n::tr!("group-ability-land-find-places"),
                    help: deeply_i18n::tr!("group-ability-land-find-places-help"),
                },
                Ability {
                    bit: 18,
                    name: deeply_i18n::tr!("group-ability-land-change-identity"),
                    help: deeply_i18n::tr!("group-ability-land-change-identity-help"),
                },
                Ability {
                    bit: 19,
                    name: deeply_i18n::tr!("group-ability-land-set-landing-point"),
                    help: deeply_i18n::tr!("group-ability-land-set-landing-point-help"),
                },
            ],
        },
        AbilitySet {
            icon: "adjustments-filled",
            name: deeply_i18n::tr!("group-abilities-parcel-settings"),
            help: deeply_i18n::tr!("group-abilities-parcel-settings-help"),
            abilities: vec![
                Ability {
                    bit: 20,
                    name: deeply_i18n::tr!("group-ability-land-change-media"),
                    help: deeply_i18n::tr!("group-ability-land-change-media-help"),
                },
                Ability {
                    bit: 21,
                    name: deeply_i18n::tr!("group-ability-land-edit"),
                    help: deeply_i18n::tr!("group-ability-land-edit-help"),
                },
                Ability {
                    bit: 22,
                    name: deeply_i18n::tr!("group-ability-land-options"),
                    help: deeply_i18n::tr!("group-ability-land-options-help"),
                },
                Ability {
                    bit: 46,
                    name: deeply_i18n::tr!("group-ability-land-change-environment"),
                    help: deeply_i18n::tr!("group-ability-land-change-environment-help"),
                },
            ],
        },
        AbilitySet {
            icon: "bolt-filled",
            name: deeply_i18n::tr!("group-abilities-parcel-powers"),
            help: deeply_i18n::tr!("group-abilities-parcel-powers-help"),
            abilities: vec![
                Ability {
                    bit: 23,
                    name: deeply_i18n::tr!("group-ability-land-allow-edit-land"),
                    help: deeply_i18n::tr!("group-ability-land-allow-edit-land-help"),
                },
                Ability {
                    bit: 24,
                    name: deeply_i18n::tr!("group-ability-land-allow-fly"),
                    help: deeply_i18n::tr!("group-ability-land-allow-fly-help"),
                },
                Ability {
                    bit: 25,
                    name: deeply_i18n::tr!("group-ability-land-allow-create"),
                    help: deeply_i18n::tr!("group-ability-land-allow-create-help"),
                },
                Ability {
                    bit: 26,
                    name: deeply_i18n::tr!("group-ability-land-allow-direct-teleport"),
                    help: deeply_i18n::tr!("group-ability-land-allow-direct-teleport-help"),
                },
                Ability {
                    bit: 28,
                    name: deeply_i18n::tr!("group-ability-land-allow-set-home"),
                    help: deeply_i18n::tr!("group-ability-land-allow-set-home-help"),
                },
                Ability {
                    bit: 41,
                    name: deeply_i18n::tr!("group-ability-land-allow-host-event"),
                    help: deeply_i18n::tr!("group-ability-land-allow-host-event-help"),
                },
            ],
        },
        AbilitySet {
            icon: "lock-filled",
            name: deeply_i18n::tr!("group-abilities-parcel-access"),
            help: deeply_i18n::tr!("group-abilities-parcel-access-help"),
            abilities: vec![
                Ability {
                    bit: 29,
                    name: deeply_i18n::tr!("group-ability-land-manage-allowed"),
                    help: deeply_i18n::tr!("group-ability-land-manage-allowed-help"),
                },
                Ability {
                    bit: 30,
                    name: deeply_i18n::tr!("group-ability-land-manage-banned"),
                    help: deeply_i18n::tr!("group-ability-land-manage-banned-help"),
                },
                Ability {
                    bit: 31,
                    name: deeply_i18n::tr!("group-ability-land-manage-passes"),
                    help: deeply_i18n::tr!("group-ability-land-manage-passes-help"),
                },
                Ability {
                    bit: 32,
                    name: deeply_i18n::tr!("group-ability-land-admin"),
                    help: deeply_i18n::tr!("group-ability-land-admin-help"),
                },
            ],
        },
        AbilitySet {
            icon: "photo-filled",
            name: deeply_i18n::tr!("group-abilities-parcel-content"),
            help: deeply_i18n::tr!("group-abilities-parcel-content-help"),
            abilities: vec![
                Ability {
                    bit: 48,
                    name: deeply_i18n::tr!("group-ability-land-return-group-owned"),
                    help: deeply_i18n::tr!("group-ability-land-return-group-owned-help"),
                },
                Ability {
                    bit: 33,
                    name: deeply_i18n::tr!("group-ability-land-return-group-set"),
                    help: deeply_i18n::tr!("group-ability-land-return-group-set-help"),
                },
                Ability {
                    bit: 34,
                    name: deeply_i18n::tr!("group-ability-land-return-non-group"),
                    help: deeply_i18n::tr!("group-ability-land-return-non-group-help"),
                },
                Ability {
                    bit: 35,
                    name: deeply_i18n::tr!("group-ability-land-gardening"),
                    help: deeply_i18n::tr!("group-ability-land-gardening-help"),
                },
            ],
        },
        AbilitySet {
            icon: "package",
            name: deeply_i18n::tr!("group-abilities-object-management"),
            help: deeply_i18n::tr!("group-abilities-object-management-help"),
            abilities: vec![
                Ability {
                    bit: 36,
                    name: deeply_i18n::tr!("group-ability-object-deed"),
                    help: deeply_i18n::tr!("group-ability-object-deed-help"),
                },
                Ability {
                    bit: 38,
                    name: deeply_i18n::tr!("group-ability-object-manipulate"),
                    help: deeply_i18n::tr!("group-ability-object-manipulate-help"),
                },
                Ability {
                    bit: 39,
                    name: deeply_i18n::tr!("group-ability-object-set-sale"),
                    help: deeply_i18n::tr!("group-ability-object-set-sale-help"),
                },
            ],
        },
        AbilitySet {
            icon: "coin-filled",
            name: deeply_i18n::tr!("group-abilities-accounting"),
            help: deeply_i18n::tr!("group-abilities-accounting-help"),
            abilities: vec![Ability {
                bit: 40,
                name: deeply_i18n::tr!("group-ability-accounting-accountable"),
                help: deeply_i18n::tr!("group-ability-accounting-accountable-help"),
            }],
        },
        AbilitySet {
            icon: "speakerphone",
            name: deeply_i18n::tr!("group-abilities-notices"),
            help: deeply_i18n::tr!("group-abilities-notices-help"),
            abilities: vec![
                Ability {
                    bit: 42,
                    name: deeply_i18n::tr!("group-ability-notices-send"),
                    help: deeply_i18n::tr!("group-ability-notices-send-help"),
                },
                Ability {
                    bit: 43,
                    name: deeply_i18n::tr!("group-ability-notices-receive"),
                    help: deeply_i18n::tr!("group-ability-notices-receive-help"),
                },
            ],
        },
        AbilitySet {
            icon: "messages-filled",
            name: deeply_i18n::tr!("group-abilities-chat"),
            help: deeply_i18n::tr!("group-abilities-chat-help"),
            abilities: vec![
                Ability {
                    bit: 16,
                    name: deeply_i18n::tr!("group-ability-join-group-chat"),
                    help: deeply_i18n::tr!("group-ability-join-group-chat-help"),
                },
                Ability {
                    bit: 27,
                    name: deeply_i18n::tr!("group-ability-join-voice-chat"),
                    help: deeply_i18n::tr!("group-ability-join-voice-chat-help"),
                },
                Ability {
                    bit: 37,
                    name: deeply_i18n::tr!("group-ability-moderate-group-chat"),
                    help: deeply_i18n::tr!("group-ability-moderate-group-chat-help"),
                },
            ],
        },
        AbilitySet {
            icon: "key-filled",
            name: deeply_i18n::tr!("group-abilities-experience-tools-experience"),
            help: deeply_i18n::tr!("group-abilities-experience-tools-experience-help"),
            abilities: vec![
                Ability {
                    bit: 49,
                    name: deeply_i18n::tr!("group-ability-experience-admin"),
                    help: deeply_i18n::tr!("group-ability-experience-admin-help"),
                },
                Ability {
                    bit: 50,
                    name: deeply_i18n::tr!("group-ability-experience-contributor"),
                    help: deeply_i18n::tr!("group-ability-experience-contributor-help"),
                },
            ],
        },
    ]
}
