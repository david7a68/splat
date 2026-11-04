---
splat_schema: 2
splat_id: a5bcb9fa-4c3b-5e4a-be4d-fc4c65d7ddac
splat_kind: rule
splat_key: dnd2024:rules/armor_one_at_a_time
splat_name: One at a Time
splat_origin: dnd2024:reference/pages/092
splat_sources:
- SRD 5.2.1, pp. 92, https://www.dndbeyond.com/srd
---

# One at a Time

## Source text

<!-- SRD page 92 -->

A creature can wear only one suit of armor at a time and wield only one Shield at a time.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/armor_one_at_a_time ` |
| ` implementation ` | ` dnd2024:rules_armor_one_at_a_time ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` worn_armor_suits_max ` | ` 1 ` |
| ` wielded_shields_max ` | ` 1 ` |

#### Armor table

<!-- splat:table
path: splat_rule/parameters/armor_table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
- row_12
- row_13
columns:
  Key:
    type: key
  Armor:
    path: armor
    type: symbol
  Strength min:
    path: strength_min
    type: integer
    nullable: true
  Stealth disadvantage:
    path: stealth_disadvantage
    type: boolean
  Weight lb:
    path: weight_lb
    type: integer
  Cost gp:
    path: cost_gp
    type: integer
-->

| Key | Armor | Strength min | Stealth disadvantage | Weight lb | Cost gp |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Padded ` | null | true | ` 8 ` | ` 5 ` |
| ` row_2 ` | ` Leather ` | null | false | ` 10 ` | ` 10 ` |
| ` row_3 ` | ` Studded leather ` | null | false | ` 13 ` | ` 45 ` |
| ` row_4 ` | ` Hide ` | null | false | ` 12 ` | ` 10 ` |
| ` row_5 ` | ` Chain shirt ` | null | false | ` 20 ` | ` 50 ` |
| ` row_6 ` | ` Scale mail ` | null | true | ` 45 ` | ` 50 ` |
| ` row_7 ` | ` Breastplate ` | null | false | ` 20 ` | ` 400 ` |
| ` row_8 ` | ` Half plate ` | null | true | ` 40 ` | ` 750 ` |
| ` row_9 ` | ` Ring mail ` | null | true | ` 40 ` | ` 30 ` |
| ` row_10 ` | ` Chain mail ` | ` 13 ` | true | ` 55 ` | ` 75 ` |
| ` row_11 ` | ` Splint ` | ` 15 ` | true | ` 60 ` | ` 200 ` |
| ` row_12 ` | ` Plate ` | ` 15 ` | true | ` 65 ` | ` 1500 ` |
| ` row_13 ` | ` Shield ` | null | false | ` 6 ` | ` 10 ` |
