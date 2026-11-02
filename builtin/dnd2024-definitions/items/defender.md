---
splat_schema: 1
splat_id: 584405e4-63b8-5ad4-995a-61f0e6091339
splat_kind: item
splat_key: dnd2024:items/defender
splat_name: Defender
splat_origin:
  key: dnd2024:reference/magic_item/defender
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
---

# Defender

## Source text

<!-- SRD page 218 -->

Weapon (Any Melee Weapon), Legendary (Requires Attunement)

You gain a +3 bonus to attack rolls and damage rolls made with this magic weapon.

The first time you attack with the weapon on each of your turns, you can transfer some or all of the weapon’s bonus to your Armor Class. For example, you could reduce the bonus to your attack rolls and damage rolls to +1 and gain a +2 bonus to Armor Class. The adjusted bonuses remain in effect until the start of your next turn, although you must hold the weapon to gain a bonus to AC from it.

Original layout: [[../../srd-5.2.1/pages/218|page 218]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/defender ` |
| ` implementation ` | ` dnd2024:item_defender ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_bonus ` | ` 3 ` |
| ` damage_bonus ` | ` 3 ` |
| ` attunement_required ` | ` true ` |

#### Allocation

<!-- splat:yaml path=splat_rule/parameters/allocation -->

```yaml
event: first_attack_with_item_on_own_turn
choice:
  integer_min: 0
  integer_max: 3
ac_bonus: chosen_amount
attack_and_damage_bonus:
  subtract:
  - 3
  - chosen_amount
duration_end: actor.next_turn_start
ac_requires_holding: true
```
