---
splat_schema: 1
splat_id: b1c3a794-12a5-5ad1-9427-e3f29854c512
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_15_persistent_rage
splat_name: 'Level 15: Persistent Rage'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Level 15: Persistent Rage

## Source text

<!-- SRD page 30 -->

When you roll Initiative, you can regain all ex-\
pended uses of Rage. After you regain uses of Rage\
in this way, you can’t do so again until you finish a\
Long Rest.\
In addition, your Rage is so fierce that it now lasts\
for 10 minutes without you needing to do anything\
to extend it from round to round. Your Rage ends\
early if you have the Unconscious condition (not just\
the Incapacitated condition) or don Heavy armor.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_15_persistent_rage ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_15_persistent_rage ` |

### Parameters

#### Initiative recovery

<!-- splat:yaml path=splat_rule/parameters/initiative_recovery -->

```yaml
optional: true
resource: rage
amount: all_expended
uses:
  maximum: 1
  recover:
    event: rest.long_rest_completed
    amount: all
```

#### Rage override

<!-- splat:yaml path=splat_rule/parameters/rage_override -->

```yaml
duration:
  minutes: 10
requires_round_extension: false
early_end_conditions:
- unconscious
early_end_armor: heavy
replaces_incapacitated_end_trigger: true
```
