---
splat_schema: 2
splat_id: 8b7bc468-6474-58f7-81fb-da94f040c7d7
splat_kind: rule
splat_key: dnd2024:glossary/ally
splat_name: Ally
splat_origin: dnd2024:reference/glossary/ally
splat_sources:
- SRD 5.2.1, pp. 176, https://www.dndbeyond.com/srd
---

# Ally

## Source text

<!-- SRD page 176 -->

A creature is your ally if it is a member of your adventuring party, your friend, on your side in combat, or a creature that the rules or the GM designates as your ally.

Original layout: [[../../srd-5.2.1/pages/176|page 176]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/ally ` |
| ` implementation ` | ` dnd2024:glossary_ally ` |

### Parameters

#### Predicate

<!-- splat:values path=splat_rule/parameters/predicate -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["same_adventuring_party", "friend", "same_combat_side", "rule_designated_ally", "gm_designated_ally"] ` |

#### Gm designation

<!-- splat:values path=splat_rule/parameters/gm_designation -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` options ` | ` [true, false] ` |
