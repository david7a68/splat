---
splat_schema: 1
splat_id: 4c950cfd-53fd-5274-8288-6b3c76a44b29
splat_kind: rule
splat_key: dnd2024:rules/warlock_eldritch_spear
splat_name: Eldritch Spear
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Eldritch Spear

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 2+ Warlock, a Warlock Cantrip\
That Deals Damage\
Choose one of your known Warlock cantrips that\
deals damage and has a range of 10+ feet. When you\
cast that spell, its range increases by a number of\
feet equal to 30 times your Warlock level.\
Repeatable. You can gain this invocation more\
than once. Each time you do so, choose a different\
eligible cantrip.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_eldritch_spear ` |
| ` implementation ` | ` dnd2024:rules_warlock_eldritch_spear ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` repeatable ` | ` true ` |
| ` repeat_selection ` | ` different_eligible_cantrip ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_min ` | ` 2 ` |

#### Selection

<!-- splat:values path=splat_rule/parameters/selection -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` known ` | ` true ` |
| ` class ` | ` warlock ` |
| ` level ` | ` 0 ` |
| ` deals_damage ` | ` true ` |
| ` range_min_ft ` | ` 10 ` |

#### Range increase ft

<!-- splat:yaml path=splat_rule/parameters/range_increase_ft -->

```yaml
op: multiply
args:
- 30
- var: subject.warlock_level
```
