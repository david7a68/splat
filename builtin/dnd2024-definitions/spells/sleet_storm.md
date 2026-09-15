---
splat_schema: 1
splat_id: 6ebcd8ec-1036-5ca6-82ff-1bcd29d51812
splat_kind: spell
splat_key: dnd2024:spells/sleet_storm
splat_name: Sleet Storm
splat_origin:
  key: dnd2024:reference/spell/sleet_storm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
---

# Sleet Storm

## Source text

<!-- SRD page 163 -->

Level 3 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a miniature umbrella)\
Duration: Concentration, up to 1 minute\
Until the spell ends, sleet falls in a 40-foot-tall,\
20-foot-radius Cylinder centered on a point you\
choose within range. The area is Heavily Obscured,\
and exposed flames in the area are doused.\
Ground in the Cylinder is Difficult Terrain. When\
a creature enters the Cylinder for the first time on\
a turn or starts its turn there, it must succeed on a\
Dexterity saving throw or have the Prone condition\
and lose Concentration.\

Original layout: [[../../srd-5.2.1/pages/163|page 163]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
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
| ` description ` | ` a miniature umbrella ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sleet_storm ` |
| ` implementation ` | ` dnd2024:spell_sleet_storm ` |

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
| ` shape ` | ` cylinder ` |
| ` height_feet ` | ` 40 ` |
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 150 ` |

#### Properties

<!-- splat:values path=splat_rule/parameters/properties -->

| Field | Value |
| --- | --- |
| ` obscurement ` | ` heavy ` |
| ` exposed_flames ` | ` doused ` |
| ` ground ` | ` difficult_terrain ` |

#### Triggers

<!-- splat:rows path=splat_rule/parameters/triggers -->

| Row | event | occurrence_per_turn | subjects |
| --- | --- | --- | --- |
| ` 0 ` | ` creature.entered_area ` | ` first ` |  |
| ` 1 ` | ` turn.start ` |  | ` creatures_in_area ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc: spell_save_dc
success: unaffected
failure:
  condition: prone
  concentration: lost
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
