---
splat_schema: 1
splat_id: 80ef0c51-5868-51d4-a1a1-33676b083151
splat_kind: rule
splat_key: dnd2024:rules/warlock_armor_of_shadows
splat_name: Armor of Shadows
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
---

# Armor of Shadows

## Source text

<!-- SRD page 72 -->

You can cast Mage Armor on yourself without expending a spell slot.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_armor_of_shadows ` |
| ` implementation ` | ` dnd2024:rules_warlock_armor_of_shadows ` |

### Parameters

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_min ` | ` 0 ` |

#### Grant cast

<!-- splat:yaml path=splat_rule/parameters/grant_cast -->

```yaml
spell:
  key: dnd2024:spells/mage_armor
spell_slot_cost: 0
uses: unlimited
target: self
```
