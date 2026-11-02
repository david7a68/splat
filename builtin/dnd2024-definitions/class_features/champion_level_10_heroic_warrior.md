---
splat_schema: 1
splat_id: 6cee6c8b-6b86-52a4-8cb8-da705b531ca2
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_10_heroic_warrior
splat_name: 'Level 10: Heroic Warrior'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
---

# Level 10: Heroic Warrior

## Source text

<!-- SRD page 49 -->

The thrill of battle drives you toward victory. During combat, you can give yourself Heroic Inspiration whenever you start your turn without it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/champion_level_10_heroic_warrior ` |
| ` implementation ` | ` dnd2024:class_features_champion_level_10_heroic_warrior ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional_grant ` | ` heroic_inspiration ` |

#### When

<!-- splat:values path=splat_rule/parameters/when -->

| Field | Value |
| --- | --- |
| ` combat_active ` | ` true ` |
| ` heroic_inspiration ` | ` false ` |
