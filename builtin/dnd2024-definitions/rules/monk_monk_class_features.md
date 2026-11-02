---
splat_schema: 1
splat_id: 71d76934-1fc8-50be-a15c-cca568917292
splat_kind: rule
splat_key: dnd2024:rules/monk_monk_class_features
splat_name: Monk Class Features
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49, 50
  url: https://www.dndbeyond.com/srd
---

# Monk Class Features

## Source text

<!-- SRD page 49 -->

As a Monk, you gain the following class features when you reach the specified Monk levels. These features are listed in the Monk Features table.

<!-- SRD page 50 -->

**Monk Features**

| Level | Proficiency Bonus | Class Features | Martial Arts | Focus Points | Unarmored Movement |
| --- | --- | --- | --- | --- | --- |
| 1 | +2 | Martial Arts, Unarmored Defense | 1d6 | — | — |
| 2 | +2 | Monk’s Focus, Unarmored Movement, Uncanny Metabolism | 1d6 | 2 | +10 ft. |
| 3 | +2 | Deflect Attacks, Monk Subclass | 1d6 | 3 | +10 ft. |
| 4 | +2 | Ability Score Improvement, Slow Fall | 1d6 | 4 | +10 ft. |
| 5 | +3 | Extra Attack, Stunning Strike | 1d8 | 5 | +10 ft. |
| 6 | +3 | Empowered Strikes, Subclass feature | 1d8 | 6 | +15 ft. |
| 7 | +3 | Evasion | 1d8 | 7 | +15 ft. |
| 8 | +3 | Ability Score Improvement | 1d8 | 8 | +15 ft. |
| 9 | +4 | Acrobatic Movement | 1d8 | 9 | +15 ft. |
| 10 | +4 | Heightened Focus, Self-Restoration | 1d8 | 10 | +20 ft. |
| 11 | +4 | Subclass feature | 1d10 | 11 | +20 ft. |
| 12 | +4 | Ability Score Improvement | 1d10 | 12 | +20 ft. |
| 13 | +5 | Deflect Energy | 1d10 | 13 | +20 ft. |
| 14 | +5 | Disciplined Survivor | 1d10 | 14 | +25 ft. |
| 15 | +5 | Perfect Focus | 1d10 | 15 | +25 ft. |
| 16 | +5 | Ability Score Improvement | 1d10 | 16 | +25 ft. |
| 17 | +6 | Subclass feature | 1d12 | 17 | +25 ft. |
| 18 | +6 | Superior Defense | 1d12 | 18 | +30 ft. |
| 19 | +6 | Epic Boon | 1d12 | 19 | +30 ft. |
| 20 | +6 | Body and Mind | 1d12 | 20 | +30 ft. |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/monk_monk_class_features ` |
| ` implementation ` | ` dnd2024:rules_monk_monk_class_features ` |

### Parameters

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/monk ` |

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
- monk_level_10_heightened_focus
- monk_level_10_self_restoration
- monk_level_13_deflect_energy
- monk_level_14_disciplined_survivor
- monk_level_15_perfect_focus
- monk_level_18_superior_defense
- monk_level_19_epic_boon
- monk_level_1_martial_arts
- monk_level_1_unarmored_defense
- monk_level_20_body_and_mind
- monk_level_2_monk_s_focus
- monk_level_2_unarmored_movement
- monk_level_2_uncanny_metabolism
- monk_level_3_deflect_attacks
- monk_level_3_monk_subclass
- monk_level_4_ability_score_improvement
- monk_level_4_slow_fall
- monk_level_5_extra_attack
- monk_level_5_stunning_strike
- monk_level_6_empowered_strikes
- monk_level_7_evasion
- monk_level_9_acrobatic_movement
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
| [Level 10: Heightened Focus](<../class_features/monk_level_10_heightened_focus.md> "dnd2024:class_features/monk_level_10_heightened_focus") | ` 10 ` |
| [Level 10: Self-Restoration](<../class_features/monk_level_10_self_restoration.md> "dnd2024:class_features/monk_level_10_self_restoration") | ` 10 ` |
| [Level 13: Deflect Energy](<../class_features/monk_level_13_deflect_energy.md> "dnd2024:class_features/monk_level_13_deflect_energy") | ` 13 ` |
| [Level 14: Disciplined Survivor](<../class_features/monk_level_14_disciplined_survivor.md> "dnd2024:class_features/monk_level_14_disciplined_survivor") | ` 14 ` |
| [Level 15: Perfect Focus](<../class_features/monk_level_15_perfect_focus.md> "dnd2024:class_features/monk_level_15_perfect_focus") | ` 15 ` |
| [Level 18: Superior Defense](<../class_features/monk_level_18_superior_defense.md> "dnd2024:class_features/monk_level_18_superior_defense") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/monk_level_19_epic_boon.md> "dnd2024:class_features/monk_level_19_epic_boon") | ` 19 ` |
| [Level 1: Martial Arts](<../class_features/monk_level_1_martial_arts.md> "dnd2024:class_features/monk_level_1_martial_arts") | ` 1 ` |
| [Level 1: Unarmored Defense](<../class_features/monk_level_1_unarmored_defense.md> "dnd2024:class_features/monk_level_1_unarmored_defense") | ` 1 ` |
| [Level 20: Body and Mind](<../class_features/monk_level_20_body_and_mind.md> "dnd2024:class_features/monk_level_20_body_and_mind") | ` 20 ` |
| [Level 2: Monk’s Focus](<../class_features/monk_level_2_monk_s_focus.md> "dnd2024:class_features/monk_level_2_monk_s_focus") | ` 2 ` |
| [Level 2: Unarmored Movement](<../class_features/monk_level_2_unarmored_movement.md> "dnd2024:class_features/monk_level_2_unarmored_movement") | ` 2 ` |
| [Level 2: Uncanny Metabolism](<../class_features/monk_level_2_uncanny_metabolism.md> "dnd2024:class_features/monk_level_2_uncanny_metabolism") | ` 2 ` |
| [Level 3: Deflect Attacks](<../class_features/monk_level_3_deflect_attacks.md> "dnd2024:class_features/monk_level_3_deflect_attacks") | ` 3 ` |
| [Level 3: Monk Subclass](<../class_features/monk_level_3_monk_subclass.md> "dnd2024:class_features/monk_level_3_monk_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/monk_level_4_ability_score_improvement.md> "dnd2024:class_features/monk_level_4_ability_score_improvement") | ` 4 ` |
| [Level 4: Slow Fall](<../class_features/monk_level_4_slow_fall.md> "dnd2024:class_features/monk_level_4_slow_fall") | ` 4 ` |
| [Level 5: Extra Attack](<../class_features/monk_level_5_extra_attack.md> "dnd2024:class_features/monk_level_5_extra_attack") | ` 5 ` |
| [Level 5: Stunning Strike](<../class_features/monk_level_5_stunning_strike.md> "dnd2024:class_features/monk_level_5_stunning_strike") | ` 5 ` |
| [Level 6: Empowered Strikes](<../class_features/monk_level_6_empowered_strikes.md> "dnd2024:class_features/monk_level_6_empowered_strikes") | ` 6 ` |
| [Level 7: Evasion](<../class_features/monk_level_7_evasion.md> "dnd2024:class_features/monk_level_7_evasion") | ` 7 ` |
| [Level 9: Acrobatic Movement](<../class_features/monk_level_9_acrobatic_movement.md> "dnd2024:class_features/monk_level_9_acrobatic_movement") | ` 9 ` |

#### Recurring feature levels

<!-- splat:values path=splat_rule/parameters/recurring_feature_levels -->

| Field | Value |
| --- | --- |
| ` ability_score_improvement ` | ` [4, 8, 12, 16] ` |
| ` subclass ` | ` [3, 6, 11, 17] ` |
