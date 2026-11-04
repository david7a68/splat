---
splat_schema: 2
splat_id: b27f6b06-d9d0-55c8-ac07-deb3ea70bf57
splat_kind: item
splat_key: dnd2024:equipment/bedroll_1_gp
splat_name: Bedroll (1 GP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Bedroll (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

A Bedroll sleeps one Small or Medium creature. While in a Bedroll, you automatically succeed on saving throws against extreme cold (see “Gameplay Toolbox”).









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/bedroll_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_bedroll_1_gp ` |

### Parameters

#### Sleep capacity

<!-- splat:values path=splat_rule/parameters/sleep_capacity -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sizes ` | ` ["small", "medium"] ` |

#### While inside

<!-- splat:values path=splat_rule/parameters/while_inside -->

| Field | Value |
| --- | --- |
| ` saving_throw_scope ` | ` extreme_cold ` |
| ` result ` | ` automatic_success ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 7.0 ` |
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
>   value: 7.0
> ```

