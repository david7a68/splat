---
splat_schema: 2
splat_id: a8f89c11-3734-5cf2-85c3-02f60ec49335
splat_kind: spell
splat_key: dnd2024:spells/telekinesis
splat_name: Telekinesis
splat_origin: dnd2024:reference/spell/telekinesis
splat_sources:
- SRD 5.2.1, pp. 167, 168, https://www.dndbeyond.com/srd
---

# Telekinesis


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Transmutation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 167 -->

Level 5 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You gain the ability to move or manipulate creatures or objects by thought. When you cast the spell and as a Magic action on your later turns before the spell ends, you can exert your will on one creature or object that you can see within range, causing the appropriate effect below. You can affect the same target round after round or choose a new one at any time. If you switch targets, the prior target is no longer affected by the spell.

Creature. You can try to move a Huge or smaller creature. The target must succeed on a Strength saving throw, or you move it up to 30 feet in any direction within the spell’s range. Until the end of your next turn, the creature has the Restrained condition, and if you lift it into the air, it is suspended there. It falls at the end of your next turn unless you use this option on it again and it fails the save.

Object. You can try to move a Huge or smaller object. If the object isn’t being worn or carried, you automatically move it up to 30 feet in any direction within the spell’s range.

<!-- SRD page 168 -->

If the object is worn or carried by a creature, that creature must succeed on a Strength saving throw, or you pull the object away and move it up to 30 feet in any direction within the spell’s range.

You can exert fine control on objects with your telekinetic grip, such as manipulating a simple tool, opening a door or a container, stowing or retrieving an item from an open container, or pouring the contents from a vial.

Original layout: [[../../srd-5.2.1/pages/167|page 167]], [[../../srd-5.2.1/pages/168|page 168]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/telekinesis ` |
| ` implementation ` | ` dnd2024:spell_telekinesis ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |
| ` maximum_size ` | ` huge ` |

#### Use

<!-- splat:values path=splat_rule/parameters/use -->

| Field | Value |
| --- | --- |
| ` initial ` | ` spell.cast_completed ` |
| ` repeat_action ` | ` magic ` |
| ` repeat_timing ` | ` later_caster_turns ` |
| ` switch_target ` | ` ends_prior_target_effect ` |

#### Creature

<!-- splat:yaml path=splat_rule/parameters/creature -->

```yaml
save:
  ability: strength
  dc:
    binding: save_dc
  failure:
    move_maximum_feet: 30
    direction: any
    destination_within_spell_range: true
    condition: restrained
    until: caster.next_turn.end
    airborne: suspended
  success:
    effect: none
fall_at: caster.next_turn.end
renew_suspension_requires:
- reuse_on_target
- target_fails_save
```

#### Object

<!-- splat:yaml path=splat_rule/parameters/object -->

```yaml
unworn_uncarried: automatic
worn_or_carried:
  holder_save:
    ability: strength
    dc:
      binding: save_dc
    failure:
      pull_away: true
      move: true
    success:
      effect: none
move_maximum_feet: 30
direction: any
destination_within_spell_range: true
fine_control: true
simple_tool_manipulation: true
```
