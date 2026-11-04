---
splat_schema: 2
splat_id: 70a6f59d-157f-51c2-9360-a2c43c96cd11
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_environmental_curses
splat_name: Environmental Curses
splat_origin: dnd2024:reference/pages/193
splat_sources:
- SRD 5.2.1, pp. 193, https://www.dndbeyond.com/srd
---

# Environmental Curses

## Source text

<!-- SRD page 193 -->

Some locations are so suffused with evil that anyone who lingers there is burdened with a curse. Demonic Possession is one example of an environmental curse.

Demonic Possession. Demonic Possession arises from the chaos and evil of the Abyss and commonly besets creatures that interact with demonic objects or linger in desecrated locations, where demonic spirits await victims.

A creature that becomes the target of Demonic Possession must succeed on a DC 15 Charisma saving throw or be possessed by a bodiless demonic entity. Whenever the possessed creature rolls a 1 on a D20 Test, the demonic entity takes control of the creature and determines the creature’s behavior thereafter. At the end of each of the possessed creature’s later turns, the creature makes a DC 15 Charisma saving throw, regaining control of itself on a success.

After finishing a Long Rest, a creature with Demonic Possession makes a DC 15 Charisma saving throw. On a successful save, the effect ends on the creature. A Dispel Evil and Good spell or any magic that removes a curse also ends the effect on it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_environmental_curses ` |
| ` implementation ` | ` dnd2024:rule_magical_contagions_environmental_curses ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` curse ` | ` demonic_possession ` |
| ` exposure_contexts ` | ` ["demonic_object_interaction", "desecrated_location_lingering"] ` |
| ` target_selection ` | ` gm ` |

#### Initial save

<!-- splat:values path=splat_rule/parameters/initial_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` dc ` | ` 15 ` |
| ` failure ` | ` possessed_by_bodiless_demonic_entity ` |

#### While possessed

<!-- splat:yaml path=splat_rule/parameters/while_possessed -->

```yaml
trigger:
  d20_test_die_result: 1
effect: entity_controls_creature_behavior
regain_control_save:
  timing: end_of_each_later_turn
  ability: charisma
  dc: 15
  success: creature_regains_control
  curse_persists: true
```

#### End curse

<!-- splat:yaml path=splat_rule/parameters/end_curse -->

```yaml
long_rest_end_save:
  ability: charisma
  dc: 15
  success: curse_ends
magic:
- key: dnd2024:spells/dispel_evil_and_good
- any_curse_removal_magic
```
