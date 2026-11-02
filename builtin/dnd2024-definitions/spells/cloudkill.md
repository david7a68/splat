---
splat_schema: 1
splat_id: d8636a54-0b4a-51bf-b387-2389a6f0741b
splat_kind: spell
splat_key: dnd2024:spells/cloudkill
splat_name: Cloudkill
splat_origin:
  key: dnd2024:reference/spell/cloudkill
splat_sources:
- title: SRD 5.2.1
  locator: pp. 116
  url: https://www.dndbeyond.com/srd
---

# Cloudkill

## Source text

<!-- SRD page 116 -->

Level 5 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You create a 20-foot-radius Sphere of yellow-green fog centered on a point within range. The fog lasts for the duration or until strong wind (such as the one created by Gust of Wind) disperses it, ending the spell. Its area is Heavily Obscured.

Each creature in the Sphere makes a Constitution saving throw, taking 5d8 Poison damage on a failed save or half as much damage on a successful one. A creature must also make this save when the Sphere moves into its space and when it enters the Sphere or ends its turn there. A creature makes this save only once per turn.

The Sphere moves 10 feet away from you at the start of each of your turns.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 5.

Original layout: [[../../srd-5.2.1/pages/116|page 116]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/cloudkill ` |
| ` implementation ` | ` dnd2024:spell_cloudkill ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` save_triggers ` | ` ["initial_in_area", "sphere_moves_into_creature_space", "creature_enters_area", "creature_turn_ends_in_area"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_point_within_range ` |
| ` obscurement ` | ` heavy ` |
| ` duration ` | ` spell_duration ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count:
        op: add
        args:
        - 5
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 5
      sides: 8
      add: 0
    type: poison
success:
  damage:
    dice:
      count:
        op: add
        args:
        - 5
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 5
      sides: 8
      add: 0
    type: poison
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Save limit

<!-- splat:values path=splat_rule/parameters/save_limit -->

| Field | Value |
| --- | --- |
| ` per_creature_per_turn ` | ` 1 ` |

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` event ` | ` caster_turn_start ` |
| ` feet ` | ` 10 ` |
| ` direction ` | ` away_from_caster ` |

#### End when

<!-- splat:values path=splat_rule/parameters/end_when -->

| Field | Value |
| --- | --- |
| ` strong_wind_disperses_fog ` | ` true ` |
