---
splat_schema: 1
splat_id: ce7e7bb7-5224-5d68-a9ff-1501a5266ca0
splat_kind: spell
splat_key: dnd2024:spells/sending
splat_name: Sending
splat_origin:
  key: dnd2024:reference/spell/sending
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
---

# Sending

## Source text

<!-- SRD page 161 -->

Level 3 Divination (Bard, Cleric, Wizard)\
Casting Time: Action\
Range: Unlimited\
Components: V, S, M (a copper wire)\
Duration: Instantaneous

You send a short message of 25 words or fewer to a creature you have met or a creature described to you by someone who has met it. The target hears the message in its mind, recognizes you as the sender if it knows you, and can answer in a like manner immediately. The spell enables targets to understand the meaning of your message.

You can send the message across any distance and even to other planes of existence, but if the target is on a different plane than you, there is a 5 percent chance that the message doesn’t arrive. You know if the delivery fails.

Upon receiving your message, a creature can block your ability to reach it again with this spell for 8 hours. If you try to send another message during that time, you learn that you are blocked, and the spell fails.

Original layout: [[../../srd-5.2.1/pages/161|page 161]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` unlimited ` |

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
| ` mechanic ` | ` dnd2024:spells/sending ` |
| ` implementation ` | ` dnd2024:spell_sending ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` eligibility_any ` | ` ["caster_has_met", "described_by_someone_who_has_met"] ` |
| ` distance ` | ` unlimited ` |
| ` same_plane_required ` | ` false ` |

#### Message

<!-- splat:yaml path=splat_rule/parameters/message -->

```yaml
word_limit: 25
channel: mental
recognize_sender_if_target_knows_caster: true
understands_meaning: true
reply:
  optional: true
  timing: immediate
  same_manner: true
  word_limit: 25
```

#### Cross plane

<!-- splat:values path=splat_rule/parameters/cross_plane -->

| Field | Value |
| --- | --- |
| ` delivery_failure_chance_percent ` | ` 5 ` |
| ` caster_knows_failure ` | ` true ` |

#### Block

<!-- splat:yaml path=splat_rule/parameters/block -->

```yaml
choice: recipient_after_receiving_message
duration_hours: 8
scope: same_caster_sending
attempt_while_blocked:
  spell_fails: true
  caster_learns_blocked: true
```
