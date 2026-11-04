---
splat_schema: 2
splat_id: 95d524ef-f1e5-5022-b1f5-8060e6aaf1d8
splat_kind: rule
splat_key: dnd2024:rules/warlock_warlock_class_features
splat_name: Warlock Class Features
splat_origin: dnd2024:reference/pages/070
splat_sources:
- SRD 5.2.1, pp. 70, https://www.dndbeyond.com/srd
---

# Warlock Class Features

## Source text

<!-- SRD page 70 -->

As a Warlock, you gain the following class features when you reach the specified Warlock levels. These features are listed in the Warlock Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_warlock_class_features ` |
| ` implementation ` | ` dnd2024:rules_warlock_warlock_class_features ` |

### Parameters

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/warlock ` |

#### Grant when class level reached

<!-- splat:table
path: splat_rule/parameters/grant_when_class_level_reached
key: Feature
order:
- warlock_level_11_mystic_arcanum
- warlock_level_19_epic_boon
- warlock_level_1_eldritch_invocations
- warlock_level_1_pact_magic
- warlock_level_20_eldritch_master
- warlock_level_2_magical_cunning
- warlock_level_3_warlock_subclass
- warlock_level_4_ability_score_improvement
- warlock_level_9_contact_patron
columns:
  Feature:
    path: feature
    type: reference
  Level:
    path: level
    type: integer
-->

| Feature | Level |
| --- | --- |
| [Level 11: Mystic Arcanum](<../class_features/warlock_level_11_mystic_arcanum.md> "dnd2024:class_features/warlock_level_11_mystic_arcanum") | ` 11 ` |
| [Level 19: Epic Boon](<../class_features/warlock_level_19_epic_boon.md> "dnd2024:class_features/warlock_level_19_epic_boon") | ` 19 ` |
| [Level 1: Eldritch Invocations](<../class_features/warlock_level_1_eldritch_invocations.md> "dnd2024:class_features/warlock_level_1_eldritch_invocations") | ` 1 ` |
| [Level 1: Pact Magic](<../class_features/warlock_level_1_pact_magic.md> "dnd2024:class_features/warlock_level_1_pact_magic") | ` 1 ` |
| [Level 20: Eldritch Master](<../class_features/warlock_level_20_eldritch_master.md> "dnd2024:class_features/warlock_level_20_eldritch_master") | ` 20 ` |
| [Level 2: Magical Cunning](<../class_features/warlock_level_2_magical_cunning.md> "dnd2024:class_features/warlock_level_2_magical_cunning") | ` 2 ` |
| [Level 3: Warlock Subclass](<../class_features/warlock_level_3_warlock_subclass.md> "dnd2024:class_features/warlock_level_3_warlock_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/warlock_level_4_ability_score_improvement.md> "dnd2024:class_features/warlock_level_4_ability_score_improvement") | ` 4 ` |
| [Level 9: Contact Patron](<../class_features/warlock_level_9_contact_patron.md> "dnd2024:class_features/warlock_level_9_contact_patron") | ` 9 ` |
