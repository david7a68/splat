---
splat_schema: 1
splat_id: 1c510fca-c984-5d0a-80e7-6a53fa65f24e
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls_cantrips
splat_name: Cantrips
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
---

# Cantrips

## Source text

<!-- SRD page 103 -->

If the scribed spell is a cantrip, the version on the\
scroll works as if the caster were your level.\
Spell Scroll Costs\
Spell Level Time Cost\
Cantrip 1 day 15 GP\
1 1 day 25 GP\
2 3 days 100 GP\
3 5 days 150 GP\
4 10 days 1,000 GP\
5 25 days 1,500 GP\
6 40 days 10,000 GP\
7 50 days 12,500 GP\
8 60 days 15,000 GP\
9 120 days 50,000 GP\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/scribing_spell_scrolls_cantrips ` |
| ` implementation ` | ` dnd2024:rules_scribing_spell_scrolls_cantrips ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cantrip_effective_caster_level ` | ` scribe_level ` |

#### Scroll costs

<!-- splat:rows path=splat_rule/parameters/scroll_costs -->

| Row | spell_level | days | cost_gp |
| --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` 1 ` | ` 15 ` |
| ` 1 ` | ` 1 ` | ` 1 ` | ` 25 ` |
| ` 2 ` | ` 2 ` | ` 3 ` | ` 100 ` |
| ` 3 ` | ` 3 ` | ` 5 ` | ` 150 ` |
| ` 4 ` | ` 4 ` | ` 10 ` | ` 1000 ` |
| ` 5 ` | ` 5 ` | ` 25 ` | ` 1500 ` |
| ` 6 ` | ` 6 ` | ` 40 ` | ` 10000 ` |
| ` 7 ` | ` 7 ` | ` 50 ` | ` 12500 ` |
| ` 8 ` | ` 8 ` | ` 60 ` | ` 15000 ` |
| ` 9 ` | ` 9 ` | ` 120 ` | ` 50000 ` |
