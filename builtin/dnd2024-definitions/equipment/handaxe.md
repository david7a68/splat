---
splat_schema: 2
splat_id: 34e4dda3-515b-589c-b53c-ff865efe81ce
splat_kind: item
splat_key: dnd2024:equipment/handaxe
splat_name: Handaxe
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Handaxe


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
    "expression": "1d6",
    "type": "slashing"
  },
  "properties_text": "Light, Thrown (Range 20/60)",
  "mastery": "vex",
  "weapon_category": "simple melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/handaxe ` |
| ` implementation ` | ` dnd2024:equipment_handaxe ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` simple_melee ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d6
type: slashing
amount:
  dice:
    count: 1
    sides: 6
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | trigger | draw_as_part_of_attack | attack_type | melee_weapon_uses_melee_ability |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` light ` | ` attack_with_light_weapon_during_own_attack_action ` |  |  |  |
| ` 1 ` | ` thrown ` |  | ` true ` | ` ranged ` | ` true ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_light ` |

##### Extra attack

<!-- splat:values path=splat_rule/parameters/properties/0/extra_attack -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` economy ` | ` bonus_action ` |
| ` timing ` | ` later_same_turn ` |
| ` weapon ` | ` different_light_weapon ` |
| ` damage_ability_modifier ` | ` negative_only ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/1/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_thrown ` |

##### Range

<!-- splat:yaml path=splat_rule/parameters/properties/1/range -->

```yaml
normal_ft: 20
long_ft: 60
normal_range_mode: normal
long_range_mode: disadvantage
beyond_long_range: cannot_attack
rule:
  key: dnd2024:rules/weapons_range
```

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` proficiency_bonus_if_proficient ` | ` true ` |
| ` base_ability ` | ` strength ` |
| ` normal_hit_damage_ability_modifier ` | ` attack_ability_modifier ` |

#### Mastery

<!-- splat:yaml path=splat_rule/parameters/mastery -->

```yaml
property: vex
rule:
  key: dnd2024:rules/weapons_vex
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_and_damage_creature
  effect:
    attacker_attack_roll_mode: advantage
    against: same_target
    count: 1
    expiry: attacker_next_turn_end
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
> weapon_category: simple melee
> damage:
>   expression: 1d6
>   type: slashing
> mass:
>   unit: lb
>   value: 2.0
> mastery: vex
> price:
>   unit: gp
>   value: 5
> properties_text: Light, Thrown (Range 20/60)
> ```

