---
splat_schema: 2
splat_id: 0a4aac78-c87d-5f56-8366-6ab785b04fc5
splat_kind: item
splat_key: dnd2024:equipment/torch_1_cp
splat_name: Torch (1 CP)
splat_origin: dnd2024:reference/pages/100
splat_sources:
- SRD 5.2.1, pp. 100, https://www.dndbeyond.com/srd
---

# Torch (1 CP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 100 -->

A Torch burns for 1 hour, casting Bright Light in a 20-foot radius and Dim Light for an additional 20 feet. When you take the Attack action, you can attack with the Torch, using it as a Simple Melee weapon. On a hit, the target takes 1 Fire damage.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/torch_1_cp ` |
| ` implementation ` | ` dnd2024:equipment_torch_1_cp ` |

### Parameters

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` requires_lit ` | ` true ` |
| ` shape ` | ` radius ` |
| ` bright_ft ` | ` 20 ` |
| ` additional_dim_ft ` | ` 20 ` |

#### Burn duration

<!-- splat:values path=splat_rule/parameters/burn_duration -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` hour ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
action: attack
weapon_category: simple_melee
damage:
  amount: 1
  type: fire
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: cp
>   value: 1
> mass:
>   unit: lb
>   value: 1.0
> ```

