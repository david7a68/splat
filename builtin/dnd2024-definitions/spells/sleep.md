---
splat_schema: 1
splat_id: aecb4178-b2fb-5b41-9610-a5ad9a398f94
splat_kind: spell
splat_key: dnd2024:spells/sleep
splat_name: Sleep
splat_origin:
  key: dnd2024:reference/spell/sleep
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
---

# Sleep

## Source text

<!-- SRD page 163 -->

Level 1 Enchantment (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a pinch of sand or rose petals)\
Duration: Concentration, up to 1 minute

Each creature of your choice in a 5-foot-radius Sphere centered on a point within range must succeed on a Wisdom saving throw or have the Incapacitated condition until the end of its next turn, at which point it must repeat the save. If the target fails the second save, the target has the Unconscious condition for the duration. The spell ends on a target if it takes damage or someone within 5 feet of it takes an action to shake it out of the spell’s effect.

Creatures that don’t sleep, such as elves, or that have Immunity to the Exhaustion condition automatically succeed on saves against this spell.

Original layout: [[../../srd-5.2.1/pages/163|page 163]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pinch of sand or rose petals ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sleep ` |
| ` implementation ` | ` dnd2024:spell_sleep ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` automatic_save_success_if_any ` | ` ["does_not_sleep", "immune_to_exhaustion"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 5 ` |
| ` origin ` | ` chosen_point_within_range ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` in_area ` | ` true ` |
| ` chooser ` | ` caster ` |

#### Initial save

<!-- splat:yaml path=splat_rule/parameters/initial_save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: incapacitated
  until: target_next_turn_end
  then_repeat_save:
    ability: wisdom
    dc:
      binding: save_dc
    failure:
      condition: unconscious
      duration: spell_duration
    success:
      effect: none
success:
  effect: none
```

#### End on target

<!-- splat:yaml path=splat_rule/parameters/end_on_target -->

```yaml
- takes_damage
- another_creature:
    within_feet: 5
    cost: action
    activity: shake_awake
```
