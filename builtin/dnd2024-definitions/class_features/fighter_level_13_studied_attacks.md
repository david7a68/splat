---
splat_schema: 1
splat_id: 0cb8fa23-160f-5feb-9b95-50ceb950a9c7
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_13_studied_attacks
splat_name: 'Level 13: Studied Attacks'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
---

# Level 13: Studied Attacks

## Source text

<!-- SRD page 48 -->

You study your opponents and learn from each attack you make. If you make an attack roll against a creature and miss, you have Advantage on your next attack roll against that creature before the end of your next turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_13_studied_attacks ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_13_studied_attacks ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.attack_missed_creature ` |

#### Grant

<!-- splat:values path=splat_rule/parameters/grant -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |
| ` scope ` | ` next_attack_against_same_creature ` |
| ` expires ` | ` actor.next_turn.end ` |
| ` consume_on ` | ` matching_attack_roll ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 13 ` |
