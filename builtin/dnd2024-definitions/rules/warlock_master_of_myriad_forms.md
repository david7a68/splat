---
splat_schema: 1
splat_id: 460b9e04-03ad-565c-8b97-c6c6454ed2af
splat_kind: rule
splat_key: dnd2024:rules/warlock_master_of_myriad_forms
splat_name: Master of Myriad Forms
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Master of Myriad Forms

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock\
You can cast Alter Self without expending a spell\
slot.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_master_of_myriad_forms ` |
| ` implementation ` | ` dnd2024:rules_warlock_master_of_myriad_forms ` |

### Parameters

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_min ` | ` 5 ` |

#### Grant cast

<!-- splat:yaml path=splat_rule/parameters/grant_cast -->

```yaml
spell:
  key: dnd2024:spells/alter_self
spell_slot_cost: 0
uses: unlimited
```
