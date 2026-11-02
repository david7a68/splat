---
splat_schema: 1
splat_id: 94e81453-9723-5b08-a541-f6f5bfd8074b
splat_kind: spell
splat_key: dnd2024:spells/locate_creature
splat_name: Locate Creature
splat_origin:
  key: dnd2024:reference/spell/locate_creature
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
---

# Locate Creature

## Source text

<!-- SRD page 145 -->

Level 4 Divination (Bard, Cleric, Druid, Paladin, Ranger, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (fur from a bloodhound)\
Duration: Concentration, up to 1 hour

Describe or name a creature that is familiar to you. You sense the direction to the creature’s location if that creature is within 1,000 feet of you. If the creature is moving, you know the direction of its movement.

The spell can locate a specific creature known to you or the nearest creature of a specific kind (such as a human or a unicorn) if you have seen such a creature up close—within 30 feet—at least once. If the creature you described or named is in a different form, such as under the effects of a Flesh to Stone or Polymorph spell, this spell doesn’t locate the creature.

This spell can’t locate a creature if any thickness of lead blocks a direct path between you and the creature.

Original layout: [[../../srd-5.2.1/pages/145|page 145]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "druid", "paladin", "ranger", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` fur from a bloodhound ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/locate_creature ` |
| ` implementation ` | ` dnd2024:spell_locate_creature ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` failure_conditions ` | ` ["target_in_different_form", "lead_blocks_direct_path"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Target choice

<!-- splat:yaml path=splat_rule/parameters/target_choice -->

```yaml
chooser: caster
named_or_described: true
familiar: true
options:
- known_specific_creature
- nearest_of_specific_kind
qualifying_prior_sighting:
  distance_feet_max: 30
  count_min: 1
```

#### Detection

<!-- splat:values path=splat_rule/parameters/detection -->

| Field | Value |
| --- | --- |
| ` distance_feet_max ` | ` 1000 ` |
| ` outputs ` | ` ["direction_to_location"] ` |
| ` moving_target_additional_output ` | ` movement_direction ` |

#### Lead block

<!-- splat:values path=splat_rule/parameters/lead_block -->

| Field | Value |
| --- | --- |
| ` thickness ` | ` any ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
