---
splat_schema: 2
splat_id: a102cd30-ca0e-5c7e-8276-e374c171bf82
splat_kind: item
splat_key: dnd2024:equipment/barrel_2_gp
splat_name: Barrel (2 GP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Barrel (2 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

A Barrel holds up to 40 gallons of liquid or up to 4 cubic feet of dry goods.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/barrel_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_barrel_2_gp ` |

### Parameters

#### Capacity

##### Alternative capacities

<!-- splat:table
path: splat_rule/parameters/capacity/alternative_capacities
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Liquid gallons:
    path: liquid_gallons
    type: integer
    optional: true
  Dry volume ft3:
    path: dry_volume_ft3
    type: integer
    optional: true
-->

| Key | Liquid gallons | Dry volume ft3 |
| --- | --- | --- |
| ` row_1 ` | ` 40 ` |  |
| ` row_2 ` |  | ` 4 ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 70.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 2
> mass:
>   unit: lb
>   value: 70.0
> ```

