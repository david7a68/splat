---
splat_schema: 2
splat_id: 2fd061de-bedc-5b54-adcd-0f6dd4ea5d7f
splat_kind: rule
splat_key: dnd2024:rules/weapons_cleave
splat_name: Cleave
splat_origin: dnd2024:reference/pages/090
splat_sources:
- SRD 5.2.1, pp. 90, https://www.dndbeyond.com/srd
---

# Cleave

## Source text

<!-- SRD page 90 -->

If you hit a creature with a melee attack roll using this weapon, you can make a melee attack roll with the weapon against a second creature within 5 feet of the first that is also within your reach. On a hit, the second creature takes the weapon’s damage, but don’t add your ability modifier to that damage unless that modifier is negative. You can make this extra attack only once per turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_cleave ` |
| ` implementation ` | ` dnd2024:rules_weapons_cleave ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` melee_attack_hit ` | ` true ` |
| ` target_kind ` | ` creature ` |
| ` weapon ` | ` this_weapon ` |

#### Extra attack

<!-- splat:values path=splat_rule/parameters/extra_attack -->

| Field | Value |
| --- | --- |
| ` kind ` | ` melee ` |
| ` weapon ` | ` same_weapon ` |
| ` target ` | ` different_creature ` |
| ` target_distance_from_first_max_ft ` | ` 5 ` |
| ` target_within_attacker_reach ` | ` true ` |

#### Extra hit damage

<!-- splat:values path=splat_rule/parameters/extra_hit_damage -->

| Field | Value |
| --- | --- |
| ` weapon_damage ` | ` true ` |
| ` ability_modifier ` | ` only_if_negative ` |

#### Uses

<!-- splat:values path=splat_rule/parameters/uses -->

| Field | Value |
| --- | --- |
| ` max ` | ` 1 ` |
| ` period ` | ` turn ` |
