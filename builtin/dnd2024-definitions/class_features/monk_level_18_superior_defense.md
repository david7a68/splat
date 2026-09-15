---
splat_schema: 1
splat_id: 1459f9af-b62b-502a-a285-1112785c55ab
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_18_superior_defense
splat_name: 'Level 18: Superior Defense'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
---

# Level 18: Superior Defense

## Source text

<!-- SRD page 52 -->

At the start of your turn, you can expend 3 Focus\
Points to bolster yourself against harm for 1 min-\
ute or until you have the Incapacitated condition.\
During that time, you have Resistance to all damage\
except Force damage.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_18_superior_defense ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_18_superior_defense ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.turn.start ` |
| ` optional ` | ` true ` |
| ` ends_on ` | ` ["actor.incapacitated"] ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` monk.focus_points ` |
| ` amount ` | ` 3 ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` minutes ` | ` 1 ` |

#### Resistance

<!-- splat:values path=splat_rule/parameters/resistance -->

| Field | Value |
| --- | --- |
| ` all_damage_types ` | ` true ` |
| ` except ` | ` ["force"] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 18 ` |
