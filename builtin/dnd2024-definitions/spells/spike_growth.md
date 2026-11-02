---
splat_schema: 1
splat_id: f796a67c-326b-504d-aa54-dab654fef250
splat_kind: spell
splat_key: dnd2024:spells/spike_growth
splat_name: Spike Growth
splat_origin:
  key: dnd2024:reference/spell/spike_growth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
---

# Spike Growth

## Source text

<!-- SRD page 164 -->

Level 2 Transmutation (Druid, Ranger)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (seven thorns)\
Duration: Concentration, up to 10 minutes

The ground in a 20-foot-radius Sphere centered on a point within range sprouts hard spikes and thorns. The area becomes Difficult Terrain for the duration. When a creature moves into or within the area, it takes 2d4 Piercing damage for every 5 feet it travels.

The transformation of the ground is camouflaged to look natural. Any creature that can’t see the area when the spell is cast must take a Search action and succeed on a Wisdom (Perception or Survival) check against your spell save DC to recognize the terrain as hazardous before entering it.

Original layout: [[../../srd-5.2.1/pages/164|page 164]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 150 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` seven thorns ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/spike_growth ` |
| ` implementation ` | ` dnd2024:spell_spike_growth ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 150 ` |
| ` affected_surface ` | ` ground ` |

#### Properties

<!-- splat:values path=splat_rule/parameters/properties -->

| Field | Value |
| --- | --- |
| ` difficult_terrain ` | ` true ` |
| ` camouflage ` | ` natural_terrain ` |

#### Damage

<!-- splat:values path=splat_rule/parameters/damage -->

| Field | Value |
| --- | --- |
| ` event ` | ` participant.moved ` |
| ` target_kind ` | ` creature ` |
| ` movement_scopes ` | ` ["into_area", "within_area"] ` |
| ` per_distance_feet ` | ` 5 ` |
| ` dice ` | ` 2d4 ` |
| ` type ` | ` piercing ` |
| ` saving_throw ` | ` false ` |

#### Hazard recognition

<!-- splat:yaml path=splat_rule/parameters/hazard_recognition -->

```yaml
applies_if: did_not_see_area_at_cast
before_entering: true
economy: search_action
check:
  ability: wisdom
  skill_choice:
  - perception
  - survival
  dc: spell_save_dc
success: recognize_hazard
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
