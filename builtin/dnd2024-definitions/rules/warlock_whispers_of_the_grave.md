---
splat_schema: 1
splat_id: f9838231-2dc8-5eed-9fb3-4a4345633755
splat_kind: rule
splat_key: dnd2024:rules/warlock_whispers_of_the_grave
splat_name: Whispers of the Grave
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Whispers of the Grave

## Source text

<!-- SRD page 74 -->

Prerequisite: Level 7+ Warlock

You can cast Speak with Dead without expending a spell slot.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_whispers_of_the_grave ` |
| ` implementation ` | ` dnd2024:rule_warlock_whispers_of_the_grave ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 7 ` |

#### Cast

<!-- splat:yaml path=splat_rule/parameters/cast -->

```yaml
spell:
  key: dnd2024:spells/speak_with_dead
spell_slot_cost: 0
target: spell_definition
uses: unlimited
```
