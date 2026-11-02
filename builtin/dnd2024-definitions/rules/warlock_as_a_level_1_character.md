---
splat_schema: 1
splat_id: e661b70d-3f6f-5fcb-ba3e-3506cf801465
splat_kind: rule
splat_key: dnd2024:rules/warlock_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 70 -->

•  Gain all the traits in the Core Warlock Traits table.

•  Gain the Warlock’s level 1 features, which are listed in the Warlock Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rule_warlock_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry_mode ` | ` level_one_character ` |
| ` grant_all_core_traits ` | ` true ` |
| ` grant_features_at_class_level ` | ` 1 ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/warlock ` |

#### Embedded feature progression source

<!-- splat:values path=splat_rule/parameters/embedded_feature_progression_source -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/warlock_progression ` |

#### Level one features

<!-- splat:rows path=splat_rule/parameters/level_one_features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/warlock_level_1_eldritch_invocations ` |
| ` 1 ` | ` dnd2024:class_features/warlock_level_1_pact_magic ` |
