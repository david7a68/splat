---
splat_schema: 1
splat_id: b547b909-2f9d-594a-9dec-54430a9a77e9
splat_kind: item
splat_key: dnd2024:equipment/oil_1_sp
splat_name: Oil (1 SP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98, 99
  url: https://www.dndbeyond.com/srd
---

# Oil (1 SP)

## Source text

<!-- SRD page 98 -->

You can douse a creature, object, or space with Oil\
or use it as fuel, as detailed below.\
Dousing a Creature or an Object. When you\
take the Attack action, you can replace one of your\
attacks with throwing an Oil flask. Target one crea-\
ture or object within 20 feet of yourself. The target\
must succeed on a Dexterity saving throw (DC 8\
plus your Dexterity modifier and Proficiency Bonus)\
or be covered in oil. If the target takes Fire damage\
before the oil dries (after 1 minute), the target takes\
an extra 5 Fire damage from burning oil.\
Dousing a Space. You can take the Utilize ac-\
tion to pour an Oil flask on level ground to cover a\
5-foot-square area within 5 feet of yourself. If lit,\
the oil burns until the end of the turn 2 rounds from\
when the oil was lit (or 12 seconds) and deals 5 Fire\
damage to any creature that enters the area or ends\
its turn there. A creature can take this damage only\
once per turn.\

<!-- SRD page 99 -->

Fuel. Oil serves as fuel for Lamps and Lanterns.\
Once lit, a flask of Oil burns for 6 hours in a Lamp or\
Lantern. That duration doesn’t need to be consecu-\
tive; you can extinguish the burning Oil (as a Utilize\
action) and rekindle it again until it has burned for a\
total of 6 hours.\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` sp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/oil_1_sp ` |
| ` implementation ` | ` dnd2024:equipment_oil_1_sp ` |

### Parameters

#### Douse target

<!-- splat:yaml path=splat_rule/parameters/douse_target -->

```yaml
economy: replace_one_attack
action: attack
consume: 1
target:
  count: 1
  kinds:
  - creature
  - object
  range_ft: 20
save:
  ability: dexterity
  dc:
    op: add
    operands:
    - 8
    - ability_modifier: dexterity
    - stat: proficiency_bonus
  success: no_effect
  failure:
    coating_duration:
      value: 1
      unit: minute
    trigger: target_takes_fire_damage
    extra_damage:
      amount: 5
      type: fire
```

#### Douse area

<!-- splat:yaml path=splat_rule/parameters/douse_area -->

```yaml
action: utilize
consume: 1
area:
  shape: square
  side_ft: 5
  range_ft: 5
  level_ground: true
requires_ignition: true
duration:
  rounds: 2
  expiry: turn_end
  outside_combat_seconds: 12
triggers:
- creature_enters
- creature_turn_end_in_area
damage:
  amount: 5
  type: fire
maximum_applications_per_creature_per_turn: 1
```

#### Fuel

<!-- splat:yaml path=splat_rule/parameters/fuel -->

```yaml
compatible:
- key: dnd2024:equipment/lamp_5_sp
- key: dnd2024:equipment/lantern_bullseye_10_gp
- key: dnd2024:equipment/lantern_hooded_5_gp
burn_time_per_flask:
  value: 6
  unit: hour
time_is_cumulative: true
extinguish_action: utilize
rekindling_allowed: true
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |
