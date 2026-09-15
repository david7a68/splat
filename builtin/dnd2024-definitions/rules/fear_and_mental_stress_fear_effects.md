---
splat_schema: 1
splat_id: 3f86bacd-c824-5fbd-8b2e-da184b352930
splat_kind: rule
splat_key: dnd2024:rules/fear_and_mental_stress_fear_effects
splat_name: Fear Effects
splat_origin:
  key: dnd2024:reference/pages/196
splat_sources:
- title: SRD 5.2.1
  locator: pp. 196
  url: https://www.dndbeyond.com/srd
---

# Fear Effects

## Source text

<!-- SRD page 196 -->

Whenever the characters encounter something that\
is supernaturally frightful, use the Frightened con-\
dition as the baseline effect. Fear effects typically\
require a Wisdom saving throw, with a save DC\
based on how terrifying the situation is. The Sample\
Fear DCs table provides some examples.\
Sample Fear DCs\
Example Save DC\
When the characters open a sarcophagus, a 10\
harmless yet terrifying apparition appears.\
A character triggers a magical trap that 15\
creates an illusory manifestation of that\
character’s worst fears, visible only to that\
character.\
A portal to the Abyss opens, revealing a 20\
nightmarish realm of torment and slaughter.\
Typically, a Frightened creature repeats the saving\
throw at the end of each of its turns, ending the ef-\
fect on itself on a success.\
At your discretion, a Frightened creature might\
be subject to other effects as long as the Frightened\
condition lasts. Consider these examples:\
•  The Frightened creature must take the Dash ac-\
tion on each of its turns and uses its movement to\
get farther away from the source of its fear.\
•  Attack rolls against the Frightened creature have\
Advantage.\
•    The Frightened creature can do only one of the\
following on each of its turns: move, take an ac-\
tion, or take a Bonus Action.\

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

<!-- splat:yaml path=splat_rule/parameters/dc_choice -->

```yaml
actor: gm
criterion: situation_terror
examples:
- situation: harmless_sarcophagus_apparition
  dc: 10
- situation: magical_trap_private_worst_fear_illusion
  dc: 15
- situation: abyss_portal_reveals_torment
  dc: 20
```

#### Default repeat save

<!-- splat:values path=splat_rule/parameters/default_repeat_save -->

| Field | Value |
| --- | --- |
| ` timing ` | ` end_of_each_target_turn ` |
| ` success ` | ` effect_ends ` |

#### Optional riders

<!-- splat:yaml path=splat_rule/parameters/optional_riders -->

```yaml
actor: gm
duration: while_frightened
choices:
- must_take_action: dash
  movement_direction: away_from_fear_source
- attacks_against_target: advantage
- per_turn_choose_exactly_one:
  - movement
  - action
  - bonus_action
```
