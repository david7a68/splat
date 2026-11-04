---
splat_schema: 2
splat_id: 3ee1085a-5278-5c47-840b-2ad360958a83
splat_kind: rule
splat_key: dnd2024:rules/warlock_misty_visions
splat_name: Misty Visions
splat_origin: dnd2024:reference/pages/074
splat_sources:
- SRD 5.2.1, pp. 74, https://www.dndbeyond.com/srd
---

# Misty Visions

## Source text

<!-- SRD page 74 -->

Prerequisite: Level 2+ Warlock

You can cast Silent Image without expending a spell slot.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_misty_visions ` |
| ` implementation ` | ` dnd2024:rule_warlock_misty_visions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 2 ` |

#### Cast

<!-- splat:yaml path=splat_rule/parameters/cast -->

```yaml
spell:
  key: dnd2024:spells/silent_image
spell_slot_cost: 0
target: spell_definition
uses: unlimited
```
