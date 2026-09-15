---
splat_schema: 1
splat_id: c2d97ced-3600-5a0b-9643-d39884784fca
splat_kind: rule
splat_key: dnd2024:rules/armor
splat_name: Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
---

# Armor

## Source text

<!-- SRD page 92 -->

The Armor table lists the game’s main armor. The\
table includes the cost and weight of armor, as well\
as the following details:\
Category. Every type of armor falls into a category:\
Light, Medium, or Heavy. The category determines\
how long it takes to don or doff the armor (as\
shown in the table).\
Armor Class (AC). The table’s Armor Class column\
tells you what your base AC is when you wear a\
type of armor. For example, if you wear Leather\
Armor, your base AC is 11 plus your Dexterity\
modifier, whereas your AC is 16 in Chain Mail.\
Strength. If the table shows a Strength score in the\
Strength column for an armor type, that armor\
reduces the wearer’s speed by 10 feet unless the\
wearer has a Strength score equal to or higher\
than the listed score.\
Stealth. If the table shows “Disadvantage” in the\
Stealth column for an armor type, the wearer has\
Disadvantage on Dexterity (Stealth) checks.\
Armor\
Armor Armor Class (AC)\
Light Armor (1 Minute to Don or Doff)\
Padded Armor 11 + Dex modifier\
Leather Armor 11 + Dex modifier\
Studded Leather Armor 12 + Dex modifier\
Medium Armor (5 Minutes to Don and 1 Minute to Doff)\
Hide Armor 12 + Dex modifier (max 2)\
Chain Shirt 13 + Dex modifier (max 2)\
Scale Mail 14 + Dex modifier (max 2)\
Breastplate 14 + Dex modifier (max 2)\
Half Plate Armor 15 + Dex modifier (max 2)\
Heavy Armor (10 Minutes to Don and 5 Minutes to Doff)\
Ring Mail 14\
Chain Mail 16\
Splint Armor 17\
Plate Armor 18\
Shield (Utilize Action to Don or Doff)\
Shield +2\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/armor ` |
| ` implementation ` | ` dnd2024:rules_armor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` insufficient_strength_speed_penalty_feet ` | ` 10 ` |

#### Armor

<!-- splat:table
path: splat_rule/parameters/armor
key: Variant
order:
- padded_armor
- leather_armor
- studded_leather_armor
- hide_armor
- chain_shirt
- scale_mail
- breastplate
- half_plate_armor
- ring_mail
- chain_mail
- splint_armor
- plate_armor
columns:
  Variant:
    path: id
    type: symbol
  Category:
    path: category
    type: symbol
  Base ac:
    path: base_ac
    type: integer
  Dexterity cap:
    path: dexterity_cap
    type: integer
    nullable: true
  Add dexterity:
    path: add_dexterity
    type: boolean
  Strength minimum:
    path: strength_minimum
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

| Variant | Category | Base ac | Dexterity cap | Add dexterity | Strength minimum | Stealth disadvantage | Weight lb | Cost gp |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` Padded armor ` | ` Light ` | ` 11 ` | null | true | null | true | ` 8 ` | ` 5 ` |
| ` Leather armor ` | ` Light ` | ` 11 ` | null | true | null | false | ` 10 ` | ` 10 ` |
| ` Studded leather armor ` | ` Light ` | ` 12 ` | null | true | null | false | ` 13 ` | ` 45 ` |
| ` Hide armor ` | ` Medium ` | ` 12 ` | ` 2 ` | true | null | false | ` 12 ` | ` 10 ` |
| ` Chain shirt ` | ` Medium ` | ` 13 ` | ` 2 ` | true | null | false | ` 20 ` | ` 50 ` |
| ` Scale mail ` | ` Medium ` | ` 14 ` | ` 2 ` | true | null | true | ` 45 ` | ` 50 ` |
| ` Breastplate ` | ` Medium ` | ` 14 ` | ` 2 ` | true | null | false | ` 20 ` | ` 400 ` |
| ` Half plate armor ` | ` Medium ` | ` 15 ` | ` 2 ` | true | null | true | ` 40 ` | ` 750 ` |
| ` Ring mail ` | ` Heavy ` | ` 14 ` | ` 0 ` | false | null | true | ` 40 ` | ` 30 ` |
| ` Chain mail ` | ` Heavy ` | ` 16 ` | ` 0 ` | false | ` 13 ` | true | ` 55 ` | ` 75 ` |
| ` Splint armor ` | ` Heavy ` | ` 17 ` | ` 0 ` | false | ` 15 ` | true | ` 60 ` | ` 200 ` |
| ` Plate armor ` | ` Heavy ` | ` 18 ` | ` 0 ` | false | ` 15 ` | true | ` 65 ` | ` 1500 ` |

#### Don doff minutes

<!-- splat:yaml path=splat_rule/parameters/don_doff_minutes -->

```yaml
light:
  don: 1
  doff: 1
medium:
  don: 5
  doff: 1
heavy:
  don: 10
  doff: 5
```

#### Shield

<!-- splat:values path=splat_rule/parameters/shield -->

| Field | Value |
| --- | --- |
| ` ac_bonus ` | ` 2 ` |
| ` don_action ` | ` utilize ` |
| ` doff_action ` | ` utilize ` |
| ` weight_lb ` | ` 6 ` |
| ` cost_gp ` | ` 10 ` |
