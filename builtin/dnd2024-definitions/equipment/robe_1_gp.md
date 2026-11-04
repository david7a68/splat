---
splat_schema: 2
splat_id: 891bccc3-1366-53c0-b1a5-ce5494bda5e5
splat_kind: item
splat_key: dnd2024:equipment/robe_1_gp
splat_name: Robe (1 GP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Robe (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

A Robe has vocational or ceremonial significance. Some events and locations admit only people wearing a Robe bearing certain colors or symbols.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/robe_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_robe_1_gp ` |

### Parameters

#### Access requirement

<!-- splat:yaml path=splat_rule/parameters/access_requirement -->

```yaml
authority: dm
scope:
- event
- location
eligible_if_wearing: true
required_colors:
  type: choice
  authority: dm
required_symbols:
  type: choice
  authority: dm
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4.0 ` |
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
>   value: 4.0
> ```

