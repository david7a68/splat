---
splat_schema: 1
splat_id: d1fb3766-2184-5bd4-80a3-51f2f795af83
splat_kind: item
splat_key: dnd2024:items/apparatus_of_the_crab
splat_name: Apparatus of the Crab
splat_origin:
  key: dnd2024:reference/magic_item/apparatus_of_the_crab
splat_sources:
- title: SRD 5.2.1
  locator: pp. 210
  url: https://www.dndbeyond.com/srd
---

# Apparatus of the Crab

## Source text

<!-- SRD page 210 -->

Wondrous Item, Legendary

This item first appears to be a sealed iron barrel weighing 500 pounds. The barrel has a hidden catch, which can be found with a successful DC 20 Intelligence (Investigation) check. Releasing the catch unlocks a hatch at one end of the barrel, allowing two Medium or smaller creatures to crawl inside. Ten levers are set in a row at the far end, each in a neutral position, able to move up or down. When certain levers are used, the apparatus transforms to resemble a giant lobster. The Apparatus of the Crab is a Large object with the following statistics: AC 20; HP 200; Speed 30 ft., Swim 30 ft. (or 0 ft. for both if the legs aren’t extended); Immunity to Poison and Psychic damage.

To be used as a vehicle, the apparatus requires one pilot. While the apparatus’s hatch is closed, the compartment is airtight and watertight. The compartment holds enough air for 10 hours of breathing, divided by the number of breathing creatures inside.

The apparatus floats on water. It can also go underwater to a depth of 900 feet. Below that, the vehicle takes 2d6 Bludgeoning damage each minute from pressure.

A creature in the compartment can take a Utilize action to move as many as two of the apparatus’s levers up or down. After each use, a lever goes back to its neutral position. Each lever, from left to right, functions as shown in the Apparatus of the Crab Levers table.

**Apparatus of the Crab Levers**

| Lever | Up | Down |
| --- | --- | --- |
| 1 | Legs extend, allowing the apparatus to walk and swim. | Legs retract, reducing the apparatus’s Speed and Swim Speed to 0 and making it unable to benefit from bonuses to speed. |
| 2 | Forward window shutter opens. | Forward window shutter closes. |
| 3 | Side window shutters open (two per side). | Side window shutters close (two per side). |
| 4 | Two claws extend from the front side of the apparatus. | The claws retract. |
| 5 | Each extended claw makes the following melee attack: +8 to hit, reach 5 ft. Hit: 7 (2d6) Bludgeoning damage. | Each extended claw makes the following melee attack: +8 to hit, reach 5 ft. Hit: The target has the Grappled condition (escape DC 15). |
| 6 | The apparatus walks or swims forward provided its legs are extended. | The apparatus walks or swims backward provided its legs are extended. |
| 7 | The apparatus turns 90 degrees counterclockwise provided its legs are extended. | The apparatus turns 90 degrees clockwise provided its legs are extended. |
| 8 | Eyelike fixtures emit Bright Light in a 30-foot radius and Dim Light for an additional 30 feet. | The light turns off. |
| 9 | The apparatus sinks up to 20 feet if it’s in liquid. | The apparatus rises up to 20 feet if it’s in liquid. |
| 10 | The rear hatch unseals and opens. | The rear hatch closes and seals. |

