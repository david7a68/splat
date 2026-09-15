---
splat_schema: 1
splat_id: 50772e5a-4ec9-514c-aafe-0302af9a3ed4
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_bestow_curse
splat_name: Bestow Curse
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
---

# Bestow Curse

## Source text

<!-- SRD page 193 -->

The simplest curses are created by the Bestow Curse\
spell. The effects of such curses are limited and can\
be ended by the Remove Curse spell.\
Bestow Curse provides useful benchmarks for\
gauging the potency of other curses. A curse that\
lasts for 1 minute equates to a level 3 spell, while\
one that lasts until dispelled equates to a level 9\
spell.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_bestow_curse ` |
| ` implementation ` | ` dnd2024:rules_magical_contagions_bestow_curse ` |

### Parameters

#### Curse

<!-- splat:values path=splat_rule/parameters/curse -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bestow_curse ` |

#### Ended by

<!-- splat:values path=splat_rule/parameters/ended_by -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/remove_curse ` |

#### Potency benchmarks

<!-- splat:rows path=splat_rule/parameters/potency_benchmarks -->

| Row | spell_level | duration |
| --- | --- | --- |
| ` 0 ` | ` 3 ` |  |
| ` 1 ` | ` 9 ` | ` until_dispelled ` |

##### Duration

<!-- splat:values path=splat_rule/parameters/potency_benchmarks/0/duration -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` minute ` |
