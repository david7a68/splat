---
splat_schema: 1
splat_id: 444170bd-6766-5fc5-b386-ecded380d0e0
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/054
splat_sources:
- title: SRD 5.2.1
  locator: pp. 54
  url: https://www.dndbeyond.com/srd
---

# Level 1: Weapon Mastery

## Source text

<!-- SRD page 54 -->

Your training with weapons allows you to use the\
mastery properties of two kinds of weapons of your\
choice with which you have proficiency, such as\
Longswords and Javelins.\
Whenever you finish a Long Rest, you can change\
the kinds of weapons you chose. For example, you\
could switch to using the mastery properties of Hal-\
berds and Flails.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_1_weapon_mastery ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_1_weapon_mastery ` |

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
