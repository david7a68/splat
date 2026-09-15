---
splat_schema: 1
splat_id: f358b1f0-0e7c-5853-9dd4-4b52a8da47f5
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_casting_time
splat_name: Casting Time
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
---

# Casting Time

## Source text

<!-- SRD page 105 -->

Most spells require the Magic action to cast, but\
some spells require a Bonus Action, a Reaction, or 1\
minute or more. A spell’s Casting Time entry speci-\
fies which of those is required.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_casting_time ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_casting_time ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` casting_economy_source ` | ` spell_casting_time ` |
| ` default_economy ` | ` magic_action ` |
| ` other_casting_times ` | ` ["bonus_action", "reaction", "one_minute_or_longer"] ` |
