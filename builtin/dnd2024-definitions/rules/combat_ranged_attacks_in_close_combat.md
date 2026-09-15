---
splat_schema: 1
splat_id: 9c568993-cdec-5914-91ff-fc45f1dca1e9
splat_kind: rule
splat_key: dnd2024:rules/combat_ranged_attacks_in_close_combat
splat_name: Ranged Attacks in Close Combat
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
---

# Ranged Attacks in Close Combat

## Source text

<!-- SRD page 15 -->

Aiming a ranged attack is more difficult when a foe\
is next to you. When you make a ranged attack roll\
with a weapon, a spell, or some other means, you\
have Disadvantage on the roll if you are within 5\
feet of an enemy who can see you and doesn’t have\
the Incapacitated condition (see “Rules Glossary”).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_ranged_attacks_in_close_combat ` |
| ` implementation ` | ` dnd2024:rule_combat_ranged_attacks_in_close_combat ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_kind ` | ` ranged ` |
| ` attack_sources ` | ` ["weapon", "spell", "other"] ` |

#### Disadvantage when

<!-- splat:values path=splat_rule/parameters/disadvantage_when -->

| Field | Value |
| --- | --- |
| ` enemy_distance_feet_maximum ` | ` 5 ` |
| ` enemy_can_see_attacker ` | ` true ` |
| ` enemy_incapacitated ` | ` false ` |
