---
splat_schema: 2
splat_id: a8e95bdd-9833-5cd5-8a85-1656c48edf56
splat_kind: item
splat_key: dnd2024:equipment/block_and_tackle_1_gp
splat_name: Block and Tackle (1 GP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Block and Tackle (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

A Block and Tackle allows you to hoist up to four times the weight you can normally lift.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/block_and_tackle_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_block_and_tackle_1_gp ` |

### Parameters

#### Hoist capacity

<!-- splat:yaml path=splat_rule/parameters/hoist_capacity -->

```yaml
op: multiply
operands:
- 4
- stat: normal_lift_capacity
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 1
> mass:
>   unit: lb
>   value: 5.0
> ```

