---
splat_schema: 1
splat_id: 78a1d483-6af6-51d4-907e-9e994fc248ce
splat_kind: rule
splat_key: dnd2024:rules/warlock_visions_of_distant_realms
splat_name: Visions of Distant Realms
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Visions of Distant Realms

## Source text

<!-- SRD page 74 -->

Prerequisite: Level 9+ Warlock\
You can cast Arcane Eye without expending a spell\
slot.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_visions_of_distant_realms ` |
| ` implementation ` | ` dnd2024:rules_warlock_visions_of_distant_realms ` |

### Parameters

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_min ` | ` 9 ` |

#### Grant cast

<!-- splat:yaml path=splat_rule/parameters/grant_cast -->

```yaml
spell:
  key: dnd2024:spells/arcane_eye
spell_slot_cost: 0
uses: unlimited
```
