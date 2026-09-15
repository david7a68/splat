---
splat_schema: 1
splat_id: 9484799d-b608-508a-b072-41698346c374
splat_kind: spell
splat_key: dnd2024:spells/sunburst
splat_name: Sunburst
splat_origin:
  key: dnd2024:reference/spell/sunburst
splat_sources:
- title: SRD 5.2.1
  locator: pp. 167
  url: https://www.dndbeyond.com/srd
---

# Sunburst

## Source text

<!-- SRD page 167 -->

Level 8 Evocation (Cleric, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a piece of sunstone)\
Duration: Instantaneous\
Brilliant sunlight flashes in a 60-foot-radius Sphere\
centered on a point you choose within range. Each\
creature in the Sphere makes a Constitution saving\
throw. On a failed save, a creature takes 12d6 Ra-\
diant damage and has the Blinded condition for 1\
minute. On a successful save, it takes half as much\
damage only.\
A creature Blinded by this spell makes another\
Constitution saving throw at the end of each of its\
turns, ending the effect on itself on a success.\
This spell dispels Darkness in its area that was\
created by any spell.\

Original layout: [[../../srd-5.2.1/pages/167|page 167]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 8 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric", "druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
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
| ` description ` | ` a piece of sunstone ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sunburst ` |
| ` implementation ` | ` dnd2024:spell_sunburst ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 60 ` |
| ` origin ` | ` chosen_point_within_range ` |
| ` illumination ` | ` sunlight ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` each_in_area ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count: 12
      sides: 6
      add: 0
    type: radiant
  condition: blinded
  duration_minutes: 1
success:
  damage:
    dice:
      count: 12
      sides: 6
      add: 0
    type: radiant
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` requires ` | ` blinded_by_spell ` |
| ` event ` | ` target_turn_end ` |
| ` ability ` | ` constitution ` |
| ` success ` | ` end_effect_on_target ` |

#### Dispel

<!-- splat:values path=splat_rule/parameters/dispel -->

| Field | Value |
| --- | --- |
| ` in_area ` | ` true ` |
| ` effect ` | ` darkness ` |
| ` created_by ` | ` any_spell ` |
