---
splat_schema: 1
splat_id: d651b2ce-1539-507f-aaf0-009ae6bda075
splat_kind: spell
splat_key: dnd2024:spells/seeming
splat_name: Seeming
splat_origin:
  key: dnd2024:reference/spell/seeming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160, 161
  url: https://www.dndbeyond.com/srd
---

# Seeming

## Source text

<!-- SRD page 160 -->

Level 5 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 8 hours

You give an illusory appearance to each creature of your choice that you can see within range. An unwilling target can make a Charisma saving throw, and if it succeeds, it is unaffected by this spell.

You can give the same appearance or different ones to the targets. The spell can change the appearance of the targets’ bodies and equipment. You can make each creature seem 1 foot shorter or taller and appear heavier or lighter. A target’s new appearance must have the same basic arrangement of limbs as the target, but the extent of the illusion is otherwise up to you. The spell lasts for the duration.

<!-- SRD page 161 -->

The changes wrought by this spell fail to hold up to physical inspection. For example, if you use this spell to add a hat to a creature’s outfit, objects pass through the hat.

A creature that takes the Study action to examine a target can make an Intelligence (Investigation) check against your spell save DC. If it succeeds, it becomes aware that the target is disguised.

Original layout: [[../../srd-5.2.1/pages/160|page 160]], [[../../srd-5.2.1/pages/161|page 161]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/seeming ` |
| ` implementation ` | ` dnd2024:spell_seeming ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures ` |
| ` chosen ` | ` true ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Unwilling

<!-- splat:values path=splat_rule/parameters/unwilling -->

| Field | Value |
| --- | --- |
| ` save_optional ` | ` true ` |
| ` ability ` | ` charisma ` |
| ` success ` | ` unaffected ` |

#### Illusion

<!-- splat:values path=splat_rule/parameters/illusion -->

| Field | Value |
| --- | --- |
| ` appearance_per_target ` | ` same_or_different_caster_choice ` |
| ` includes ` | ` ["body", "equipment"] ` |
| ` height_change_max_feet ` | ` 1 ` |
| ` weight_appearance ` | ` ["heavier", "lighter"] ` |
| ` limb_arrangement ` | ` same_basic ` |
| ` remaining_appearance ` | ` caster_choice ` |
| ` duration ` | ` spell_duration ` |
| ` physical_inspection ` | ` fails ` |
| ` physical_substance ` | ` false ` |

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
economy: study_action
check:
  ability: intelligence
  skill: investigation
  dc: spell_save_dc
success: observer_knows_disguised
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 8
  unit: hour
```
