---
splat_schema: 2
splat_id: 951565b3-3df2-56cf-9aa0-e5cba90132d7
splat_kind: item
splat_key: dnd2024:equipment/sling
splat_name: Sling
splat_origin: dnd2024:reference/pages/091
splat_sources:
- SRD 5.2.1, pp. 91, https://www.dndbeyond.com/srd
---

# Sling


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
    "expression": "1d4",
    "type": "bludgeoning"
  },
  "properties_text": "Ammunition (Range 30/120; Bullet)",
  "mastery": "slow",
  "weapon_category": "simple ranged",
  "price": {
    "value": 1,
    "unit": "sp"
  },
  "category": "weapon"
}







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/sling ` |
| ` implementation ` | ` dnd2024:equipment_sling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` simple_ranged ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
expression: 1d4
type: bludgeoning
amount:
  dice:
    count: 1
    sides: 4
  bonus: 0
```

#### Properties

<!-- splat:rows path=splat_rule/parameters/properties -->

| Row | id | expend_per_attack | draw_as_part_of_attack | free_hand_to_load_if_one_handed |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` ammunition ` | ` 1 ` | ` true ` | ` true ` |

##### Rule

<!-- splat:values path=splat_rule/parameters/properties/0/rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/weapons_ammunition ` |

##### Range

<!-- splat:yaml path=splat_rule/parameters/properties/0/range -->

```yaml
normal_ft: 30
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
| ` variant ` | ` sling_bullets ` |

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

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> weapon_category: simple ranged
> damage:
>   expression: 1d4
>   type: bludgeoning
> mastery: slow
> price:
>   unit: sp
>   value: 1
> properties_text: Ammunition (Range 30/120; Bullet)
> ```

