---
splat_schema: 2
splat_id: 03285678-23b9-549a-b63d-4683f772e315
splat_kind: item
splat_key: dnd2024:items/feather_token
splat_name: Feather Token
splat_origin: dnd2024:reference/magic_item/feather_token
splat_sources:
- SRD 5.2.1, pp. 221, https://www.dndbeyond.com/srd
---

# Feather Token


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rarity Varies |
| Attunement | No |


## Source text

<!-- SRD page 221 -->

Wondrous Item, Rarity Varies

This object looks like a feather. Different types of feather tokens exist, each with a different single-use effect. The GM chooses the kind of token or determines it randomly by rolling on the Feather Tokens table. The type of token determines its rarity.

Anchor (Uncommon). You can take a Magic action to touch the token to a boat or ship. For the next 24 hours, the vessel can’t be moved by any means. Touching the token to the vessel again ends the effect. When the effect ends, the token disappears.

Bird (Rare). You can take a Magic action to toss the token 5 feet into the air. The token disappears and an enormous, multicolored bird takes its place. The bird has the statistics of a Roc, but it can’t attack. It obeys your simple commands and can carry up to 500 pounds while flying at its maximum speed (16 miles per hour for a maximum of 144 miles per day, with a 1-hour rest for every 3 hours of flying) or 1,000 pounds at half that speed. The bird disappears after flying its maximum distance for a day or if it drops to 0 Hit Points. You can dismiss the bird as a Magic action.

Fan (Uncommon). If you are on a boat or ship, you can take a Magic action to toss the token up to 10 feet in the air. The token disappears, and a giant flapping fan takes its place. The fan floats and creates a strong wind. This wind can fill the sails of one ship, increasing its speed by 5 miles per hour for 8 hours. You can dismiss the fan as a Magic action.

Swan Boat (Rare). You can take a Magic action to touch the token to a body of water at least 60 feet in diameter. The token disappears, and a 50-foot-long, 20-foot-wide boat shaped like a swan takes its place. The boat is self-propelled and moves across water at a speed of 6 miles per hour. You can take a Magic action while on the boat to command it to move or to turn up to 90 degrees. The boat remains for 24 hours and then disappears. You can dismiss the boat as a Magic action.

Tree (Uncommon). You must be outdoors to use this token. You can take a Magic action to touch it to an unoccupied space on the ground. The token disappears, and in its place a nonmagical oak tree springs into existence. The tree is 60 feet tall and has a 5-foot-diameter trunk, and its branches at the top spread out in a 20-foot radius.

Whip (Rare). You can take a Magic action to throw the token to a point within 10 feet of yourself. The token disappears, and a floating whip takes its place. You can then take a Bonus Action to make a melee spell attack against a creature within 10 feet of the whip, with an attack bonus of +9. On a hit, the target takes 1d6 + 5 Force damage.

As a Bonus Action, you can direct the whip to fly up to 20 feet and repeat the attack against a creature within 10 feet of the whip. The whip disappears after 1 hour, when you take a Magic action to dismiss it, or when you die or have the Incapacitated condition.

**Feather Tokens**

| 1d100 | Token | Rarity |
| --- | --- | --- |
| 01–20 | Anchor | Uncommon |
| 21–35 | Bird | Rare |
| 36–50 | Fan | Uncommon |
| 51–65 | Swan boat | Rare |
| 66–90 | Tree | Uncommon |
| 91–00 | Whip | Rare |

