---
splat_schema: 2
splat_id: cec90d8e-7a1e-533b-ac52-1cb607f5e548
splat_kind: spell
splat_key: dnd2024:spells/ray_of_enfeeblement
splat_name: Ray of Enfeeblement
splat_origin: dnd2024:reference/spell/ray_of_enfeeblement
splat_sources:
- SRD 5.2.1, pp. 157, https://www.dndbeyond.com/srd
---

# Ray of Enfeeblement


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Necromancy |
| Classes | Warlock, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 157 -->

Level 2 Necromancy (Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

A beam of enervating energy shoots from you toward a creature within range. The target must make a Constitution saving throw. On a successful save, the target has Disadvantage on the next attack roll it makes until the start of your next turn.

On a failed save, the target has Disadvantage on Strength-based D20 Tests for the duration. During that time, it also subtracts 1d8 from all its damage rolls. The target repeats the save at the end of each of its turns, ending the spell on a success.

Original layout: [[../../srd-5.2.1/pages/157|page 157]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/ray_of_enfeeblement ` |
| ` implementation ` | ` dnd2024:spell_ray_of_enfeeblement ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  binding: save_dc
failure:
  duration: spell_duration
  disadvantage:
    rolls: d20_tests
    ability: strength
  damage_roll_modifier:
    all_damage_rolls: true
    operation: subtract
    amount:
      dice:
        count: 1
        sides: 8
        add: 0
  repeat_save:
    event: target_turn_end
    success: end_spell
success:
  disadvantage_next_attack: true
  until: caster_next_turn_start
```
