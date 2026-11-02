---
splat_schema: 1
splat_id: 95d7bee8-d399-51db-b328-2eb6cdc1d1c7
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_1_fighting_style
splat_name: 'Level 1: Fighting Style'
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
---

# Level 1: Fighting Style

## Source text

<!-- SRD page 47 -->

You have honed your martial prowess and gain a Fighting Style feat of your choice (see “Feats”). Defense is recommended.

Whenever you gain a Fighter level, you can replace the feat you chose with a different Fighting Style feat.

**Fighter Features**

| Level | Proficiency Bonus | Class Features | Second Wind | Weapon Mastery |
| --- | --- | --- | --- | --- |
| 1 | +2 | Fighting Style, Second Wind, Weapon Mastery | 2 | 3 |
| 2 | +2 | Action Surge (one use), Tactical Mind | 2 | 3 |
| 3 | +2 | Fighter Subclass | 2 | 3 |
| 4 | +2 | Ability Score Improvement | 3 | 4 |
| 5 | +3 | Extra Attack, Tactical Shift | 3 | 4 |
| 6 | +3 | Ability Score Improvement | 3 | 4 |
| 7 | +3 | Subclass feature | 3 | 4 |
| 8 | +3 | Ability Score Improvement | 3 | 4 |
| 9 | +4 | Indomitable (one use), Tactical Master | 3 | 4 |
| 10 | +4 | Subclass feature | 4 | 5 |
| 11 | +4 | Two Extra Attacks | 4 | 5 |
| 12 | +4 | Ability Score Improvement | 4 | 5 |
| 13 | +5 | Indomitable (two uses), Studied Attacks | 4 | 5 |
| 14 | +5 | Ability Score Improvement | 4 | 5 |
| 15 | +5 | Subclass feature | 4 | 5 |
| 16 | +5 | Ability Score Improvement | 4 | 6 |
| 17 | +6 | Action Surge (two uses), Indomitable (three uses) | 4 | 6 |
| 18 | +6 | Subclass feature | 4 | 6 |
| 19 | +6 | Epic Boon | 4 | 6 |
| 20 | +6 | Three Extra Attacks | 4 | 6 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_1_fighting_style ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_1_fighting_style ` |

### Parameters

#### Feat choice

<!-- splat:values path=splat_rule/parameters/feat_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` category ` | ` fighting_style ` |

#### Replace

<!-- splat:values path=splat_rule/parameters/replace -->

| Field | Value |
| --- | --- |
| ` event ` | ` fighter.level_gained ` |
| ` category ` | ` fighting_style ` |
| ` maximum ` | ` 1 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 1 ` |
