---
splat_schema: 2
splat_id: f2ee4569-e383-5e0e-9f73-63624c6494c4
splat_kind: item
splat_key: dnd2024:items/arrow_catching_shield
splat_name: Arrow-Catching Shield
splat_origin: dnd2024:reference/magic_item/arrow_catching_shield
splat_sources:
- SRD 5.2.1, pp. 211, https://www.dndbeyond.com/srd
---

# Arrow-Catching Shield


## Item

| Field | Value |
| --- | --- |
| Type | Armor |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 211 -->

Armor (Shield), Rare (Requires Attunement)

You gain a +2 bonus to Armor Class against ranged attack rolls while you wield this Shield. This bonus is in addition to the Shield’s normal bonus to AC.

Whenever an attacker makes a ranged attack roll against a target within 5 feet of you, you can take a Reaction to become the target of the attack instead.

Original layout: [[../../srd-5.2.1/pages/211|page 211]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/arrow_catching_shield ` |
| ` implementation ` | ` dnd2024:item_arrow_catching_shield ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` wielded ` |

#### Armor class

<!-- splat:values path=splat_rule/parameters/armor_class -->

| Field | Value |
| --- | --- |
| ` bonus ` | ` 2 ` |
| ` against ` | ` ranged_attack_roll ` |
| ` stacks_with_normal_shield ` | ` true ` |

#### Redirect

<!-- splat:values path=splat_rule/parameters/redirect -->

| Field | Value |
| --- | --- |
| ` action ` | ` reaction ` |
| ` trigger ` | ` ranged_attack_roll_against_target_within_5_feet ` |
| ` target_distance_ft ` | ` 5 ` |
| ` replace_target ` | ` self ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
