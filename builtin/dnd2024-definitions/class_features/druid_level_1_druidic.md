---
splat_schema: 1
splat_id: 69fa7c15-e235-5184-bfee-dcc1f9731f0b
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_1_druidic
splat_name: 'Level 1: Druidic'
splat_origin:
  key: dnd2024:reference/pages/042
splat_sources:
- title: SRD 5.2.1
  locator: pp. 42
  url: https://www.dndbeyond.com/srd
---

# Level 1: Druidic

## Source text

<!-- SRD page 42 -->

You know Druidic, the secret language of Druids. While learning this ancient tongue, you also unlocked the magic of communicating with animals; you always have the Speak with Animals spell prepared.

You can use Druidic to leave hidden messages. You and others who know Druidic automatically spot such a message. Others spot the message’s presence with a successful DC 15 Intelligence (Investigation) check but can’t decipher it without magic.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_1_druidic ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_1_druidic ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` language_grant ` | ` druidic ` |

#### Always prepared

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/speak_with_animals ` |

#### Hidden messages

<!-- splat:yaml path=splat_rule/parameters/hidden_messages -->

```yaml
can_create: true
druidic_knowers:
  automatically_spot: true
  can_read: true
others:
  spot_check:
    ability: intelligence
    skill: investigation
    dc: 15
  decipher_requires_magic: true
```
