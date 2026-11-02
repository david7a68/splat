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

When a character is subjected to an effect that causes intense mental stress, Psychic damage is the best way to emulate that effect.

The Sample Mental Stress Effects table provides a few examples of such effects, with suggested saving throw DCs and damage. Mental stress can usually be resisted with a successful Wisdom save, but sometimes an Intelligence or Charisma save is more appropriate. On a successful save, a character might take half as much damage instead of no damage, at your discretion.

**Sample Mental Stress Effects**

| Example | Save DC | Psychic Damage |
| --- | --- | --- |
| A character ingests a hallucinogenic substance that distorts the character’s perception of reality. | 10 | 1d6 |
| A character touches a fiendish idol that tears at the character’s mind, threatening to shatter it. | 15 | 3d6 |
| A magical trap flings a character into the Far Realm until the end of that character’s next turn. | 20 | 9d6 |

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

<!-- splat:table
path: splat_rule/parameters/example_effects
key: Variant
order:
- hallucinogenic_substance
- fiendish_idol
- far_realm_trap
columns:
  Variant:
    path: id
    type: symbol
  Save dc:
    path: save_dc
    type: integer
  Damage dice:
    path: damage_dice
    type: dice
  Planar displacement plane:
    path: planar_displacement/plane
    type: symbol
    optional: true
  Planar displacement until:
    path: planar_displacement/until
    type: symbol
    optional: true
-->

| Variant | Save dc | Damage dice | Planar displacement plane | Planar displacement until |
| --- | --- | --- | --- | --- |
| ` Hallucinogenic substance ` | ` 10 ` | ` 1d6 ` |  |  |
| ` Fiendish idol ` | ` 15 ` | ` 3d6 ` |  |  |
| ` Far realm trap ` | ` 20 ` | ` 9d6 ` | ` Far realm ` | ` Target next turn end ` |
