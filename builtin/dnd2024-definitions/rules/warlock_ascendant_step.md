---
splat_schema: 2
splat_id: 2670d539-a57a-511b-8bb7-7588a4e8da93
splat_kind: rule
splat_key: dnd2024:rules/warlock_ascendant_step
splat_name: Ascendant Step
splat_origin: dnd2024:reference/pages/072
splat_sources:
- SRD 5.2.1, pp. 72, https://www.dndbeyond.com/srd
---

# Ascendant Step

## Source text

<!-- SRD page 72 -->

Prerequisite: Level 5+ Warlock

You can cast Levitate on yourself without expending a spell slot.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_ascendant_step ` |
| ` implementation ` | ` dnd2024:rule_warlock_ascendant_step ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 5 ` |

#### Cast

<!-- splat:yaml path=splat_rule/parameters/cast -->

```yaml
spell:
  key: dnd2024:spells/levitate
spell_slot_cost: 0
target: self
uses: unlimited
```
