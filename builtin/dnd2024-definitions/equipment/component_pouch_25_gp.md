---
splat_schema: 2
splat_id: 110756e4-fd60-5ef1-90e2-dd5e90eb40c2
splat_kind: item
splat_key: dnd2024:equipment/component_pouch_25_gp
splat_name: Component Pouch (25 GP)
splat_origin: dnd2024:reference/pages/097
splat_sources:
- SRD 5.2.1, pp. 97, https://www.dndbeyond.com/srd
---

# Component Pouch (25 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 97 -->

A Component Pouch is watertight and filled with compartments that hold all the free Material components of your spells.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/component_pouch_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_component_pouch_25_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` watertight ` | ` true ` |

#### Material components

<!-- splat:yaml path=splat_rule/parameters/material_components -->

```yaml
available: all_free_components_of_owner_spells
priced_components_included: false
rule:
  key: dnd2024:rules/spellcasting
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 25
> mass:
>   unit: lb
>   value: 2.0
> ```

