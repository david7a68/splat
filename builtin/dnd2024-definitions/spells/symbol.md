---
splat_schema: 1
splat_id: bc5f0952-a285-5838-a7d8-4f19965ff406
splat_kind: spell
splat_key: dnd2024:spells/symbol
splat_name: Symbol
splat_origin:
  key: dnd2024:reference/spell/symbol
splat_sources:
- title: SRD 5.2.1
  locator: pp. 167
  url: https://www.dndbeyond.com/srd
---

# Symbol

## Source text

<!-- SRD page 167 -->

Level 7 Abjuration (Bard, Cleric, Druid, Wizard)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (powdered diamond worth 1,000+ GP, which the spell consumes)\
Duration: Until dispelled or triggered

You inscribe a harmful glyph either on a surface (such as a section of floor or wall) or within an object that can be closed (such as a book or chest). The glyph can cover an area no larger than 10 feet in diameter. If you choose an object, it must remain in place; if it is moved more than 10 feet from where you cast this spell, the glyph is broken, and the spell ends without being triggered.

The glyph is nearly imperceptible and requires a successful Wisdom (Perception) check against your spell save DC to notice.

When you inscribe the glyph, you set its trigger and choose which effect the symbol bears: Death, Discord, Fear, Pain, Sleep, or Stunning. Each one is explained below.

Set the Trigger. You decide what triggers the glyph when you cast the spell. For glyphs inscribed on a surface, common triggers include touching or stepping on the glyph, removing another object covering it, or approaching within a certain distance of it. For glyphs inscribed within an object, common triggers include opening that object or seeing the glyph.

You can refine the trigger so that only creatures of certain types activate it (for example, the glyph could be set to affect Aberrations). You can also set conditions for creatures that don’t trigger the glyph, such as those who say a certain password.

Once triggered, the glyph glows, filling a 60-foot-radius Sphere with Dim Light for 10 minutes, after which time the spell ends. Each creature in the Sphere when the glyph activates is targeted by its effect, as is a creature that enters the Sphere for the first time on a turn or ends its turn there. A creature is targeted only once per turn.

Death. Each target makes a Constitution saving throw, taking 10d10 Necrotic damage on a failed save or half as much damage on a successful save.

Discord. Each target makes a Wisdom saving throw. On a failed save, a target argues with other creatures for 1 minute. During this time, it is incapable of meaningful communication and has Disadvantage on attack rolls and ability checks.

Fear. Each target must succeed on a Wisdom saving throw or have the Frightened condition for 1 minute. While Frightened, the target must move at least 30 feet away from the glyph on each of its turns, if able.

Pain. Each target must succeed on a Constitution saving throw or have the Incapacitated condition for 1 minute.

Sleep. Each target must succeed on a Wisdom saving throw or have the Unconscious condition for 10 minutes. A creature awakens if it takes damage or if someone takes an action to shake it awake.

Stunning. Each target must succeed on a Wisdom saving throw or have the Stunned condition for 1 minute.

Original layout: [[../../srd-5.2.1/pages/167|page 167]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric", "druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled or triggered ` |
| ` casting_time ` | ` 1 minute ` |
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
| ` description ` | ` powdered diamond worth 1,000+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/symbol ` |
| ` implementation ` | ` dnd2024:spell_symbol ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` until ` | ` dispelled_or_source_specific_end ` |

#### Glyph

<!-- splat:yaml path=splat_rule/parameters/glyph -->

```yaml
placement_choices:
- surface
- inside_closable_object
maximum_diameter_feet: 10
object_movement_limit_feet_from_cast_point: 10
movement_limit_exceeded:
  break_glyph: true
  end_spell: true
  trigger: false
```

#### Detection

<!-- splat:yaml path=splat_rule/parameters/detection -->

```yaml
check:
  ability: wisdom
  skill: perception
  dc: spell_save_dc
success: notice_glyph
```

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` timing ` | ` cast ` |
| ` surface_examples ` | ` ["touch", "step_on", "remove_covering_object", "approach_distance"] ` |
| ` object_examples ` | ` ["open_object", "see_glyph"] ` |
| ` creature_type_filters ` | ` allowed ` |
| ` exemption_conditions ` | ` allowed ` |
| ` password_exemption ` | ` allowed ` |
| ` condition_resolution ` | ` supplied_trigger_predicate ` |

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` glow ` | ` true ` |
| ` light ` | ` dim ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 60 ` |
| ` active_seconds ` | ` 600 ` |
| ` active_expiry ` | ` end_spell ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
events:
- glyph.activated
- creature.entered_area
- turn.end
enter_occurrence_per_turn: first
in_sphere: true
limit:
  count: 1
  per: turn
  scope: target
```

#### Effect choice

<!-- splat:values path=splat_rule/parameters/effect_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` timing ` | ` inscription ` |
| ` count ` | ` 1 ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
death:
  save:
    ability: constitution
    dc: spell_save_dc
  failure_damage:
    dice: 10d10
    type: necrotic
  success_damage_scale:
    numerator: 1
    denominator: 2
discord:
  save:
    ability: wisdom
    dc: spell_save_dc
  failure:
    seconds: 60
    behavior: argue_with_other_creatures
    meaningful_communication: false
    disadvantage:
    - attack_rolls
    - ability_checks
fear:
  save:
    ability: wisdom
    dc: spell_save_dc
  failure:
    condition: frightened
    seconds: 60
    source: glyph
    movement:
      timing: each_target_turn
      minimum_feet: 30
      direction: away_from_glyph
      only_if_able: true
pain:
  save:
    ability: constitution
    dc: spell_save_dc
  failure:
    condition: incapacitated
    seconds: 60
sleep:
  save:
    ability: wisdom
    dc: spell_save_dc
  failure:
    condition: unconscious
    seconds: 600
    awaken_if:
    - takes_damage
    - another_creature_uses_action_to_shake_awake
stunning:
  save:
    ability: wisdom
    dc: spell_save_dc
  failure:
    condition: stunned
    seconds: 60
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
