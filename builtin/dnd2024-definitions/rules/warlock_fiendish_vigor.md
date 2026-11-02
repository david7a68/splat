---
splat_schema: 1
splat_id: f9b2dd51-e21e-5589-ba8f-6e6fb4a551f4
splat_kind: rule
splat_key: dnd2024:rules/warlock_fiendish_vigor
splat_name: Fiendish Vigor
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Fiendish Vigor

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 2+ Warlock

You can cast False Life on yourself without expending a spell slot. When you cast the spell with this feature, you don’t roll the die for the Temporary Hit Points; you automatically get the highest number on the die.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_fiendish_vigor ` |
| ` implementation ` | ` dnd2024:rule_warlock_fiendish_vigor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 2 ` |

#### Cast

<!-- splat:yaml path=splat_rule/parameters/cast -->

```yaml
spell:
  key: dnd2024:spells/false_life
spell_slot_cost: 0
target: self
uses: unlimited
temporary_hit_points_dice_result: maximum
```
