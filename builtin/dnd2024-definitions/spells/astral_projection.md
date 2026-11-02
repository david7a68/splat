---
splat_schema: 1
splat_id: 8dadd17c-9a0c-5858-b18f-560406795e16
splat_kind: spell
splat_key: dnd2024:spells/astral_projection
splat_name: Astral Projection
splat_origin:
  key: dnd2024:reference/spell/astral_projection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111
  url: https://www.dndbeyond.com/srd
---

# Astral Projection

## Source text

<!-- SRD page 111 -->

Level 9 Necromancy (Cleric, Warlock, Wizard)\
Casting Time: 1 hour\
Range: 10 feet\
Components: V, S, M (for each of the spell’s targets, one jacinth worth 1,000+ GP and one silver bar worth 100+ GP, all of which the spell consumes)\
Duration: Until dispelled

You and up to eight willing creatures within range project your astral bodies into the Astral Plane (the spell ends instantly if you are already on that plane). Each target’s body is left behind in a state of suspended animation; it has the Unconscious condition, doesn’t need food or air, and doesn’t age.

A target’s astral form resembles its body in almost every way, replicating its game statistics and possessions. The principal difference is the addition of a silvery cord that trails from between the shoulder blades of the astral form. The cord fades from view after 1 foot. If the cord is cut—which happens only when an effect states that it does so—the target’s body and astral form both die.

A target’s astral form can travel through the Astral Plane. The moment an astral form leaves that plane, the target’s body and possessions travel along the silver cord, causing the target to re-enter its body on the new plane.

Any damage or other effects that apply to an astral form have no effect on the target’s body and vice versa. If a target’s body or astral form drops to 0 Hit Points, the spell ends for that target. The spell ends for all the targets if you take a Magic action to dismiss it.

When the spell ends for a target who isn’t dead, the target reappears in its body and exits the state of suspended animation.

Original layout: [[../../srd-5.2.1/pages/111|page 111]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["cleric", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled ` |
| ` casting_time ` | ` 1 hour ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 10 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` for each of the spell’s targets, one jacinth worth 1,000+ GP and one silver bar worth 100+ GP, all of which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/astral_projection ` |
| ` implementation ` | ` dnd2024:spell_astral_projection ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 8 ` |
| ` within_spell_range ` | ` true ` |
| ` willing ` | ` true ` |
| ` additional_target ` | ` caster ` |

#### Already astral plane

<!-- splat:values path=splat_rule/parameters/already_astral_plane -->

| Field | Value |
| --- | --- |
| ` end_immediately ` | ` true ` |

#### Projection

<!-- splat:yaml path=splat_rule/parameters/projection -->

```yaml
plane: astral
copy:
- game_statistics
- possessions
- body_appearance
physical_body:
  condition: unconscious
  needs_food: false
  needs_air: false
  ages: false
silver_cord:
  visible_length_feet: 1
  origin: between_shoulders
  cut_only_by_explicit_effect: true
  cut_effect: kill_body_and_astral_form
damage_and_effects_shared_between_forms: false
astral_plane_exit:
  physical_body_and_possessions: travel_along_cord
  reenter_body: new_plane
```

#### End

<!-- splat:yaml path=splat_rule/parameters/end -->

```yaml
per_target:
- body_hp_zero
- astral_form_hp_zero
all_targets:
  caster_cost: magic_action
  choice: dismiss
living_target_cleanup:
- return_to_body
- end_suspended_animation
```
