---
splat_schema: 2
splat_id: 121b2a25-1d9a-5f2b-bb67-8db7c489c366
splat_kind: item
splat_key: dnd2024:equipment/splint_armor
splat_name: Splint Armor
splat_origin: dnd2024:reference/pages/092
splat_sources:
- SRD 5.2.1, pp. 92, https://www.dndbeyond.com/srd
---

# Splint Armor


## Item

| Field | Value |
| --- | --- |
| Type | Armor |
| Rarity |  |
| Attunement |  |


## Source text

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 17,
    "add_dexterity_modifier": false,
    "strength_minimum": 15
  },
  "price": {
    "value": 200,
    "unit": "gp"
  },
  "mass": {
    "value": 60.0,
    "unit": "lb"
  },
  "category": "armor"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/splint_armor ` |
| ` implementation ` | ` dnd2024:equipment_splint_armor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` while_equipped ` | ` true ` |
| ` maximum_simultaneous_same_category ` | ` 1 ` |
| ` stacking_group ` | ` body_armor ` |

#### Armor

<!-- splat:values path=splat_rule/parameters/armor -->

| Field | Value |
| --- | --- |
| ` category ` | ` heavy armor ` |
| ` stealth_disadvantage ` | ` true ` |
| ` base_ac ` | ` 17 ` |
| ` add_dexterity_modifier ` | ` false ` |
| ` strength_minimum ` | ` 15 ` |

#### Equip

<!-- splat:yaml path=splat_rule/parameters/equip -->

```yaml
don:
  value: 10
  unit: minute
doff:
  value: 5
  unit: minute
```

#### Training

<!-- splat:yaml path=splat_rule/parameters/training -->

```yaml
without_training:
  d20_test_disadvantage_abilities:
  - strength
  - dexterity
  can_cast_spells: false
```

#### Stealth

<!-- splat:values path=splat_rule/parameters/stealth -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` skill ` | ` stealth ` |
| ` mode ` | ` disadvantage ` |

#### Strength penalty

<!-- splat:values path=splat_rule/parameters/strength_penalty -->

| Field | Value |
| --- | --- |
| ` if_strength_below ` | ` 15 ` |
| ` speed_reduction_ft ` | ` 10 ` |

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` actor ` | ` equipper ` |
| ` item ` | ` equipped_instance ` |
| ` inactive_if_not_equipped ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> armor:
>   add_dexterity_modifier: false
>   base_ac: 17
>   category: heavy armor
>   stealth_disadvantage: true
>   strength_minimum: 15
> price:
>   unit: gp
>   value: 200
> mass:
>   unit: lb
>   value: 60.0
> ```

