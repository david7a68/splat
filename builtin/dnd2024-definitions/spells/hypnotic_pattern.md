---
splat_schema: 1
splat_id: fc4b0931-2173-59ab-a1df-e4807fc8aad3
splat_kind: spell
splat_key: dnd2024:spells/hypnotic_pattern
splat_name: Hypnotic Pattern
splat_origin:
  key: dnd2024:reference/spell/hypnotic_pattern
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
---

# Hypnotic Pattern

## Source text

<!-- SRD page 141 -->

Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: S, M (a pinch of confetti)\
Duration: Concentration, up to 1 minute\
You create a twisting pattern of colors in a 30-foot\
Cube within range. The pattern appears for a mo-\
ment and vanishes. Each creature in the area who\
can see the pattern must succeed on a Wisdom\
saving throw or have the Charmed condition for the\
duration. While Charmed, the creature has the Inca-\
pacitated condition and a Speed of 0.\
The spell ends for an affected creature if it takes\
any damage or if someone else uses an action to\
shake the creature out of its stupor.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pinch of confetti ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hypnotic_pattern ` |
| ` implementation ` | ` dnd2024:spell_hypnotic_pattern ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 30 ` |
| ` range_feet ` | ` 120 ` |

#### Pattern

<!-- splat:values path=splat_rule/parameters/pattern -->

| Field | Value |
| --- | --- |
| ` appears ` | ` momentarily ` |
| ` then ` | ` vanishes ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` in_area ` | ` true ` |
| ` sees_pattern ` | ` true ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition: charmed
while_charmed:
  condition: incapacitated
  speed_feet: 0
```

#### End target triggers

<!-- splat:table
path: splat_rule/parameters/end_target_triggers
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Subject:
    path: subject
    type: symbol
    optional: true
  Actor:
    path: actor
    type: symbol
    optional: true
  Action:
    path: action
    type: symbol
    optional: true
  Economy:
    path: economy
    type: symbol
    optional: true
-->

| Key | Event | Subject | Actor | Action | Economy |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` damage.applied ` | ` Target ` |  |  |  |
| ` row_2 ` | ` action.completed ` |  | ` Another creature ` | ` Shake target awake ` | ` Action ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
