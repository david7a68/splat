---
splat_schema: 1
splat_id: 5ca55667-0040-5455-a5b2-3421baaf054a
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing
splat_name: 'Level 3: Dark One’s Blessing'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
---

# Level 3: Dark One’s Blessing

## Source text

<!-- SRD page 76 -->

When you reduce an enemy to 0 Hit Points, you gain\
Temporary Hit Points equal to your Charisma mod-\
ifier plus your Warlock level (minimum of 1 Tempo-\
rary Hit Point). You also gain this benefit if someone\
else reduces an enemy within 10 feet of you to 0 Hit\
Points.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing ` |
| ` implementation ` | ` dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing ` |

### Parameters

#### Triggers

<!-- splat:rows path=splat_rule/parameters/triggers -->

| Row | enemy_reduced_to_zero_by | enemy_distance_max_feet |
| --- | --- | --- |
| ` 0 ` | ` self ` |  |
| ` 1 ` | ` other ` | ` 10 ` |

#### Temporary hit points

<!-- splat:yaml path=splat_rule/parameters/temporary_hit_points -->

```yaml
op: max
args:
- 1
- op: add
  args:
  - stat: abilities.charisma.modifier
  - stat: classes.warlock.level
```
