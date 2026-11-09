---
splat_schema: 2
splat_id: a42e0a5b-0341-5b22-b5e1-000d11d47c62
splat_kind: spell
splat_key: dnd2024:spells/daylight
splat_name: Daylight
splat_origin: dnd2024:reference/spell/daylight
splat_sources:
- SRD 5.2.1, pp. 122, https://www.dndbeyond.com/srd
---

# Daylight


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Evocation |
| Classes | Cleric, Druid, Paladin, Ranger, Sorcerer |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | 1 hour |
| Ritual | No |
| Concentration | No |


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
