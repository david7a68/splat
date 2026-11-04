---
splat_schema: 2
splat_id: 5ca55667-0040-5455-a5b2-3421baaf054a
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing
splat_name: 'Level 3: Dark One’s Blessing'
splat_origin: dnd2024:reference/pages/076
splat_sources:
- SRD 5.2.1, pp. 76, https://www.dndbeyond.com/srd
---

# Level 3: Dark One’s Blessing

## Source text

<!-- SRD page 76 -->

When you reduce an enemy to 0 Hit Points, you gain Temporary Hit Points equal to your Charisma modifier plus your Warlock level (minimum of 1 Temporary Hit Point). You also gain this benefit if someone else reduces an enemy within 10 feet of you to 0 Hit Points.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing ` |
| ` implementation ` | ` dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing ` |

### Parameters

#### Triggers

<!-- splat:table
path: splat_rule/parameters/triggers
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Enemy reduced to zero by:
    path: enemy_reduced_to_zero_by
    type: symbol
  Enemy distance max feet:
    path: enemy_distance_max_feet
    type: integer
    optional: true
-->

| Key | Enemy reduced to zero by | Enemy distance max feet |
| --- | --- | --- |
| ` row_1 ` | ` Self ` |  |
| ` row_2 ` | ` Other ` | ` 10 ` |

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
