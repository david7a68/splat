---
splat_schema: 2
splat_id: c8f45d11-5d90-568c-91e8-a9abcb3f39c7
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_18_survivor
splat_name: 'Level 18: Survivor'
splat_origin: dnd2024:reference/pages/049
splat_sources:
- SRD 5.2.1, pp. 49, https://www.dndbeyond.com/srd
---

# Level 18: Survivor

## Source text

<!-- SRD page 49 -->

You attain the pinnacle of resilience in battle, giving you these benefits.

Defy Death. You have Advantage on Death Saving Throws. Moreover, when you roll 18–20 on a Death Saving Throw, you gain the benefit of rolling a 20 on it.

Heroic Rally. At the start of each of your turns, you regain Hit Points equal to 5 plus your Constitution modifier if you are Bloodied and have at least 1 Hit Point.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/champion_level_18_survivor ` |
| ` implementation ` | ` dnd2024:class_features_champion_level_18_survivor ` |

### Parameters

#### Death saves

<!-- splat:values path=splat_rule/parameters/death_saves -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |
| ` natural_d20_results_count_as_20 ` | ` [18, 19, 20] ` |

#### Turn start healing

<!-- splat:yaml path=splat_rule/parameters/turn_start_healing -->

```yaml
requires:
  bloodied: true
  minimum_hit_points: 1
amount:
  op: add
  args:
  - 5
  - stat: abilities.constitution.modifier
```