Original layout: [[../../srd-5.2.1/pages/221|page 221]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/feather_token ` |
| ` implementation ` | ` dnd2024:item_feather_token ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` single_use ` | ` true ` |
| ` selection ` | ` gm_choice_or_roll ` |
| ` variant_die ` | ` 100 ` |
| ` attunement_required ` | ` false ` |

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | name | roll | rarity | action | target | range | consumed_on | toss_height_ft | ends_on | requires | toss_height_max_ft | floating | wind | ship_count | sailing_speed_bonus_mph | dismiss_action | throw_range_ft |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` anchor ` | ` [1, 20] ` | ` uncommon ` | ` magic ` | ` boat_or_ship ` | ` touch ` | ` effect_end ` |  |  |  |  |  |  |  |  |  |  |
| ` 1 ` | ` bird ` | ` [21, 35] ` | ` rare ` | ` magic ` |  |  | ` activation ` | ` 5 ` | ` ["daily_maximum_distance_flown", "zero_hp", "magic_action_dismissal"] ` |  |  |  |  |  |  |  |  |
| ` 2 ` | ` fan ` | ` [36, 50] ` | ` uncommon ` | ` magic ` |  |  | ` activation ` |  |  | ` aboard_boat_or_ship ` | ` 10 ` | ` true ` | ` strong ` | ` 1 ` | ` 5 ` | ` magic ` |  |
| ` 3 ` | ` swan_boat ` | ` [51, 65] ` | ` rare ` | ` magic ` |  | ` touch ` | ` activation ` |  |  |  |  |  |  |  |  | ` magic ` |  |
| ` 4 ` | ` tree ` | ` [66, 90] ` | ` uncommon ` | ` magic ` |  | ` touch ` | ` activation ` |  |  | ` outdoors ` |  |  |  |  |  |  |  |
| ` 5 ` | ` whip ` | ` [91, 100] ` | ` rare ` | ` magic ` |  |  | ` activation ` |  | ` ["magic_action_dismissal", "user_death", "user_incapacitated"] ` |  |  | ` true ` |  |  |  |  | ` 10 ` |

##### Effect

<!-- splat:yaml path=splat_rule/parameters/variants/0/effect -->

```yaml
immovable_by_any_means: true
duration:
  amount: 24
  unit: hours
early_end: token_touches_vessel_again
```

##### Summon

<!-- splat:values path=splat_rule/parameters/variants/1/summon -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/roc ` |

##### Overrides

<!-- splat:yaml path=splat_rule/parameters/variants/1/overrides -->

```yaml
can_attack: false
obeys: simple_commands
flight:
  normal_capacity_lb: 500
  normal_speed_mph: 16
  maximum_distance_per_day_miles: 144
  rest_hours: 1
  rest_after_flight_hours: 3
  maximum_capacity_lb: 1000
  over_normal_capacity_speed_multiplier: 0.5
```

##### Duration

<!-- splat:values path=splat_rule/parameters/variants/2/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 8 ` |
| ` unit ` | ` hours ` |

##### Target

<!-- splat:values path=splat_rule/parameters/variants/3/target -->

| Field | Value |
| --- | --- |
| ` body_of_water_minimum_diameter_ft ` | ` 60 ` |

##### Boat

<!-- splat:values path=splat_rule/parameters/variants/3/boat -->

| Field | Value |
| --- | --- |
| ` length_ft ` | ` 50 ` |
| ` width_ft ` | ` 20 ` |
| ` self_propelled ` | ` true ` |
| ` water_speed_mph ` | ` 6 ` |

##### Command

<!-- splat:values path=splat_rule/parameters/variants/3/command -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` requires ` | ` aboard ` |
| ` options ` | ` ["move", "turn"] ` |
| ` maximum_turn_degrees ` | ` 90 ` |

##### Duration

<!-- splat:values path=splat_rule/parameters/variants/3/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 24 ` |
| ` unit ` | ` hours ` |

##### Target

<!-- splat:values path=splat_rule/parameters/variants/4/target -->

| Field | Value |
| --- | --- |
| ` unoccupied ` | ` true ` |
| ` ground ` | ` true ` |

##### Create

<!-- splat:values path=splat_rule/parameters/variants/4/create -->

| Field | Value |
| --- | --- |
| ` type ` | ` oak_tree ` |
| ` magical ` | ` false ` |
| ` height_ft ` | ` 60 ` |
| ` trunk_diameter_ft ` | ` 5 ` |
| ` top_branch_radius_ft ` | ` 20 ` |

##### Attack

<!-- splat:yaml path=splat_rule/parameters/variants/5/attack -->

```yaml
action: bonus_action
type: melee_spell
bonus: 9
target_count: 1
target_range_from_whip_ft: 10
damage:
  amount:
    roll:
      count: 1
      sides: 6
      add: 5
  type: force
movement_max_ft: 20
```

##### Duration

<!-- splat:values path=splat_rule/parameters/variants/5/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 1 ` |
| ` unit ` | ` hours ` |
