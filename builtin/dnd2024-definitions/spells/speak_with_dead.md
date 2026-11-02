---
splat_schema: 1
splat_id: be9423d1-2ca8-544d-b016-3c882f52a1b1
splat_kind: spell
splat_key: dnd2024:spells/speak_with_dead
splat_name: Speak with Dead
splat_origin:
  key: dnd2024:reference/spell/speak_with_dead
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
---

# Speak with Dead

## Source text

<!-- SRD page 164 -->

Level 3 Necromancy (Bard, Cleric, Wizard)\
Casting Time: Action\
Range: 10 feet\
Components: V, S, M (burning incense)\
Duration: 10 minutes

You grant the semblance of life to a corpse of your choice within range, allowing it to answer questions you pose. The corpse must have a mouth, and this spell fails if the deceased creature was Undead when it died. The spell also fails if the corpse was the target of this spell within the past 10 days.

Until the spell ends, you can ask the corpse up to five questions. The corpse knows only what it knew in life, including the languages it knew. Answers are usually brief, cryptic, or repetitive, and the corpse is under no compulsion to offer a truthful answer if you are antagonistic toward it or it recognizes you as an enemy. This spell doesn’t return the creature’s soul to its body, only its animating spirit. Thus, the corpse can’t learn new information, doesn’t comprehend anything that has happened since it died, and can’t speculate about future events.

Original layout: [[../../srd-5.2.1/pages/164|page 164]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["bard", "cleric", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 10 minutes ` |
| ` casting_time ` | ` Action ` |
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
| ` description ` | ` burning incense ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/speak_with_dead ` |
| ` implementation ` | ` dnd2024:spell_speak_with_dead ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` corpse ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` mouth_required ` | ` true ` |
| ` undead_when_died ` | ` false ` |
| ` no_same_spell_target_within_days ` | ` 10 ` |

#### Questions

<!-- splat:values path=splat_rule/parameters/questions -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 5 ` |
| ` knowledge ` | ` in_life_only ` |
| ` languages ` | ` in_life_only ` |
| ` answer_style ` | ` ["brief", "cryptic", "repetitive"] ` |
| ` truth_compulsion_if_caster_antagonistic_or_known_enemy ` | ` false ` |
| ` learn_new_information ` | ` false ` |
| ` comprehend_events_after_death ` | ` false ` |
| ` speculate_future ` | ` false ` |

#### Animation

<!-- splat:values path=splat_rule/parameters/animation -->

| Field | Value |
| --- | --- |
| ` returns_soul ` | ` false ` |
| ` animating_spirit_only ` | ` true ` |
