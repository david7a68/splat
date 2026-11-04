---
splat_schema: 2
splat_id: 07d470ab-d68e-5abe-b224-d36b2ba35380
splat_kind: item
splat_key: dnd2024:equipment/healer_s_kit_5_gp
splat_name: Healer’s Kit (5 GP)
splat_origin: dnd2024:reference/pages/097
splat_sources:
- SRD 5.2.1, pp. 97, https://www.dndbeyond.com/srd
---

# Healer’s Kit (5 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 97 -->

A Healer’s Kit has ten uses. As a Utilize action, you can expend one of its uses to stabilize an Unconscious creature that has 0 Hit Points without needing to make a Wisdom (Medicine) check.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/healer_s_kit_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_healer_s_kit_5_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 10 ` |

#### Use

<!-- splat:yaml path=splat_rule/parameters/use -->

```yaml
action: utilize
expend_uses: 1
target:
  condition: unconscious
  hit_points: 0
effect: stabilize
medicine_check_required: false
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 5
> mass:
>   unit: lb
>   value: 3.0
> ```

