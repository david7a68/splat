---
splat_schema: 1
splat_id: 6bb212a0-e1a9-5a31-a937-8be18be81053
splat_kind: rule
splat_key: dnd2024:rules/ranger_ranger_class_features
splat_name: Ranger Class Features
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
---

# Ranger Class Features

## Source text

<!-- SRD page 57 -->

As a Ranger, you gain the following class features when you reach the specified Ranger levels. These features are listed in the Ranger Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/ranger_ranger_class_features ` |
| ` implementation ` | ` dnd2024:rules_ranger_ranger_class_features ` |

### Parameters

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/ranger ` |

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` level_basis ` | ` class_level ` |
| ` unlock_when ` | ` class_level_gte_feature_level ` |
| ` from ` | ` class_progression ` |

#### Feature unlocks

<!-- splat:table
path: splat_rule/parameters/feature_unlocks
key: Feature
order:
- ranger_level_10_tireless
- ranger_level_13_relentless_hunter
- ranger_level_14_nature_s_veil
- ranger_level_17_precise_hunter
- ranger_level_18_feral_senses
- ranger_level_19_epic_boon
- ranger_level_1_favored_enemy
- ranger_level_1_spellcasting
- ranger_level_1_weapon_mastery
- ranger_level_20_foe_slayer
- ranger_level_2_deft_explorer
- ranger_level_2_fighting_style
- ranger_level_3_ranger_subclass
- ranger_level_4_ability_score_improvement
- ranger_level_5_extra_attack
- ranger_level_6_roving
- ranger_level_9_expertise
columns:
  Feature:
    path: feature
    type: reference
  Class level:
    path: class_level
    type: integer
-->

| Feature | Class level |
| --- | --- |
| [Level 10: Tireless](<../class_features/ranger_level_10_tireless.md> "dnd2024:class_features/ranger_level_10_tireless") | ` 10 ` |
| [Level 13: Relentless Hunter](<../class_features/ranger_level_13_relentless_hunter.md> "dnd2024:class_features/ranger_level_13_relentless_hunter") | ` 13 ` |
| [Level 14: Nature’s Veil](<../class_features/ranger_level_14_nature_s_veil.md> "dnd2024:class_features/ranger_level_14_nature_s_veil") | ` 14 ` |
| [Level 17: Precise Hunter](<../class_features/ranger_level_17_precise_hunter.md> "dnd2024:class_features/ranger_level_17_precise_hunter") | ` 17 ` |
| [Level 18: Feral Senses](<../class_features/ranger_level_18_feral_senses.md> "dnd2024:class_features/ranger_level_18_feral_senses") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/ranger_level_19_epic_boon.md> "dnd2024:class_features/ranger_level_19_epic_boon") | ` 19 ` |
| [Level 1: Favored Enemy](<../class_features/ranger_level_1_favored_enemy.md> "dnd2024:class_features/ranger_level_1_favored_enemy") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/ranger_level_1_spellcasting.md> "dnd2024:class_features/ranger_level_1_spellcasting") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/ranger_level_1_weapon_mastery.md> "dnd2024:class_features/ranger_level_1_weapon_mastery") | ` 1 ` |
| [Level 20: Foe Slayer](<../class_features/ranger_level_20_foe_slayer.md> "dnd2024:class_features/ranger_level_20_foe_slayer") | ` 20 ` |
| [Level 2: Deft Explorer](<../class_features/ranger_level_2_deft_explorer.md> "dnd2024:class_features/ranger_level_2_deft_explorer") | ` 2 ` |
| [Level 2: Fighting Style](<../class_features/ranger_level_2_fighting_style.md> "dnd2024:class_features/ranger_level_2_fighting_style") | ` 2 ` |
| [Level 3: Ranger Subclass](<../class_features/ranger_level_3_ranger_subclass.md> "dnd2024:class_features/ranger_level_3_ranger_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/ranger_level_4_ability_score_improvement.md> "dnd2024:class_features/ranger_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Extra Attack](<../class_features/ranger_level_5_extra_attack.md> "dnd2024:class_features/ranger_level_5_extra_attack") | ` 5 ` |
| [Level 6: Roving](<../class_features/ranger_level_6_roving.md> "dnd2024:class_features/ranger_level_6_roving") | ` 6 ` |
| [Level 9: Expertise](<../class_features/ranger_level_9_expertise.md> "dnd2024:class_features/ranger_level_9_expertise") | ` 9 ` |
