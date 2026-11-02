---
splat_schema: 1
splat_id: 44a28709-8b1c-5801-814a-505dabb20225
splat_kind: spell
splat_key: dnd2024:spells/resilient_sphere
splat_name: Resilient Sphere
splat_origin:
  key: dnd2024:reference/spell/resilient_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
---

# Resilient Sphere

## Source text

<!-- SRD page 158 -->

Level 4 Abjuration (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a glass sphere)\
Duration: Concentration, up to 1 minute

A shimmering sphere encloses a Large or smaller creature or object within range. An unwilling creature must succeed on a Dexterity saving throw or be enclosed for the duration.

Nothing—not physical objects, energy, or other spell effects—can pass through the barrier, in or out, though a creature in the sphere can breathe there. The sphere is immune to all damage, and a creature or object inside can’t be damaged by attacks or effects originating from outside, nor can a creature inside the sphere damage anything outside it.

The sphere is weightless and just large enough to contain the creature or object inside. An enclosed creature can take an action to push against the sphere’s walls and thus roll the sphere at up to half the creature’s Speed. Similarly, the globe can be picked up and moved by other creatures. A Disintegrate spell targeting the globe destroys it without harming anything inside.

Original layout: [[../../srd-5.2.1/pages/158|page 158]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a glass sphere ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/resilient_sphere ` |
| ` implementation ` | ` dnd2024:spell_resilient_sphere ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` maximum_size ` | ` large ` |

#### Unwilling creature

<!-- splat:yaml path=splat_rule/parameters/unwilling_creature -->

```yaml
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    enclosed: true
  success:
    effect: none
```

#### Enclosure

<!-- splat:values path=splat_rule/parameters/enclosure -->

| Field | Value |
| --- | --- |
| ` blocks_in_and_out ` | ` ["physical_objects", "energy", "spell_effects"] ` |
| ` breathing_allowed ` | ` true ` |
| ` sphere_damage_immunity ` | ` all ` |
| ` outside_origin_damage_to_inside ` | ` false ` |
| ` inside_creature_damage_to_outside ` | ` false ` |
| ` weightless ` | ` true ` |
| ` size ` | ` just_fits_target ` |

#### Movement

<!-- splat:yaml path=splat_rule/parameters/movement -->

```yaml
enclosed_creature_action: action
roll_maximum_speed_fraction:
  numerator: 1
  denominator: 2
other_creatures_can_pick_up_and_move: true
```

#### Disintegrate

<!-- splat:values path=splat_rule/parameters/disintegrate -->

| Field | Value |
| --- | --- |
| ` target_sphere ` | ` destroy_sphere ` |
| ` harm_contents ` | ` false ` |

#### Automatic enclosure

<!-- splat:values path=splat_rule/parameters/automatic_enclosure -->

| Field | Value |
| --- | --- |
| ` eligible ` | ` ["willing_creature", "object"] ` |
