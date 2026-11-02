---
splat_schema: 1
splat_id: 83b18bd8-7cf2-5876-b671-37a806ea35b1
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_5_font_of_inspiration
splat_name: 'Level 5: Font of Inspiration'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
---

# Level 5: Font of Inspiration

## Source text

<!-- SRD page 32 -->

You now regain all your expended uses of Bardic Inspiration when you finish a Short or Long Rest.

In addition, you can expend a spell slot (no action required) to regain one expended use of Bardic Inspiration.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_5_font_of_inspiration ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_5_font_of_inspiration ` |

### Parameters

#### Resource recovery override

<!-- splat:values path=splat_rule/parameters/resource_recovery_override -->

| Field | Value |
| --- | --- |
| ` id ` | ` bard.bardic_inspiration ` |

##### Maximum

<!-- splat:yaml path=splat_rule/parameters/resource_recovery_override/maximum -->

```yaml
op: max
args:
- 1
- var: actor.abilities.charisma.modifier
```

##### Recovery

<!-- splat:table
path: splat_rule/parameters/resource_recovery_override/recovery
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Amount:
    path: amount
    type: symbol
-->

| Key | Event | Amount |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_completed ` | ` All ` |
| ` row_2 ` | ` rest.long_completed ` | ` All ` |

#### Conversion

<!-- splat:yaml path=splat_rule/parameters/conversion -->

```yaml
action: none
consume:
  resource: spell_slot
  count: 1
  minimum_level: 1
restore:
  resource: bard.bardic_inspiration
  amount: 1
  expended_only: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_level ` | ` 5 ` |
