---
splat_schema: 1
splat_id: 35365b15-7c04-50ae-ab40-9ce9e654b024
splat_kind: rule_group
splat_key: dnd2024:subclasses/fighter_fighter_subclass_champion
splat_name: 'Fighter Subclass: Champion'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
---

# Fighter Subclass: Champion

## Source text

<!-- SRD page 49 -->

### Pursue Physical Excellence in Combat

A Champion focuses on the development of martial prowess in a relentless pursuit of victory. Champions combine rigorous training with physical excellence to deal devastating blows, withstand peril, and garner glory. Whether in athletic contests or bloody battle, Champions strive for the crown of the victor.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/champion_level_10_heroic_warrior ` |
| ` 1 ` | ` dnd2024:class_features/champion_level_15_superior_critical ` |
| ` 2 ` | ` dnd2024:class_features/champion_level_18_survivor ` |
| ` 3 ` | ` dnd2024:class_features/champion_level_3_improved_critical ` |
| ` 4 ` | ` dnd2024:class_features/champion_level_3_remarkable_athlete ` |
| ` 5 ` | ` dnd2024:class_features/champion_level_7_additional_fighting_style ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/fighter_fighter_subclass_champion ` |
| ` implementation ` | ` dnd2024:subclasses_fighter_fighter_subclass_champion ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` subclass ` | ` champion ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- champion_level_10_heroic_warrior
- champion_level_15_superior_critical
- champion_level_18_survivor
- champion_level_3_improved_critical
- champion_level_3_remarkable_athlete
- champion_level_7_additional_fighting_style
columns:
  Feature:
    path: feature
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Feature | Minimum class level |
| --- | --- |
| [Level 10: Heroic Warrior](<../class_features/champion_level_10_heroic_warrior.md> "dnd2024:class_features/champion_level_10_heroic_warrior") | ` 10 ` |
| [Level 15: Superior Critical](<../class_features/champion_level_15_superior_critical.md> "dnd2024:class_features/champion_level_15_superior_critical") | ` 15 ` |
| [Level 18: Survivor](<../class_features/champion_level_18_survivor.md> "dnd2024:class_features/champion_level_18_survivor") | ` 18 ` |
| [Level 3: Improved Critical](<../class_features/champion_level_3_improved_critical.md> "dnd2024:class_features/champion_level_3_improved_critical") | ` 3 ` |
| [Level 3: Remarkable Athlete](<../class_features/champion_level_3_remarkable_athlete.md> "dnd2024:class_features/champion_level_3_remarkable_athlete") | ` 3 ` |
| [Level 7: Additional Fighting Style](<../class_features/champion_level_7_additional_fighting_style.md> "dnd2024:class_features/champion_level_7_additional_fighting_style") | ` 7 ` |
