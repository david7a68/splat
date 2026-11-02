---
splat_schema: 1
splat_id: a42e0a5b-0341-5b22-b5e1-000d11d47c62
splat_kind: spell
splat_key: dnd2024:spells/daylight
splat_name: Daylight
splat_origin:
  key: dnd2024:reference/spell/daylight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
---

# Daylight

## Source text

<!-- SRD page 122 -->

Level 3 Evocation (Cleric, Druid, Paladin, Ranger, Sorcerer)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: 1 hour

For the duration, sunlight spreads from a point within range and fills a 60-foot-radius Sphere. The sunlight’s area is Bright Light and sheds Dim Light for an additional 60 feet.

Alternatively, you cast the spell on an object that isn’t being worn or carried, causing the sunlight to fill a 60-foot Emanation originating from that object. Covering that object with something opaque, such as a bowl or helm, blocks the sunlight.

If any of this spell’s area overlaps with an area of Darkness created by a spell of level 3 or lower, that other spell is dispelled.

Original layout: [[../../srd-5.2.1/pages/122|page 122]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric", "druid", "paladin", "ranger", "sorcerer"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/daylight ` |
| ` implementation ` | ` dnd2024:spell_daylight ` |

### Parameters

#### Origin choice

<!-- splat:yaml path=splat_rule/parameters/origin_choice -->

```yaml
point:
  shape: sphere
object:
  shape: emanation
  not_worn: true
  not_carried: true
  opaque_cover_blocks: true
```

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` sunlight ` | ` true ` |
| ` bright_radius_feet ` | ` 60 ` |
| ` additional_dim_feet ` | ` 60 ` |

#### Dispel

<!-- splat:values path=splat_rule/parameters/dispel -->

| Field | Value |
| --- | --- |
| ` event ` | ` area.overlap ` |
| ` target ` | ` darkness_spell ` |
| ` maximum_spell_level ` | ` 3 ` |
