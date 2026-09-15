---
splat_schema: 1
splat_id: b8a9f899-c1a9-5da3-b770-90f84c5c8bf2
splat_kind: rule
splat_key: dnd2024:glossary/disadvantage
splat_name: Disadvantage
splat_origin:
  key: dnd2024:reference/glossary/disadvantage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
---

# Disadvantage

## Source text

<!-- SRD page 181 -->

If you have Disadvantage on a D20 Test, roll two\
d20s and use the lower roll. A roll can’t be affected\
by more than one Disadvantage, and Advantage and\
Disadvantage on the same roll cancel each other. See\
also “Playing the Game” (“D20 Tests”).\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/disadvantage ` |
| ` implementation ` | ` dnd2024:glossary_disadvantage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test_kinds ` | ` ["ability_check", "attack_roll", "saving_throw"] ` |
| ` same_kind_stacking ` | ` false ` |

#### Dice

<!-- splat:values path=splat_rule/parameters/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sides ` | ` 20 ` |
| ` select ` | ` lowest ` |

#### Opposite kind cancellation

<!-- splat:yaml path=splat_rule/parameters/opposite_kind_cancellation -->

```yaml
any_advantage_and_any_disadvantage: true
result:
  count: 1
  sides: 20
ignore_source_counts: true
```
