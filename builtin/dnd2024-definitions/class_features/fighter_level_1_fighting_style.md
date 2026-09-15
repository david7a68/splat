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

You have honed your martial prowess and gain a\
Fighting Style feat of your choice (see “Feats”). De-\
fense is recommended.\
Whenever you gain a Fighter level, you can replace\
the feat you chose with a different Fighting Style\
feat.\
Second Weapon\
Wind Mastery\
2 3\
2 3\
2 3\
3 4\
3 4\
3 4\
3 4\
3 4\
3 4\
4 5\
4 5\
4 5\
4 5\
4 5\
4 5\
4 6\
4 6\
4 6\
4 6\
4 6\

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
