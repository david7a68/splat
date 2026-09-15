---
splat_schema: 1
splat_id: a118c863-9c91-5fb9-8fcb-36511d325bb3
splat_kind: spell
splat_key: dnd2024:spells/message
splat_name: Message
splat_origin:
  key: dnd2024:reference/spell/message
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148, 149
  url: https://www.dndbeyond.com/srd
---

# Message

## Source text

<!-- SRD page 148 -->

Transmutation Cantrip (Bard, Druid, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: S, M (a copper wire)\
Duration: 1 round\

<!-- SRD page 149 -->

You point toward a creature within range and\
whisper a message. The target (and only the target)\
hears the message and can reply in a whisper that\
only you can hear.\
You can cast this spell through solid objects if you\
are familiar with the target and know it is beyond\
the barrier. Magical silence; 1 foot of stone, metal,\
or wood; or a thin sheet of lead blocks the spell.\

Original layout: [[../../srd-5.2.1/pages/148|page 148]], [[../../srd-5.2.1/pages/149|page 149]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 round ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a copper wire ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/message ` |
| ` implementation ` | ` dnd2024:spell_message ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` point_toward_target ` | ` true ` |
| ` caster_whispers ` | ` true ` |
| ` message_heard_only_by ` | ` target ` |
| ` reply_optional ` | ` true ` |
| ` reply_whispered ` | ` true ` |
| ` reply_heard_only_by ` | ` caster ` |

#### Through solid objects

<!-- splat:yaml path=splat_rule/parameters/through_solid_objects -->

```yaml
requires:
- caster_familiar_with_target
- caster_knows_target_beyond_barrier
blocked_by:
- kind: magical_silence
- materials:
  - stone
  - metal
  - wood
  minimum_thickness_feet: 1
- material: lead
  any_thin_sheet: true
```
