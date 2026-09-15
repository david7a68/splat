---
splat_schema: 1
splat_id: 6f2e880d-4f14-5820-a14c-228a2bc55e20
splat_kind: rule
splat_key: dnd2024:rules/fear_and_mental_stress_mental_stress_effects
splat_name: Mental Stress Effects
splat_origin:
  key: dnd2024:reference/pages/196
splat_sources:
- title: SRD 5.2.1
  locator: pp. 196
  url: https://www.dndbeyond.com/srd
---

# Mental Stress Effects

## Source text

<!-- SRD page 196 -->

When a character is subjected to an effect that\
causes intense mental stress, Psychic damage is the\
best way to emulate that effect.\
The Sample Mental Stress Effects table provides a\
few examples of such effects, with suggested saving\
throw DCs and damage. Mental stress can usually\
be resisted with a successful Wisdom save, but\
sometimes an Intelligence or Charisma save is more\
appropriate. On a successful save, a character might\
take half as much damage instead of no damage, at\
your discretion.\
Sample Mental Stress Effects\
Save Psychic\
Example DC Damage\
A character ingests a hallucinogenic 10 1d6\
substance that distorts the charac-\
ter’s perception of reality.\
A character touches a fiendish idol 15 3d6\
that tears at the character’s mind,\
threatening to shatter it.\
A magical trap flings a character into 20 9d6\
the Far Realm until the end of that\
character’s next turn.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/fear_and_mental_stress_mental_stress_effects ` |
| ` implementation ` | ` dnd2024:rules_fear_and_mental_stress_mental_stress_effects ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_type ` | ` psychic ` |
| ` examples_are_suggestions ` | ` true ` |

#### Save ability choice

<!-- splat:values path=splat_rule/parameters/save_ability_choice -->

| Field | Value |
| --- | --- |
| ` default ` | ` wisdom ` |
| ` alternatives ` | ` ["intelligence", "charisma"] ` |
| ` authority ` | ` gm ` |

#### Success damage choice

<!-- splat:values path=splat_rule/parameters/success_damage_choice -->

| Field | Value |
| --- | --- |
| ` options ` | ` ["none", "half"] ` |
| ` authority ` | ` gm ` |

#### Example effects

<!-- splat:rows path=splat_rule/parameters/example_effects -->

| Row | id | save_dc |
| --- | --- | --- |
| ` 0 ` | ` hallucinogenic_substance ` | ` 10 ` |
| ` 1 ` | ` fiendish_idol ` | ` 15 ` |
| ` 2 ` | ` far_realm_trap ` | ` 20 ` |

##### Damage dice

<!-- splat:values path=splat_rule/parameters/example_effects/0/damage_dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

##### Damage dice

<!-- splat:values path=splat_rule/parameters/example_effects/1/damage_dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 3 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

##### Damage dice

<!-- splat:values path=splat_rule/parameters/example_effects/2/damage_dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 9 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

##### Planar displacement

<!-- splat:values path=splat_rule/parameters/example_effects/2/planar_displacement -->

| Field | Value |
| --- | --- |
| ` plane ` | ` far_realm ` |
| ` until ` | ` target_next_turn_end ` |
