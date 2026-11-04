---
splat_schema: 2
splat_id: 2accb2d8-94db-5675-b3b7-8834b493fbe2
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_damage_notation
splat_name: Damage Notation
splat_origin: dnd2024:reference/pages/257
splat_sources:
- SRD 5.2.1, pp. 257, https://www.dndbeyond.com/srd
---

# Damage Notation

## Source text

<!-- SRD page 257 -->

A stat block usually provides both a number and a die expression for each instance of damage. For example, an attack might deal 4 (1d4 + 2) damage on a hit. You decide whether to use the number or the die expression in parentheses; don’t use both.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_damage_notation ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_damage_notation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` combine_modes ` | ` false ` |

#### Damage mode choice

<!-- splat:values path=splat_rule/parameters/damage_mode_choice -->

| Field | Value |
| --- | --- |
| ` options ` | ` ["fixed_average", "dice_expression"] ` |
| ` count ` | ` 1 ` |
| ` authority ` | ` gm ` |
