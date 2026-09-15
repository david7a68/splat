---
splat_schema: 1
splat_id: 2cdbf241-d2da-5938-afc8-0850fddc7ffd
splat_kind: rule
splat_key: dnd2024:rules/warlock_otherworldly_leap
splat_name: Otherworldly Leap
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Otherworldly Leap

## Source text

<!-- SRD page 74 -->

Prerequisite: Level 2+ Warlock\
You can cast Jump on yourself without expending a\
spell slot.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_otherworldly_leap ` |
| ` implementation ` | ` dnd2024:rules_warlock_otherworldly_leap ` |

### Parameters

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_min ` | ` 2 ` |

#### Grant cast

<!-- splat:yaml path=splat_rule/parameters/grant_cast -->

```yaml
spell:
  key: dnd2024:spells/jump
spell_slot_cost: 0
uses: unlimited
target: self
```
