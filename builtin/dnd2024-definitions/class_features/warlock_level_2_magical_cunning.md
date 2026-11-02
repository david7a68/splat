---
splat_schema: 1
splat_id: e5202fd8-e03e-50fe-b1db-92c78964c015
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_2_magical_cunning
splat_name: 'Level 2: Magical Cunning'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
---

# Level 2: Magical Cunning

## Source text

<!-- SRD page 72 -->

You can perform an esoteric rite for 1 minute. At the end of it, you regain expended Pact Magic spell slots but no more than a number equal to half your maximum (round up). Once you use this feature, you can’t do so again until you finish a Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warlock_level_2_magical_cunning ` |
| ` implementation ` | ` dnd2024:class_features_warlock_level_2_magical_cunning ` |

### Parameters

#### Activity duration

<!-- splat:values path=splat_rule/parameters/activity_duration -->

| Field | Value |
| --- | --- |
| ` minutes ` | ` 1 ` |

#### On completion

<!-- splat:yaml path=splat_rule/parameters/on_completion -->

```yaml
recover_resource: pact_magic_spell_slots
maximum_recovered:
  op: ceil
  args:
  - op: divide
    args:
    - stat: pact_magic.maximum_slots
    - 2
```

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 1
recover:
  event: rest.long_rest_completed
  amount: all
```
