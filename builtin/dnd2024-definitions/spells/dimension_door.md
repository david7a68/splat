---
splat_schema: 1
splat_id: 92805162-dfe9-551f-bc67-dba7847f5e2b
splat_kind: spell
splat_key: dnd2024:spells/dimension_door
splat_name: Dimension Door
splat_origin:
  key: dnd2024:reference/spell/dimension_door
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123, 124
  url: https://www.dndbeyond.com/srd
---

# Dimension Door

## Source text

<!-- SRD page 123 -->

Level 4 Conjuration (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 500 feet\
Components: V\
Duration: Instantaneous\
You teleport to a location within range. You arrive\
at exactly the spot desired. It can be a place you can\
see, one you can visualize, or one you can describe\
by stating distance and direction, such as “200 feet\
straight downward” or “300 feet upward to the\
northwest at a 45-degree angle.”\
You can also teleport one willing creature. The\
creature must be within 5 feet of you when you tele-\
port, and it teleports to a space within 5 feet of your\
destination space.\

<!-- SRD page 124 -->

If you, the other creature, or both would arrive in\
a space occupied by a creature or completely filled\
by one or more objects, you and any creature trav-\
eling with you each take 4d6 Force damage, and the\
teleportation fails.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]], [[../../srd-5.2.1/pages/124|page 124]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 500 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dimension_door ` |
| ` implementation ` | ` dnd2024:spell_dimension_door ` |

### Parameters

#### Teleport

<!-- splat:values path=splat_rule/parameters/teleport -->

| Field | Value |
| --- | --- |
| ` subject ` | ` caster ` |
| ` destination_choice ` | ` ["seen", "visualized", "distance_and_direction"] ` |
| ` range_feet ` | ` 500 ` |
| ` exact_destination ` | ` true ` |

#### Companion

<!-- splat:values path=splat_rule/parameters/companion -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` initial_distance_feet ` | ` 5 ` |
| ` destination_distance_from_caster_feet ` | ` 5 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
if_any_destination:
- creature_occupied
- completely_object_filled
teleport_fails_for_all: true
damage_to_all_travelers:
  amount:
    dice:
      count: 4
      sides: 6
      add: 0
  type: force
```
