---
splat_schema: 1
splat_id: 8ed4e98a-af2e-52fe-9716-7adf6f73930a
splat_kind: spell
splat_key: dnd2024:spells/darkness
splat_name: Darkness
splat_origin:
  key: dnd2024:reference/spell/darkness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
---

# Darkness

## Source text

<!-- SRD page 122 -->

Level 2 Evocation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, M (bat fur and a piece of coal)\
Duration: Concentration, up to 10 minutes\
For the duration, magical Darkness spreads from a\
point within range and fills a 15-foot-radius Sphere.\
Darkvision can’t see through it, and nonmagical\
light can’t illuminate it.\
Alternatively, you cast the spell on an object that\
isn’t being worn or carried, causing the Darkness\
to fill a 15-foot Emanation originating from that\
object. Covering that object with something opaque,\
such as a bowl or helm, blocks the Darkness.\
If any of this spell’s area overlaps with an area of\
Bright Light or Dim Light created by a spell of level\
2 or lower, that other spell is dispelled.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` bat fur and a piece of coal ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/darkness ` |
| ` implementation ` | ` dnd2024:spell_darkness ` |

### Parameters

#### Area modes

<!-- splat:yaml path=splat_rule/parameters/area_modes -->

```yaml
point:
  shape: sphere
  radius_feet: 15
  within_spell_range: true
object:
  shape: emanation
  radius_feet: 15
  within_spell_range: true
  object_worn: false
  object_carried: false
  blocked_while_object_covered_by_opaque_cover: true
```

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` illumination ` | ` magical_darkness ` |
| ` duration ` | ` spell_duration ` |
| ` darkvision_sees_through ` | ` false ` |
| ` nonmagical_light_illuminates ` | ` false ` |

#### Overlap

<!-- splat:values path=splat_rule/parameters/overlap -->

| Field | Value |
| --- | --- |
| ` light_levels ` | ` ["bright", "dim"] ` |
| ` created_by_spell_maximum_level ` | ` 2 ` |
| ` effect ` | ` dispel_other_spell ` |
