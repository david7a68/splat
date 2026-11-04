---
splat_schema: 2
splat_id: 36a3a2ae-343a-5897-8b0b-71147020d6aa
splat_kind: rule
splat_key: dnd2024:rules/weapons_sap
splat_name: Sap
splat_origin: dnd2024:reference/pages/090
splat_sources:
- SRD 5.2.1, pp. 90, https://www.dndbeyond.com/srd
---

# Sap

## Source text

<!-- SRD page 90 -->

If you hit a creature with this weapon, that creature has Disadvantage on its next attack roll before the start of your next turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_sap ` |
| ` implementation ` | ` dnd2024:rule_weapons_sap ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` weapon_hits_creature ` |

#### Effect

<!-- splat:values path=splat_rule/parameters/effect -->

| Field | Value |
| --- | --- |
| ` next_attack_roll ` | ` disadvantage ` |
| ` expires ` | ` start_of_attacker_next_turn ` |
