---
splat_schema: 2
splat_id: a151446c-de15-554e-87c3-b8d5ded116c4
splat_kind: item
splat_key: dnd2024:equipment/light_crossbow
splat_name: Light Crossbow
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Light Crossbow


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity |  |
| Attunement |  |


## Source text

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 80/320; Bolt), Loading, Two-Handed",
  "mastery": "slow",
  "weapon_category": "simple ranged",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 5.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/light_crossbow ` |
| ` implementation ` | ` dnd2024:equipment_light_crossbow ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` simple_ranged ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d8
type: piercing
amount:
  dice:
    count: 1
    sides: 8
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | expend_per_attack | draw_as_part_of_attack | free_hand_to_load_if_one_handed | ammunition_limit | per | ignores_extra_attacks | hands_to_attack |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` ammunition ` | ` 1 ` | ` true ` | ` true ` |  |  |  |  |
| ` 1 ` | ` loading ` |  |  |  | ` 1 ` | ` ["action", "bonus_action", "reaction"] ` | ` true ` |  |
| ` 2 ` | ` two_handed ` |  |  |  |  |  |  | ` 2 ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_ammunition ` |

##### Range

<!-- splat:yaml path=splat_rule/parameters/properties/0/range -->

```yaml
normal_ft: 80
long_ft: 320
normal_range_mode: normal
long_range_mode: disadvantage
beyond_long_range: cannot_attack
rule:
  key: dnd2024:rules/weapons_range
```

##### Ammunition

<!-- splat:values path=splat_rule/parameters/properties/0/ammunition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/ammunition_varies ` |
| ` variant ` | ` bolts ` |

##### Recovery

<!-- splat:yaml path=splat_rule/parameters/properties/0/recovery -->

```yaml
duration:
  value: 1
  unit: minute
timing: after_fight
fraction: 0.5
rounding: down
unrecovered: lost
```

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/1/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_loading ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/2/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_two_handed ` |

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` proficiency_bonus_if_proficient ` | ` true ` |
| ` base_ability ` | ` dexterity ` |
| ` normal_hit_damage_ability_modifier ` | ` attack_ability_modifier ` |

#### Mastery

<!-- splat:yaml path=splat_rule/parameters/mastery -->

```yaml
property: slow
rule:
  key: dnd2024:rules/weapons_slow
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_and_damage_creature
  optional: true
  speed_reduction_ft: 10
  expiry: attacker_next_turn_start
  maximum_stacked_reduction_ft: 10
```

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
> weapon_category: simple ranged
> damage:
>   expression: 1d8
>   type: piercing
> mass:
>   unit: lb
>   value: 5.0
> mastery: slow
> price:
>   unit: gp
>   value: 25
> properties_text: Ammunition (Range 80/320; Bolt), Loading, Two-Handed
> ```

