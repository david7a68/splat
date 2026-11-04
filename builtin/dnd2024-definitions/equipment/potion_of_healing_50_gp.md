---
splat_schema: 2
splat_id: 2c7b0386-7ca1-5cdc-be8c-2728de2684a9
splat_kind: item
splat_key: dnd2024:equipment/potion_of_healing_50_gp
splat_name: Potion of Healing (50 GP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Potion of Healing (50 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

This potion is a magic item. As a Bonus Action, you can drink it or administer it to another creature within 5 feet of yourself. The creature that drinks the magical red fluid in this vial regains 2d4 + 2 Hit Points.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/potion_of_healing_50_gp ` |
| ` implementation ` | ` dnd2024:equipment_potion_of_healing_50_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magical ` | ` true ` |

#### Use

<!-- splat:yaml path=splat_rule/parameters/use -->

```yaml
economy: bonus_action
consume: 1
target: self_or_creature_within_5_ft
operation: drink_or_administer
healing:
  dice:
    count: 2
    sides: 4
  bonus: 2
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.5 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 50
> mass:
>   unit: lb
>   value: 0.5
> ```

