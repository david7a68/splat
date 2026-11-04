---
splat_schema: 2
splat_id: 040eb293-0e03-5fc0-a3cd-f1fc83aa3792
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
splat_name: 'Level 20: Greater Divine Intervention'
splat_origin: dnd2024:reference/pages/038
splat_sources:
- SRD 5.2.1, pp. 38, https://www.dndbeyond.com/srd
---

# Level 20: Greater Divine Intervention

## Source text

<!-- SRD page 38 -->

You can call on even more powerful divine intervention. When you use your Divine Intervention feature, you can choose Wish when you select a spell. If you do so, you can’t use Divine Intervention again until you finish 2d4 Long Rests.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_20_greater_divine_intervention ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_20_greater_divine_intervention ` |

### Parameters

#### Extends feature

<!-- splat:values path=splat_rule/parameters/extends_feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_10_divine_intervention ` |

#### Additional spell choice

<!-- splat:values path=splat_rule/parameters/additional_spell_choice -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wish ` |

#### If chosen

<!-- splat:yaml path=splat_rule/parameters/if_chosen -->

```yaml
divine_intervention_recovery:
  event: rest.long_completed
  required_count:
    dice:
      count: 2
      sides: 4
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 20 ` |
