---
splat_schema: 1
splat_id: a5bcb9fa-4c3b-5e4a-be4d-fc4c65d7ddac
splat_kind: rule
splat_key: dnd2024:rules/armor_one_at_a_time
splat_name: One at a Time
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
---

# One at a Time

## Source text

<!-- SRD page 92 -->

A creature can wear only one suit of armor at a time\
and wield only one Shield at a time.\
Strength Stealth Weight Cost\
— Disadvantage 8 lb. 5 GP\
— — 10 lb. 10 GP\
— — 13 lb. 45 GP\
— — 12 lb. 10 GP\
— — 20 lb. 50 GP\
— Disadvantage 45 lb. 50 GP\
— — 20 lb. 400 GP\
— Disadvantage 40 lb. 750 GP\
— Disadvantage 40 lb. 30 GP\
Str 13 Disadvantage 55 lb. 75 GP\
Str 15 Disadvantage 60 lb. 200 GP\
Str 15 Disadvantage 65 lb. 1,500 GP\
— — 6 lb. 10 GP\

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

<!-- splat:rows path=splat_rule/parameters/armor_table -->

| Row | armor | strength_min | stealth_disadvantage | weight_lb | cost_gp |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` padded ` | ` null ` | ` true ` | ` 8 ` | ` 5 ` |
| ` 1 ` | ` leather ` | ` null ` | ` false ` | ` 10 ` | ` 10 ` |
| ` 2 ` | ` studded_leather ` | ` null ` | ` false ` | ` 13 ` | ` 45 ` |
| ` 3 ` | ` hide ` | ` null ` | ` false ` | ` 12 ` | ` 10 ` |
| ` 4 ` | ` chain_shirt ` | ` null ` | ` false ` | ` 20 ` | ` 50 ` |
| ` 5 ` | ` scale_mail ` | ` null ` | ` true ` | ` 45 ` | ` 50 ` |
| ` 6 ` | ` breastplate ` | ` null ` | ` false ` | ` 20 ` | ` 400 ` |
| ` 7 ` | ` half_plate ` | ` null ` | ` true ` | ` 40 ` | ` 750 ` |
| ` 8 ` | ` ring_mail ` | ` null ` | ` true ` | ` 40 ` | ` 30 ` |
| ` 9 ` | ` chain_mail ` | ` 13 ` | ` true ` | ` 55 ` | ` 75 ` |
| ` 10 ` | ` splint ` | ` 15 ` | ` true ` | ` 60 ` | ` 200 ` |
| ` 11 ` | ` plate ` | ` 15 ` | ` true ` | ` 65 ` | ` 1500 ` |
| ` 12 ` | ` shield ` | ` null ` | ` false ` | ` 6 ` | ` 10 ` |
