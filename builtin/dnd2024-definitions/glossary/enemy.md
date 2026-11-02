---
splat_schema: 1
splat_id: 8f020241-9708-53d0-ba8b-333855533507
splat_kind: rule
splat_key: dnd2024:glossary/enemy
splat_name: Enemy
splat_origin:
  key: dnd2024:reference/glossary/enemy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
---

# Enemy

## Source text

<!-- SRD page 181 -->

A creature is your enemy if it fights against you in combat, actively works to harm you, or is designated as your enemy by the rules or GM.

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/enemy ` |
| ` implementation ` | ` dnd2024:glossary_enemy ` |

### Parameters

#### Predicate

<!-- splat:values path=splat_rule/parameters/predicate -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["fights_against_subject", "actively_works_to_harm_subject", "rule_designated_enemy", "gm_designated_enemy"] ` |

#### Gm designation

<!-- splat:values path=splat_rule/parameters/gm_designation -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` options ` | ` [true, false] ` |
