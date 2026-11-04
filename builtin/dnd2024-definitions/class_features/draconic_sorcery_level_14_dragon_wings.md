---
splat_schema: 2
splat_id: 8def3014-60c5-55ce-8487-eadf262a4a14
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_14_dragon_wings
splat_name: 'Level 14: Dragon Wings'
splat_origin: dnd2024:reference/pages/070
splat_sources:
- SRD 5.2.1, pp. 70, https://www.dndbeyond.com/srd
---

# Level 14: Dragon Wings

## Source text

<!-- SRD page 70 -->

As a Bonus Action, you can cause draconic wings to appear on your back. The wings last for 1 hour or until you dismiss them (no action required). For the duration, you have a Fly Speed of 60 feet.

Once you use this feature, you can’t use it again until you finish a Long Rest unless you spend 3 Sorcery Points (no action required) to restore your use of it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/draconic_sorcery_level_14_dragon_wings ` |
| ` implementation ` | ` dnd2024:class_features_draconic_sorcery_level_14_dragon_wings ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` cost ` | ` 1 ` |
| ` fly_speed_feet ` | ` 60 ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: sorcerer.dragon_wings
maximum: 1
recovery:
- event: rest.long_completed
  amount: all
```

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` hours ` | ` 1 ` |

#### Dismiss

<!-- splat:values path=splat_rule/parameters/dismiss -->

| Field | Value |
| --- | --- |
| ` action ` | ` none ` |

#### Recharge alternative

<!-- splat:yaml path=splat_rule/parameters/recharge_alternative -->

```yaml
action: none
resource_cost:
  id: sorcerer.sorcery_points
  amount: 3
restore_uses: 1
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` minimum_level ` | ` 14 ` |
