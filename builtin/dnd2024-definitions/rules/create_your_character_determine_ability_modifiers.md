---
splat_schema: 1
splat_id: c13fcaf9-e802-5ff1-805a-386448114090
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_determine_ability_modifiers
splat_name: Determine Ability Modifiers
splat_origin:
  key: dnd2024:reference/pages/021
splat_sources:
- title: SRD 5.2.1
  locator: pp. 21
  url: https://www.dndbeyond.com/srd
---

# Determine Ability Modifiers

## Source text

<!-- SRD page 21 -->

Finally, determine your ability modifiers using the Ability Scores and Modifiers table. Write the modifier next to each of your scores.

**Ability Scores and Modifiers**

| Score | Modifier | Score | Modifier |
| --- | --- | --- | --- |
| 3 | −4 | 12–13 | +1 |
| 4–5 | −3 | 14–15 | +2 |
| 6–7 | −2 | 16–17 | +3 |
| 8–9 | −1 | 18–19 | +4 |
| 10–11 | +0 | 20 | +5 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_determine_ability_modifiers ` |
| ` implementation ` | ` dnd2024:rules_create_your_character_determine_ability_modifiers ` |

### Parameters

#### Modifier

<!-- splat:yaml path=splat_rule/parameters/modifier -->

```yaml
op: floor
value:
  op: divide
  left:
    op: subtract
    left: ability_score
    right: 10
  right: 2
```

#### Source table score range

<!-- splat:values path=splat_rule/parameters/source_table_score_range -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 3 ` |
| ` maximum ` | ` 20 ` |
