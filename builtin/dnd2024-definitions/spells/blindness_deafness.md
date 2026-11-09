---
splat_schema: 2
splat_id: 2d64dfd8-a1b9-569d-94fb-30e2b141dfd9
splat_kind: spell
splat_key: dnd2024:spells/blindness_deafness
splat_name: Blindness/Deafness
splat_origin: dnd2024:reference/spell/blindness_deafness
splat_sources:
- SRD 5.2.1, pp. 113, https://www.dndbeyond.com/srd
---

# Blindness/Deafness


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Bard, Cleric, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V |
| Duration | 1 minute |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 113 -->

Level 2 Transmutation (Bard, Cleric, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V\
Duration: 1 minute

One creature that you can see within range must succeed on a Constitution saving throw, or it has the Blinded or Deafened condition (your choice) for the duration. At the end of each of its turns, the target repeats the save, ending the spell on itself on a success.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/113|page 113]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/blindness_deafness ` |
| ` implementation ` | ` dnd2024:spell_blindness_deafness ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_choice ` | ` ["blinded", "deafened"] ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  binding: save_dc
failure:
  condition: chosen
  duration: spell
success:
  effect: none
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.end ` |
| ` ability ` | ` constitution ` |
| ` success ` | ` end_spell_on_self ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 2 ` |
| ` additional_targets ` | ` 1 ` |
