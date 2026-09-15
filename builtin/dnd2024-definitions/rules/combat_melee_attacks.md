---
splat_schema: 1
splat_id: 312d9860-b8b9-5e10-bb12-46abc16bcdc7
splat_kind: rule
splat_key: dnd2024:rules/combat_melee_attacks
splat_name: Melee Attacks
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
---

# Melee Attacks

## Source text

<!-- SRD page 15 -->

A melee attack allows you to attack a target within\
your reach. A melee attack typically uses a hand-\
held weapon or an Unarmed Strike. Many monsters\
make melee attacks with claws, teeth, or other body\
parts. A few spells also involve melee attacks.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_melee_attacks ` |
| ` implementation ` | ` dnd2024:rules_combat_melee_attacks ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target_distance_maximum ` | ` attacker_reach ` |
| ` attack_kind ` | ` melee ` |
