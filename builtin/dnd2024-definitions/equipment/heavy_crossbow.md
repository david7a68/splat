---
splat_schema: 1
splat_id: 2617805c-1c9e-5706-b9e2-3c769e36b321
splat_kind: item
splat_key: dnd2024:equipment/heavy_crossbow
splat_name: Heavy Crossbow
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
---

# Heavy Crossbow

## Source text

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed",
  "mastery": "push",
  "weapon_category": "martial ranged",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 18.0,
    "unit": "lb"
  },
  "category": "weapon"
}

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` properties_text ` | ` Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed ` |
| ` mastery ` | ` push ` |
| ` weapon_category ` | ` martial ranged ` |
| ` category ` | ` weapon ` |

### Damage

<!-- splat:values path=splat_item/damage -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d10 ` |
| ` type ` | ` piercing ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 50 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 18.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/heavy_crossbow ` |
| ` implementation ` | ` dnd2024:equipment_heavy_crossbow ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` martial_ranged ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d10
type: piercing
amount:
  dice:
    count: 1
    sides: 10
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | expend_per_attack | draw_as_part_of_attack | free_hand_to_load_if_one_handed | required_ability | minimum_score | below_minimum_attack_mode | ammunition_limit | per | ignores_extra_attacks | hands_to_attack |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` ammunition ` | ` 1 ` | ` true ` | ` true ` |  |  |  |  |  |  |  |
| ` 1 ` | ` heavy ` |  |  |  | ` dexterity ` | ` 13 ` | ` disadvantage ` |  |  |  |  |
| ` 2 ` | ` loading ` |  |  |  |  |  |  | ` 1 ` | ` ["action", "bonus_action", "reaction"] ` | ` true ` |  |
| ` 3 ` | ` two_handed ` |  |  |  |  |  |  |  |  |  | ` 2 ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_ammunition ` |

##### Range

<!-- splat:yaml path=splat_rule/parameters/properties/0/range -->

```yaml
normal_ft: 100
long_ft: 400
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
| ` key ` | ` dnd2024:rules/weapons_heavy ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/2/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_loading ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/3/rule -->

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
property: push
rule:
  key: dnd2024:rules/weapons_push
requires_weapon_mastery_feature_for_this_weapon: true
automatic_from_weapon_proficiency: false
parameters:
  trigger: hit_creature
  optional: true
  maximum_target_size: large
  push_maximum_ft: 10
  direction: straight_away_from_attacker
```

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` actor ` | ` equipper ` |
| ` item ` | ` equipped_instance ` |
| ` inactive_if_not_equipped ` | ` true ` |
