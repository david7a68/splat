---
splat_schema: 1
splat_id: ba64b943-b28b-566d-b991-fc8bb7bb2c61
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_abilities
splat_name: Abilities
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
---

# Abilities

## Source text

<!-- SRD page 207 -->

A sentient magic item has Intelligence, Wisdom, and Charisma scores. Choose the item’s abilities, or determine them randomly as follows: roll 4d6 for each one, dropping the lowest roll and totaling the rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sentient_magic_items_abilities ` |
| ` implementation ` | ` dnd2024:rule_sentient_magic_items_abilities ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` abilities ` | ` ["intelligence", "wisdom", "charisma"] ` |
| ` generation_choice ` | ` ["gm_select", "random"] ` |

#### Random each

<!-- splat:values path=splat_rule/parameters/random_each -->

| Field | Value |
| --- | --- |
| ` dice_count ` | ` 4 ` |
| ` die_sides ` | ` 6 ` |
| ` drop_lowest ` | ` 1 ` |
| ` sum_remaining ` | ` true ` |
