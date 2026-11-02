---
splat_schema: 1
splat_id: 912ffe59-b1cb-519d-b18a-62f944818fab
splat_kind: spell
splat_key: dnd2024:spells/modify_memory
splat_name: Modify Memory
splat_origin:
  key: dnd2024:reference/spell/modify_memory
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
---

# Modify Memory

## Source text

<!-- SRD page 150 -->

Level 5 Enchantment (Bard, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

You attempt to reshape another creature’s memories. One creature that you can see within range makes a Wisdom saving throw. If you are fighting the creature, it has Advantage on the save. On a failed save, the target has the Charmed condition for the duration. While Charmed in this way, the target also has the Incapacitated condition and is unaware of its surroundings, though it can hear you. If it takes any damage or is targeted by another spell, this spell ends, and no memories are modified.

While this charm lasts, you can affect the target’s memory of an event that it experienced within the last 24 hours and that lasted no more than 10 minutes. You can permanently eliminate all memory of the event, allow the target to recall the event with perfect clarity, change its memory of the event’s details, or create a memory of some other event.

You must speak to the target to describe how its memories are affected, and it must be able to understand your language for the modified memories to take root. Its mind fills in any gaps in the details of your description. If the spell ends before you finish describing the modified memories, the creature’s memory isn’t altered. Otherwise, the modified memories take hold when the spell ends.

A modified memory doesn’t necessarily affect how a creature behaves, particularly if the memory contradicts the creature’s natural inclinations, alignment, or beliefs. An illogical modified memory, such as a false memory of how much the creature enjoyed swimming in acid, is dismissed as a bad dream. The GM might deem a modified memory too nonsensical to affect a creature. A Remove Curse or Greater Restoration spell cast on the target restores the creature’s true memory.

Using a Higher-Level Spell Slot. You can alter the target’s memories of an event that took place up to 7 days ago (level 6 spell slot), 30 days ago (level 7 spell slot), 365 days ago (level 8 spell slot), or any time in the creature’s past (level 9 spell slot).

Original layout: [[../../srd-5.2.1/pages/150|page 150]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
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
| ` mechanic ` | ` dnd2024:spells/modify_memory ` |
| ` implementation ` | ` dnd2024:spell_modify_memory ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` other_than_caster ` | ` true ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 30 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` advantage_if ` | ` caster_fighting_target ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition: charmed
while_charmed:
  condition: incapacitated
  aware_of_surroundings: false
  can_hear_caster: true
```

#### Interrupted

<!-- splat:values path=splat_rule/parameters/interrupted -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["target.took_damage", "target.targeted_by_another_spell"] ` |
| ` end_spell ` | ` true ` |
| ` modify_memory ` | ` false ` |

#### Memory event

<!-- splat:values path=splat_rule/parameters/memory_event -->

| Field | Value |
| --- | --- |
| ` experienced_by_target ` | ` true ` |
| ` maximum_event_minutes ` | ` 10 ` |

##### History window by slot

<!-- splat:table
path: splat_rule/parameters/memory_event/history_window_by_slot
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
columns:
  Key:
    type: key
  Level:
    path: level
    type: integer
  Days:
    path: days
    type: integer
    optional: true
  History:
    path: history
    type: symbol
    optional: true
-->

| Key | Level | Days | History |
| --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 1 ` |  |
| ` row_2 ` | ` 6 ` | ` 7 ` |  |
| ` row_3 ` | ` 7 ` | ` 30 ` |  |
| ` row_4 ` | ` 8 ` | ` 365 ` |  |
| ` row_5 ` | ` 9 ` |  | ` Any time ` |

#### Modification choice

<!-- splat:values path=splat_rule/parameters/modification_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` options ` | ` ["erase_event", "perfect_recall", "change_details", "create_other_event_memory"] ` |
| ` duration ` | ` permanent ` |

#### Description

<!-- splat:values path=splat_rule/parameters/description -->

| Field | Value |
| --- | --- |
| ` spoken_to_target ` | ` true ` |
| ` target_understands_language ` | ` true ` |
| ` complete_before_spell_ends ` | ` true ` |
| ` fill_missing_details ` | ` target_mind ` |

#### Commit

<!-- splat:values path=splat_rule/parameters/commit -->

| Field | Value |
| --- | --- |
| ` event ` | ` spell.effect_ended ` |
| ` requires ` | ` completed_understood_description_and_not_interrupted ` |
| ` otherwise ` | ` unchanged_memory ` |

#### Behavior

<!-- splat:yaml path=splat_rule/parameters/behavior -->

```yaml
compelled_by_memory: false
gm_assessment:
  kind: choice
  options:
  - coherent
  - dismissed_as_bad_dream
  - too_nonsensical_to_affect
  consider:
  - natural_inclinations
  - alignment
  - beliefs
```

#### Restore true memory spells

<!-- splat:rows path=splat_rule/parameters/restore_true_memory_spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/remove_curse ` |
| ` 1 ` | ` dnd2024:spells/greater_restoration ` |
