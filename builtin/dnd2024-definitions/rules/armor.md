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

<!-- splat:rows path=splat_rule/parameters/armor -->

| Row | id | category | base_ac | dexterity_cap | add_dexterity | strength_minimum | stealth_disadvantage | weight_lb | cost_gp |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` padded_armor ` | ` light ` | ` 11 ` | ` null ` | ` true ` | ` null ` | ` true ` | ` 8 ` | ` 5 ` |
| ` 1 ` | ` leather_armor ` | ` light ` | ` 11 ` | ` null ` | ` true ` | ` null ` | ` false ` | ` 10 ` | ` 10 ` |
| ` 2 ` | ` studded_leather_armor ` | ` light ` | ` 12 ` | ` null ` | ` true ` | ` null ` | ` false ` | ` 13 ` | ` 45 ` |
| ` 3 ` | ` hide_armor ` | ` medium ` | ` 12 ` | ` 2 ` | ` true ` | ` null ` | ` false ` | ` 12 ` | ` 10 ` |
| ` 4 ` | ` chain_shirt ` | ` medium ` | ` 13 ` | ` 2 ` | ` true ` | ` null ` | ` false ` | ` 20 ` | ` 50 ` |
| ` 5 ` | ` scale_mail ` | ` medium ` | ` 14 ` | ` 2 ` | ` true ` | ` null ` | ` true ` | ` 45 ` | ` 50 ` |
| ` 6 ` | ` breastplate ` | ` medium ` | ` 14 ` | ` 2 ` | ` true ` | ` null ` | ` false ` | ` 20 ` | ` 400 ` |
| ` 7 ` | ` half_plate_armor ` | ` medium ` | ` 15 ` | ` 2 ` | ` true ` | ` null ` | ` true ` | ` 40 ` | ` 750 ` |
| ` 8 ` | ` ring_mail ` | ` heavy ` | ` 14 ` | ` 0 ` | ` false ` | ` null ` | ` true ` | ` 40 ` | ` 30 ` |
| ` 9 ` | ` chain_mail ` | ` heavy ` | ` 16 ` | ` 0 ` | ` false ` | ` 13 ` | ` true ` | ` 55 ` | ` 75 ` |
| ` 10 ` | ` splint_armor ` | ` heavy ` | ` 17 ` | ` 0 ` | ` false ` | ` 15 ` | ` true ` | ` 60 ` | ` 200 ` |
| ` 11 ` | ` plate_armor ` | ` heavy ` | ` 18 ` | ` 0 ` | ` false ` | ` 15 ` | ` true ` | ` 65 ` | ` 1500 ` |

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
