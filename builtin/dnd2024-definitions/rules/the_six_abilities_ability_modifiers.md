---
splat_schema: 2
splat_id: 863d1efd-e5c9-58c1-9c18-0181f3a7ac24
splat_kind: rule
splat_key: dnd2024:rules/the_six_abilities_ability_modifiers
splat_name: Ability Modifiers
splat_origin: dnd2024:reference/pages/005
splat_sources:
- SRD 5.2.1, pp. 5, 6, https://www.dndbeyond.com/srd
---

# Ability Modifiers

## Source text

<!-- SRD page 5 -->

Each ability has a modifier that you apply whenever you make a D20 Test with that ability (explained in “D20 Tests”). An ability modifier is derived from its score, as shown in the Ability Modifiers table.

### Round Down

Whenever you divide or multiply a number in the game, round down if you end up with a fraction, even if the fraction is one-half or greater. Some rules make an exception and tell you to round up.

<!-- SRD page 6 -->

**Ability Modifiers**

| Score | Modifier | Score | Modifier |
| --- | --- | --- | --- |
| 1 | −5 | 16–17 | +3 |
| 2–3 | −4 | 18–19 | +4 |
| 4–5 | −3 | 20–21 | +5 |
| 6–7 | −2 | 22–23 | +6 |
| 8–9 | −1 | 24–25 | +7 |
| 10–11 | +0 | 26–27 | +8 |
| 12–13 | +1 | 28–29 | +9 |
| 14–15 | +2 | 30 | +10 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/the_six_abilities_ability_modifiers ` |
| ` implementation ` | ` dnd2024:rule_the_six_abilities_ability_modifiers ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ability_score_range ` | ` [1, 30] ` |
| ` apply_to ` | ` d20_test_using_ability ` |
| ` fractional_multiplication_and_division_rounding ` | ` floor ` |
| ` explicit_round_up_overrides ` | ` true ` |

#### Ability modifier

<!-- splat:yaml path=splat_rule/parameters/ability_modifier -->

```yaml
op: floor
args:
- op: divide
  args:
  - op: subtract
    args:
    - variable: ability_score
    - 10
  - 2
```
