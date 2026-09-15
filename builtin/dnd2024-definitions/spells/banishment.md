---
splat_schema: 1
splat_id: fee498a7-0a7e-59f1-aeae-64dacd463972
splat_kind: spell
splat_key: dnd2024:spells/banishment
splat_name: Banishment
splat_origin:
  key: dnd2024:reference/spell/banishment
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
---

# Banishment

## Source text

<!-- SRD page 112 -->

Level 4 Abjuration (Cleric, Paladin, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a pentacle)\
Duration: Concentration, up to 1 minute\
One creature that you can see within range must\
succeed on a Charisma saving throw or be trans-\
ported to a harmless demiplane for the duration.\
While there, the target has the Incapacitated con-\
dition. When the spell ends, the target reappears in\
the space it left or in the nearest unoccupied space\
if that space is occupied.\
If the target is an Aberration, a Celestial, an Ele-\
mental, a Fey, or a Fiend, the target doesn’t return\
if the spell lasts for 1 minute. The target is instead\
transported to a random location on a plane (GM’s\
choice) associated with its creature type.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "paladin", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pentacle ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/banishment ` |
| ` implementation ` | ` dnd2024:spell_banishment ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
visible: true
range_feet: 30
count:
  base: 1
  increment_per_slot_above: 1
  base_slot: 4
```

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` transport ` | ` harmless_demiplane ` |
| ` condition_while_there ` | ` incapacitated ` |

#### Spell end

<!-- splat:yaml path=splat_rule/parameters/spell_end -->

```yaml
default:
  return: original_space
  occupied_fallback: nearest_unoccupied_space
full_minute_exception:
  types:
  - aberration
  - celestial
  - elemental
  - fey
  - fiend
  minimum_elapsed_seconds: 60
  return_to_original_plane: false
  destination_plane:
    chooser: gm
    constraint: associated_with_target_creature_type
  destination_location: random
```
