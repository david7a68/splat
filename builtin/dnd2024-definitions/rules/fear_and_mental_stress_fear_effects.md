---
splat_schema: 2
splat_id: 3f86bacd-c824-5fbd-8b2e-da184b352930
splat_kind: rule
splat_key: dnd2024:rules/fear_and_mental_stress_fear_effects
splat_name: Fear Effects
splat_origin: dnd2024:reference/pages/196
splat_sources:
- SRD 5.2.1, pp. 196, https://www.dndbeyond.com/srd
---

# Fear Effects

## Source text

<!-- SRD page 196 -->

Whenever the characters encounter something that is supernaturally frightful, use the Frightened condition as the baseline effect. Fear effects typically require a Wisdom saving throw, with a save DC based on how terrifying the situation is. The Sample Fear DCs table provides some examples.

**Sample Fear DCs**

| Example | Save DC |
| --- | --- |
| When the characters open a sarcophagus, a harmless yet terrifying apparition appears. | 10 |
| A character triggers a magical trap that creates an illusory manifestation of that character’s worst fears, visible only to that character. | 15 |
| A portal to the Abyss opens, revealing a nightmarish realm of torment and slaughter. | 20 |

Typically, a Frightened creature repeats the saving throw at the end of each of its turns, ending the effect on itself on a success.

At your discretion, a Frightened creature might be subject to other effects as long as the Frightened condition lasts. Consider these examples:

•  The Frightened creature must take the Dash action on each of its turns and uses its movement to get farther away from the source of its fear.

•  Attack rolls against the Frightened creature have Advantage.

•    The Frightened creature can do only one of the following on each of its turns: move, take an action, or take a Bonus Action.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/fear_and_mental_stress_fear_effects ` |
| ` implementation ` | ` dnd2024:rule_fear_and_mental_stress_fear_effects ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` supernaturally_frightful_encounter ` |
| ` default_save_ability ` | ` wisdom ` |
| ` failed_save_condition ` | ` frightened ` |

#### Dc choice

<!-- splat:values path=splat_rule/parameters/dc_choice -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` criterion ` | ` situation_terror ` |

##### Examples

<!-- splat:table
path: splat_rule/parameters/dc_choice/examples
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Situation:
    path: situation
    type: symbol
  Dc:
    path: dc
    type: integer
-->

| Key | Situation | Dc |
| --- | --- | --- |
| ` row_1 ` | ` Harmless sarcophagus apparition ` | ` 10 ` |
| ` row_2 ` | ` Magical trap private worst fear illusion ` | ` 15 ` |
| ` row_3 ` | ` Abyss portal reveals torment ` | ` 20 ` |

#### Default repeat save

<!-- splat:values path=splat_rule/parameters/default_repeat_save -->

| Field | Value |
| --- | --- |
| ` timing ` | ` end_of_each_target_turn ` |
| ` success ` | ` effect_ends ` |

#### Optional riders

<!-- splat:values path=splat_rule/parameters/optional_riders -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` duration ` | ` while_frightened ` |

##### Choices

<!-- splat:table
path: splat_rule/parameters/optional_riders/choices
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Must take action:
    path: must_take_action
    type: symbol
    optional: true
  Movement direction:
    path: movement_direction
    type: symbol
    optional: true
  Attacks against target:
    path: attacks_against_target
    type: symbol
    optional: true
  Per turn choose exactly one:
    path: per_turn_choose_exactly_one
    type: yaml
    optional: true
-->

| Key | Must take action | Movement direction | Attacks against target | Per turn choose exactly one |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` Dash ` | ` Away from fear source ` |  |  |
| ` row_2 ` |  |  | ` Advantage ` |  |
| ` row_3 ` |  |  |  | ` ["movement", "action", "bonus_action"] ` |
