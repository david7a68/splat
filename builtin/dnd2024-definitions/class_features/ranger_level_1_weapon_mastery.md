---
splat_schema: 1
splat_id: 2bac9887-e161-5870-b542-59c077890473
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/058
splat_sources:
- title: SRD 5.2.1
  locator: pp. 58, 59
  url: https://www.dndbeyond.com/srd
---

# Level 1: Weapon Mastery

## Source text

<!-- SRD page 58 -->

Your training with weapons allows you to use the\
mastery properties of two kinds of weapons of your\

<!-- SRD page 59 -->

choice with which you have proficiency, such as\
Longbows and Shortswords.\
Whenever you finish a Long Rest, you can change\
the kinds of weapons you chose. For example, you\
could switch to using the mastery properties of\
Scimitars and Longswords.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_1_weapon_mastery ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_1_weapon_mastery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grants ` | ` chosen_weapons_mastery_properties ` |

#### Mastery choice

<!-- splat:values path=splat_rule/parameters/mastery_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` category ` | ` weapon_kind ` |
| ` requires_proficiency ` | ` true ` |

#### Replace

<!-- splat:values path=splat_rule/parameters/replace -->

| Field | Value |
| --- | --- |
| ` event ` | ` rest.long_rest_completed ` |
| ` maximum ` | ` all ` |
