---
splat_schema: 1
splat_id: 1a205b4b-a700-5650-92f9-e6363618c8e7
splat_kind: rule
splat_key: dnd2024:rules/warlock_repelling_blast
splat_name: Repelling Blast
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Repelling Blast

## Source text

<!-- SRD page 74 -->

Prerequisite: Level 2+ Warlock, a Warlock Cantrip\
That Deals Damage via an Attack Roll\
Choose one of your known Warlock cantrips that\
requires an attack roll. When you hit a Large or\
smaller creature with that cantrip, you can push the\
creature up to 10 feet straight away from you.\
Repeatable. You can gain this invocation more\
than once. Each time you do so, choose a different\
eligible cantrip.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_repelling_blast ` |
| ` implementation ` | ` dnd2024:rule_warlock_repelling_blast ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` repeatable ` | ` true ` |
| ` repeat_choice_must_differ ` | ` true ` |

#### Prerequisite

<!-- splat:values path=splat_rule/parameters/prerequisite -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 2 ` |
| ` known_warlock_cantrip_deals_damage_via_attack_roll ` | ` true ` |

#### Cantrip choice

<!-- splat:values path=splat_rule/parameters/cantrip_choice -->

| Field | Value |
| --- | --- |
| ` known ` | ` true ` |
| ` class ` | ` warlock ` |
| ` requires_attack_roll ` | ` true ` |

#### Hit effect

<!-- splat:yaml path=splat_rule/parameters/hit_effect -->

```yaml
target_maximum_size: large
optional_push:
  feet_maximum: 10
  direction: straight_away_from_caster
```
