---
splat_schema: 2
splat_id: 5ce9cc9b-4f04-5b0e-b199-bd01003a003d
splat_kind: rule
splat_key: dnd2024:rules/warlock_agonizing_blast
splat_name: Agonizing Blast
splat_origin: dnd2024:reference/pages/072
splat_sources:
- SRD 5.2.1, pp. 72, https://www.dndbeyond.com/srd
---

# Agonizing Blast

## Source text

<!-- SRD page 72 -->

Prerequisite: Level 2+ Warlock, a Warlock Cantrip That Deals Damage

Choose one of your known Warlock cantrips that deals damage. You can add your Charisma modifier to that spell’s damage rolls.

Repeatable. You can gain this invocation more than once. Each time you do so, choose a different eligible cantrip.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_agonizing_blast ` |
| ` implementation ` | ` dnd2024:rules_warlock_agonizing_blast ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_roll_bonus ` | ` charisma_modifier ` |
| ` repeatable ` | ` true ` |
| ` repeat_choice_must_differ ` | ` true ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 2 ` |
| ` known_warlock_damaging_cantrip ` | ` true ` |

#### Choice

<!-- splat:values path=splat_rule/parameters/choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` from ` | ` known_warlock_damaging_cantrips ` |
