---
splat_schema: 1
splat_id: 35ea33e2-3a90-5e75-bedb-69b616681d60
splat_kind: rule
splat_key: dnd2024:rules/warlock_thirsting_blade
splat_name: Thirsting Blade
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Thirsting Blade

## Source text

<!-- SRD page 74 -->

Prerequisite: Level 5+ Warlock, Pact of the Blade\
Invocation\
You gain the Extra Attack feature for your pact\
weapon only. With that feature, you can attack\
twice with the weapon instead of once when you\
take the Attack action on your turn.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_thirsting_blade ` |
| ` implementation ` | ` dnd2024:rules_warlock_thirsting_blade ` |

### Parameters

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 5 ` |
| ` invocation ` | ` pact_of_the_blade ` |

#### Extra attack

<!-- splat:values path=splat_rule/parameters/extra_attack -->

| Field | Value |
| --- | --- |
| ` weapon ` | ` pact_weapon ` |
| ` attack_count ` | ` 2 ` |
| ` trigger ` | ` actor.attack_action ` |
| ` own_turn_only ` | ` true ` |
