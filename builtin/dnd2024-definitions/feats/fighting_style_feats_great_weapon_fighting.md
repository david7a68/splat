---
splat_schema: 1
splat_id: 5ed192df-06c6-57b6-872a-dae32109eb4c
splat_kind: rule
splat_key: dnd2024:feats/fighting_style_feats_great_weapon_fighting
splat_name: Great Weapon Fighting
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
---

# Great Weapon Fighting

## Source text

<!-- SRD page 88 -->

Fighting Style Feat (Prerequisite: Fighting Style\
Feature)\
When you roll damage for an attack you make with\
a Melee weapon that you are holding with two\
hands, you can treat any 1 or 2 on a damage die as a\
3. The weapon must have the Two-Handed or Versa-\
tile property to gain this benefit.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/fighting_style_feats_great_weapon_fighting ` |
| ` implementation ` | ` dnd2024:feats_fighting_style_feats_great_weapon_fighting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` fighting_style ` |
| ` optional_damage_die_floor ` | ` 3 ` |
| ` scope ` | ` attack_damage_dice ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` feature ` | ` fighting_style ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` melee ` |
| ` held_with_hands ` | ` 2 ` |
| ` weapon_property_any ` | ` ["two_handed", "versatile"] ` |
