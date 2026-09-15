---
splat_schema: 1
splat_id: 995c14de-d1b0-535c-90c3-b03475366d07
splat_kind: rule
splat_key: dnd2024:glossary/advantage
splat_name: Advantage
splat_origin:
  key: dnd2024:reference/glossary/advantage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
---

# Advantage

## Source text

<!-- SRD page 176 -->

If you have Advantage on a D20 Test, roll two d20s,\
and use the higher roll. A roll can’t be affected by\
more than one Advantage, and Advantage and Dis-\
advantage on the same roll cancel each other. See\
also “Playing the Game” (“D20 Tests”).\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/advantage ` |
| ` implementation ` | ` dnd2024:glossary_advantage ` |

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
| ` select ` | ` highest ` |

#### Opposite kind cancellation

<!-- splat:yaml path=splat_rule/parameters/opposite_kind_cancellation -->

```yaml
any_advantage_and_any_disadvantage: true
result:
  count: 1
  sides: 20
ignore_source_counts: true
```
