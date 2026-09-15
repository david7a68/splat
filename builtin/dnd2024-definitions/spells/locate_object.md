---
splat_schema: 1
splat_id: 162482da-06c9-57c0-bb21-ed2effd1e491
splat_kind: spell
splat_key: dnd2024:spells/locate_object
splat_name: Locate Object
splat_origin:
  key: dnd2024:reference/spell/locate_object
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
---

# Locate Object

## Source text

<!-- SRD page 145 -->

Level 2 Divination (Bard, Cleric, Druid, Paladin,\
Ranger, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a forked twig)\
Duration: Concentration, up to 10 minutes\
Describe or name an object that is familiar to you.\
You sense the direction to the object’s location if\
that object is within 1,000 feet of you. If the object is\
in motion, you know the direction of its movement.\
The spell can locate a specific object known to you\
if you have seen it up close—within 30 feet—at least\
once. Alternatively, the spell can locate the nearest\
object of a particular kind, such as a certain kind of\
apparel, jewelry, furniture, tool, or weapon.\
This spell can’t locate an object if any thickness\
of lead blocks a direct path between you and the\
object.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "druid", "paladin", "ranger", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
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
| ` description ` | ` a forked twig ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/locate_object ` |
| ` implementation ` | ` dnd2024:spell_locate_object ` |

### Parameters

#### Selection

<!-- splat:yaml path=splat_rule/parameters/selection -->

```yaml
specific_object:
  familiar: true
  describe_or_name: true
  seen_at_least_once_within_feet: 30
object_kind:
  nearest: true
```

#### Detection

<!-- splat:values path=splat_rule/parameters/detection -->

| Field | Value |
| --- | --- |
| ` maximum_distance_feet ` | ` 1000 ` |
| ` outputs ` | ` ["direction_to_object", "movement_direction_if_moving"] ` |

#### Blocked

<!-- splat:values path=splat_rule/parameters/blocked -->

| Field | Value |
| --- | --- |
| ` material ` | ` lead ` |
| ` thickness ` | ` any ` |
| ` path ` | ` direct_caster_to_object ` |
