---
splat_schema: 1
splat_id: 040600d5-0135-51ea-8bcf-08fa07854c03
splat_kind: rule
splat_key: dnd2024:rules/weapons_heavy
splat_name: Heavy
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
---

# Heavy

## Source text

<!-- SRD page 89 -->

You have Disadvantage on attack rolls with a Heavy\
weapon if it’s a Melee weapon and your Strength\
score isn’t at least 13 or if it’s a Ranged weapon and\
your Dexterity score isn’t at least 13.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_heavy ` |
| ` implementation ` | ` dnd2024:rules_weapons_heavy ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_property ` | ` heavy ` |

#### Disadvantage cases

<!-- splat:table
path: splat_rule/parameters/disadvantage_cases
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Weapon category:
    path: weapon_category
    type: symbol
  Strength below:
    path: strength_below
    type: integer
    optional: true
  Dexterity below:
    path: dexterity_below
    type: integer
    optional: true
-->

| Key | Weapon category | Strength below | Dexterity below |
| --- | --- | --- | --- |
| ` row_1 ` | ` Melee ` | ` 13 ` |  |
| ` row_2 ` | ` Ranged ` |  | ` 13 ` |
