---
splat_schema: 1
splat_id: fc0703ea-5fa2-5f10-8ebd-bfa9f9873944
splat_kind: rule
splat_key: dnd2024:rules/weapons
splat_name: Weapons
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
---

# Weapons

## Source text

<!-- SRD page 89 -->

The Weapons table in this section shows the game’s\
main weapons. The table lists the cost and weight of\
each weapon, as well as the following details:\
Category. Every weapon falls into a category: Sim-\
ple or Martial. Weapon proficiencies are usually\
tied to one of these categories. For example, you\
might have proficiency with Simple weapons.\
Melee or Ranged. A weapon is classified as either\
Melee or Ranged. A Melee weapon is used to at-\
tack a target within 5 feet, whereas a Ranged\
weapon is used to attack at a greater distance.\
Damage. The table lists the amount of damage a\
weapon deals when an attacker hits with it as well\
as the type of that damage.\
Properties. Any properties a weapon has are listed\
in the Properties column. Each property is de-\
fined in the “Properties” section.\
Mastery. Each weapon has a mastery property,\
which is defined in the “Mastery Properties” sec-\
tion. To use that property, you must have a feature\
that lets you use it.\
sEllinG EquipMEnt\
Equipment fetches half its cost when sold. In con-\
trast, trade goods and valuables—like gems and art\
objects—retain their full value in the marketplace.\
“Magic Items” has prices for magic items.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons ` |
| ` implementation ` | ` dnd2024:rule_weapons ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_categories ` | ` ["simple", "martial"] ` |
| ` weapon_kinds ` | ` ["melee", "ranged"] ` |
| ` base_melee_reach_feet ` | ` 5 ` |
| ` damage_on_hit_source ` | ` weapon_damage_and_type ` |
| ` properties_source ` | ` weapon_definition ` |

#### Mastery

<!-- splat:values path=splat_rule/parameters/mastery -->

| Field | Value |
| --- | --- |
| ` requires_feature_unlock ` | ` true ` |

#### Selling

<!-- splat:values path=splat_rule/parameters/selling -->

| Field | Value |
| --- | --- |
| ` equipment_fraction_of_purchase_cost ` | ` 0.5 ` |
| ` trade_goods_and_valuables_fraction ` | ` 1 ` |
| ` magic_item_prices ` | ` magic_item_rules ` |
