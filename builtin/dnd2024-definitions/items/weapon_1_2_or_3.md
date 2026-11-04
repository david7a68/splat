---
splat_schema: 2
splat_id: 3c691f80-f94b-52ed-9fca-4da222a934ef
splat_kind: item
splat_key: dnd2024:items/weapon_1_2_or_3
splat_name: Weapon, +1, +2, or +3
splat_origin: dnd2024:reference/magic_item/weapon_1_2_or_3
splat_sources:
- SRD 5.2.1, pp. 253, https://www.dndbeyond.com/srd
---

# Weapon, +1, +2, or +3


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Varies |
| Attunement | No |


## Source text

<!-- SRD page 253 -->

Weapon (Any Simple or Martial), Uncommon (+1), Rare (+2), or Very Rare (+3)

You have a bonus to attack rolls and damage rolls made with this magic weapon. The bonus is determined by the weapon’s rarity.

Original layout: [[../../srd-5.2.1/pages/253|page 253]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/weapon_1_2_or_3 ` |
| ` implementation ` | ` dnd2024:item_weapon_1_2_or_3 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` bonus_scope ` | ` attacks_with_this_weapon ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Rarity:
    path: rarity
    type: symbol
  Attack bonus:
    path: attack_bonus
    type: integer
  Damage bonus:
    path: damage_bonus
    type: integer
-->

| Key | Rarity | Attack bonus | Damage bonus |
| --- | --- | --- | --- |
| ` row_1 ` | ` Uncommon ` | ` 1 ` | ` 1 ` |
| ` row_2 ` | ` Rare ` | ` 2 ` | ` 2 ` |
| ` row_3 ` | ` Very rare ` | ` 3 ` | ` 3 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
