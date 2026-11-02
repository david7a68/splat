---
splat_schema: 1
splat_id: 0ceb28b0-5941-5176-852a-7ff58d127cf5
splat_kind: spell
splat_key: dnd2024:spells/glyph_of_warding
splat_name: Glyph of Warding
splat_origin:
  key: dnd2024:reference/spell/glyph_of_warding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 136, 137
  url: https://www.dndbeyond.com/srd
---

# Glyph of Warding

## Source text

<!-- SRD page 136 -->

Level 3 Abjuration (Bard, Cleric, Wizard)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (powdered diamond worth 200+ GP, which the spell consumes)\
Duration: Until dispelled or triggered

You inscribe a glyph that later unleashes a magical effect. You inscribe it either on a surface (such as a table or a section of floor) or within an object that can be closed (such as a book or chest) to conceal the glyph. The glyph can cover an area no larger than 10 feet in diameter. If the surface or object is moved more than 10 feet from where you cast this spell, the glyph is broken, and the spell ends without being triggered.

The glyph is nearly imperceptible and requires a successful Wisdom (Perception) check against your spell save DC to notice.

When you inscribe the glyph, you set its trigger and choose whether it’s an explosive rune or a spell glyph, as explained below.

Set the Trigger. You decide what triggers the glyph when you cast the spell. For glyphs inscribed on a surface, common triggers include touching or stepping on the glyph, removing another object covering it, or approaching within a certain distance of it. For glyphs inscribed within an object, common triggers include opening that object or seeing the glyph. Once a glyph is triggered, this spell ends.

You can refine the trigger so that only creatures of certain types activate it (for example, the glyph could be set to affect Aberrations). You can also set conditions for creatures that don’t trigger the glyph, such as those who say a certain password.

Explosive Rune. When triggered, the glyph erupts with magical energy in a 20-foot-radius Sphere centered on the glyph. Each creature in the area makes a Dexterity saving throw. A creature takes 5d8 Acid, Cold, Fire, Lightning, or Thunder damage (your choice when you create the glyph) on a failed save or half as much damage on a successful one.

<!-- SRD page 137 -->

Spell Glyph. You can store a prepared spell of level 3 or lower in the glyph by casting it as part of creating the glyph. The spell must target a single creature or an area. The spell being stored has no immediate effect when cast in this way.

When the glyph is triggered, the stored spell takes effect. If the spell has a target, it targets the creature that triggered the glyph. If the spell affects an area, the area is centered on that creature. If the spell summons Hostile creatures or creates harmful objects or traps, they appear as close as possible to the intruder and attack it. If the spell requires Concentration, it lasts until the end of its full duration.

Using a Higher-Level Spell Slot. The damage of an explosive rune increases by 1d8 for each spell slot level above 3. If you create a spell glyph, you can store any spell of up to the same level as the spell slot you use for the Glyph of Warding.

Original layout: [[../../srd-5.2.1/pages/136|page 136]], [[../../srd-5.2.1/pages/137|page 137]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled or triggered ` |
| ` casting_time ` | ` 1 hour ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` powdered diamond worth 200+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/glyph_of_warding ` |
| ` implementation ` | ` dnd2024:spell_glyph_of_warding ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` until_dispelled_or_triggered ` |
| ` mode_choice ` | ` ["explosive_rune", "spell_glyph"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` surface_or_closable_object_interior ` |
| ` touch ` | ` true ` |
| ` diameter_max_feet ` | ` 10 ` |

#### Movement limit

<!-- splat:yaml path=splat_rule/parameters/movement_limit -->

```yaml
distance_from_cast_location_feet: 10
exceeded:
  break_glyph: true
  end_spell: true
  trigger: false
```

#### Detect

<!-- splat:yaml path=splat_rule/parameters/detect -->

```yaml
check:
  ability: wisdom
  skill: perception
  dc: spell_save_dc
success_required: true
```

#### Trigger

<!-- splat:yaml path=splat_rule/parameters/trigger -->

```yaml
chooser: caster_at_cast
specification:
  type: choice
  predicate_record_required: true
  modes:
  - surface_touch
  - surface_step
  - covering_removed
  - proximity
  - object_opened
  - glyph_seen
  - custom_event_predicate
  optional_filters:
  - creature_types
  optional_exemptions:
  - spoken_password
  - custom_creature_predicate
  custom_predicates:
    type: choice
    must_provide:
    - event
    - subject
    - predicate_parameters
    unresolved_predicate: block_release
resolution:
  release_effect: true
  end_glyph_spell: true
```

#### Explosive rune

<!-- splat:yaml path=splat_rule/parameters/explosive_rune -->

```yaml
area:
  shape: sphere
  radius_feet: 20
  center: glyph
targets: all_creatures_in_area
save:
  ability: dexterity
  failure:
    damage:
      dice: 5d8
      type_choice_at_cast:
      - acid
      - cold
      - fire
      - lightning
      - thunder
  success:
    damage_fraction:
      numerator: 1
      denominator: 2
slot_scaling:
  above: 3
  damage_per_level: 1d8
```

#### Spell glyph

<!-- splat:yaml path=splat_rule/parameters/spell_glyph -->

```yaml
stored_spell:
  prepared: true
  max_level: cast_slot_level
  allowed_targeting:
  - single_creature
  - area
  cast_during_glyph_creation: true
  immediate_effect: false
release:
  single_target: triggering_creature
  area_center: triggering_creature
  hostile_summons_or_harmful_objects_or_traps:
    location: nearest_possible_to_intruder
    attack: intruder
  concentration_spell:
    duration: full_duration
    concentration_required: false
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: until_dispelled_or_triggered
```
