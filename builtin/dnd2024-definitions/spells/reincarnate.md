---
splat_schema: 1
splat_id: 31f53b8a-2be8-58e4-a533-fd0b381f61f3
splat_kind: spell
splat_key: dnd2024:spells/reincarnate
splat_name: Reincarnate
splat_origin:
  key: dnd2024:reference/spell/reincarnate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
---

# Reincarnate

## Source text

<!-- SRD page 158 -->

Level 5 Necromancy (Druid)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (rare oils worth 1,000+ GP, which the spell consumes)\
Duration: Instantaneous

You touch a dead Humanoid or a piece of one. If the creature has been dead no longer than 10 days, the spell forms a new body for it and calls the soul to enter that body. Roll 1d10 and consult the table below to determine the body’s species, or the GM chooses another playable species.

| 1d10 | Species | 1d10 | Species |
| --- | --- | --- | --- |
| 1 | Roll again. | 6 | Goliath |
| 2 | Dragonborn | 7 | Halfling |
| 3 | Dwarf | 8 | Human |
| 4 | Elf | 9 | Orc |
| 5 | Gnome | 10 | Tiefling |

The reincarnated creature makes any choices that a species’ description offers, and the creature recalls its former life. It retains the capabilities it had in its original form, except it loses the traits of its previous species and gains the traits of its new one.

Original layout: [[../../srd-5.2.1/pages/158|page 158]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 1 hour ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` rare oils worth 1,000+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/reincarnate ` |
| ` implementation ` | ` dnd2024:spell_reincarnate ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` dead_humanoid_or_piece ` |
| ` touched ` | ` true ` |
| ` dead_days_maximum ` | ` 10 ` |

#### Revive

<!-- splat:yaml path=splat_rule/parameters/revive -->

```yaml
create_new_body: true
call_soul_into_body: true
species_selection:
  methods:
  - roll
  - gm_choose_another_playable_species
  roll:
    dice:
      count: 1
      sides: 10
      add: 0
  faces:
    '1': reroll
    '2': dragonborn
    '3': dwarf
    '4': elf
    '5': gnome
    '6': goliath
    '7': halfling
    '8': human
    '9': orc
    '10': tiefling
species_options_chooser: reincarnated_creature
retains:
- former_life_memories
- original_capabilities_except_species_traits
remove: previous_species_traits
gain: new_species_traits
```
