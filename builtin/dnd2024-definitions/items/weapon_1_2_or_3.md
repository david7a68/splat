---
splat_schema: 1
splat_id: 3c691f80-f94b-52ed-9fca-4da222a934ef
splat_kind: item
splat_key: dnd2024:items/weapon_1_2_or_3
splat_name: Weapon, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/weapon_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
---

# Weapon, +1, +2, or +3

## Source text

<!-- SRD page 253 -->

Weapon (Any Simple or Martial), Uncommon (+1), Rare\
(+2), or Very Rare (+3)\
You have a bonus to attack rolls and damage rolls\
made with this magic weapon. The bonus is deter-\
mined by the weapon’s rarity.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

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

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | rarity | attack_bonus | damage_bonus |
| --- | --- | --- | --- |
| ` 0 ` | ` uncommon ` | ` 1 ` | ` 1 ` |
| ` 1 ` | ` rare ` | ` 2 ` | ` 2 ` |
| ` 2 ` | ` very_rare ` | ` 3 ` | ` 3 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
