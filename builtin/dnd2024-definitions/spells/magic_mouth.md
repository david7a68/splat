---
splat_schema: 1
splat_id: e15d7476-d650-5b00-a02a-e6254044cf76
splat_kind: spell
splat_key: dnd2024:spells/magic_mouth
splat_name: Magic Mouth
splat_origin:
  key: dnd2024:reference/spell/magic_mouth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 146
  url: https://www.dndbeyond.com/srd
---

# Magic Mouth

## Source text

<!-- SRD page 146 -->

Level 2 Illusion (Bard, Wizard)\
Casting Time: 1 minute or Ritual\
Range: 30 feet\
Components: V, S, M (jade dust worth 10+ GP, which\
the spell consumes)\
Duration: Until dispelled\
You implant a message within an object in range—a\
message that is uttered when a trigger condition is\
met. Choose an object that you can see and that isn’t\
being worn or carried by another creature. Then\
speak the message, which must be 25 words or\
fewer, though it can be delivered over as long as 10\
minutes. Finally, determine the circumstance that\
will trigger the spell to deliver your message.\
When that trigger occurs, a magical mouth ap-\
pears on the object and recites the message in your\
voice and at the same volume you spoke. If the ob-\
ject you chose has a mouth or something that looks\
like a mouth (for example, the mouth of a statue),\
the magical mouth appears there, so the words\
appear to come from the object’s mouth. When you\
cast this spell, you can have the spell end after it\
delivers its message, or it can remain and repeat its\
message whenever the trigger occurs.\
The trigger can be as general or as detailed as you\
like, though it must be based on visual or audible\
conditions that occur within 30 feet of the object.\
For example, you could instruct the mouth to speak\
when any creature moves within 30 feet of the ob-\
ject or when a silver bell rings within 30 feet of it.\

Original layout: [[../../srd-5.2.1/pages/146|page 146]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled ` |
| ` casting_time ` | ` 1 minute or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` jade dust worth 10+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/magic_mouth ` |
| ` implementation ` | ` dnd2024:spell_magic_mouth ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` until ` | ` dispelled_or_source_specific_end ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 30 ` |
| ` worn_or_carried_by_another ` | ` false ` |

#### Message

<!-- splat:values path=splat_rule/parameters/message -->

| Field | Value |
| --- | --- |
| ` spoken_at_cast ` | ` true ` |
| ` maximum_words ` | ` 25 ` |
| ` maximum_delivery_seconds ` | ` 600 ` |
| ` playback_voice ` | ` caster_voice_at_cast ` |
| ` playback_volume ` | ` spoken_volume_at_cast ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` timing ` | ` cast ` |
| ` allowed_modalities ` | ` ["visual", "audible"] ` |
| ` maximum_distance_from_object_feet ` | ` 30 ` |
| ` condition_resolution ` | ` supplied_trigger_predicate ` |

#### Delivery

<!-- splat:values path=splat_rule/parameters/delivery -->

| Field | Value |
| --- | --- |
| ` event ` | ` declared_trigger.occurred ` |
| ` mouth_appears ` | ` true ` |
| ` mouth_location ` | ` existing_mouth_or_mouthlike_feature_if_present ` |

#### Recurrence choice

<!-- splat:values path=splat_rule/parameters/recurrence_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` timing ` | ` cast ` |
| ` options ` | ` ["end_after_delivery", "repeat_on_each_trigger"] ` |
