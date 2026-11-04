---
splat_schema: 2
splat_id: 8f28f780-21f0-5845-824e-5145c40d7fe5
splat_kind: item
splat_key: dnd2024:equipment/musket
splat_name: Musket
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Musket


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
    "expression": "1d12",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 40/120; Bullet), Loading, Two-Handed",
  "mastery": "slow",
  "weapon_category": "martial ranged",
  "price": {
    "value": 500,
    "unit": "gp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/musket ` |
| ` implementation ` | ` dnd2024:equipment_musket ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_ranged ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d12
type: piercing
amount:
  dice:
    count: 1
    sides: 12
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
normal_ft: 40
long_ft: 120
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
| ` variant ` | ` firearm_bullets ` |

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
> weapon_category: martial ranged
> damage:
>   expression: 1d12
>   type: piercing
> mass:
>   unit: lb
>   value: 10.0
> mastery: slow
> price:
>   unit: gp
>   value: 500
> properties_text: Ammunition (Range 40/120; Bullet), Loading, Two-Handed
> ```