Original layout: [[../../srd-5.2.1/pages/210|page 210]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/apparatus_of_the_crab ` |
| ` implementation ` | ` dnd2024:item_apparatus_of_the_crab ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weight_lb ` | ` 500 ` |
| ` size ` | ` large ` |
| ` ac ` | ` 20 ` |
| ` hp ` | ` 200 ` |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` pilot_required ` | ` 1 ` |
| ` floats ` | ` true ` |
| ` safe_depth_ft ` | ` 900 ` |

#### Hidden catch

<!-- splat:yaml path=splat_rule/parameters/hidden_catch -->

```yaml
check:
  ability: intelligence
  skill: investigation
  dc: 20
success: find_catch
release_catch: unlock_entry_hatch
```

#### Occupants

<!-- splat:values path=splat_rule/parameters/occupants -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 2 ` |
| ` sizes ` | ` ["tiny", "small", "medium"] ` |

#### Hatch closed

<!-- splat:yaml path=splat_rule/parameters/hatch_closed -->

```yaml
airtight: true
watertight: true
breathing_hours:
  op: divide
  args:
  - 10
  - binding: breathing_occupants
zero_breathing_occupants_air_depletion: false
```

#### Below safe depth

<!-- splat:yaml path=splat_rule/parameters/below_safe_depth -->

```yaml
interval:
  amount: 1
  unit: minute
damage:
  roll:
    expression: 2d6
    scope: use
  type: bludgeoning
```

#### Legs extended

<!-- splat:values path=splat_rule/parameters/legs_extended -->

| Field | Value |
| --- | --- |
| ` speed_ft ` | ` 30 ` |
| ` swim_speed_ft ` | ` 30 ` |

#### Legs retracted

<!-- splat:values path=splat_rule/parameters/legs_retracted -->

| Field | Value |
| --- | --- |
| ` speed_ft ` | ` 0 ` |
| ` swim_speed_ft ` | ` 0 ` |
| ` speed_bonuses_allowed ` | ` false ` |

#### Operate

<!-- splat:values path=splat_rule/parameters/operate -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` actor_location ` | ` inside_compartment ` |
| ` maximum_levers ` | ` 2 ` |
| ` positions ` | ` ["up", "down"] ` |
| ` reset_after_each_use ` | ` neutral ` |

#### Levers

<!-- splat:rows path=splat_rule/parameters/levers -->

| Row | number | scope | requires |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` |  |  |
| ` 1 ` | ` 2 ` |  |  |
| ` 2 ` | ` 3 ` |  |  |
| ` 3 ` | ` 4 ` |  |  |
| ` 4 ` | ` 5 ` | ` each_extended_claw ` |  |
| ` 5 ` | ` 6 ` |  | ` legs_extended ` |
| ` 6 ` | ` 7 ` |  | ` legs_extended ` |
| ` 7 ` | ` 8 ` |  |  |
| ` 8 ` | ` 9 ` |  | ` in_liquid ` |
| ` 9 ` | ` 10 ` |  |  |

##### Up

<!-- splat:values path=splat_rule/parameters/levers/0/up -->

| Field | Value |
| --- | --- |
| ` legs ` | ` extended ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/0/down -->

| Field | Value |
| --- | --- |
| ` legs ` | ` retracted ` |

##### Up

<!-- splat:values path=splat_rule/parameters/levers/1/up -->

| Field | Value |
| --- | --- |
| ` forward_window_shutter ` | ` open ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/1/down -->

| Field | Value |
| --- | --- |
| ` forward_window_shutter ` | ` closed ` |

##### Up

<!-- splat:values path=splat_rule/parameters/levers/2/up -->

| Field | Value |
| --- | --- |
| ` side_window_shutters ` | ` open ` |
| ` count_per_side ` | ` 2 ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/2/down -->

| Field | Value |
| --- | --- |
| ` side_window_shutters ` | ` closed ` |
| ` count_per_side ` | ` 2 ` |

##### Up

<!-- splat:values path=splat_rule/parameters/levers/3/up -->

| Field | Value |
| --- | --- |
| ` claws ` | ` extended ` |
| ` count ` | ` 2 ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/3/down -->

| Field | Value |
| --- | --- |
| ` claws ` | ` retracted ` |

##### Up

<!-- splat:yaml path=splat_rule/parameters/levers/4/up -->

```yaml
attack:
  kind: melee
  bonus: 8
  reach_ft: 5
  hit:
    damage:
      average: 7
      roll:
        expression: 2d6
        scope: attack
      type: bludgeoning
```

##### Down

<!-- splat:yaml path=splat_rule/parameters/levers/4/down -->

```yaml
attack:
  kind: melee
  bonus: 8
  reach_ft: 5
  hit:
    condition: grappled
    escape_dc: 15
```

##### Up

<!-- splat:values path=splat_rule/parameters/levers/5/up -->

| Field | Value |
| --- | --- |
| ` movement ` | ` forward ` |
| ` modes ` | ` ["walk", "swim"] ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/5/down -->

| Field | Value |
| --- | --- |
| ` movement ` | ` backward ` |
| ` modes ` | ` ["walk", "swim"] ` |

##### Up

<!-- splat:values path=splat_rule/parameters/levers/6/up -->

| Field | Value |
| --- | --- |
| ` turn_degrees ` | ` 90 ` |
| ` direction ` | ` counterclockwise ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/6/down -->

| Field | Value |
| --- | --- |
| ` turn_degrees ` | ` 90 ` |
| ` direction ` | ` clockwise ` |

##### Up

<!-- splat:yaml path=splat_rule/parameters/levers/7/up -->

```yaml
light:
  bright_radius_ft: 30
  dim_additional_ft: 30
```

##### Down

<!-- splat:values path=splat_rule/parameters/levers/7/down -->

| Field | Value |
| --- | --- |
| ` light ` | ` "off" ` |

##### Up

<!-- splat:values path=splat_rule/parameters/levers/8/up -->

| Field | Value |
| --- | --- |
| ` vertical_movement ` | ` sink ` |
| ` maximum_ft ` | ` 20 ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/8/down -->

| Field | Value |
| --- | --- |
| ` vertical_movement ` | ` rise ` |
| ` maximum_ft ` | ` 20 ` |

##### Up

<!-- splat:values path=splat_rule/parameters/levers/9/up -->

| Field | Value |
| --- | --- |
| ` rear_hatch ` | ` open_unsealed ` |

##### Down

<!-- splat:values path=splat_rule/parameters/levers/9/down -->

| Field | Value |
| --- | --- |
| ` rear_hatch ` | ` closed_sealed ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
