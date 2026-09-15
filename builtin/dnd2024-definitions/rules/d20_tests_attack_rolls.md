---
splat_schema: 1
splat_id: 95464330-7ecb-512a-a334-c52988da0c7d
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_attack_rolls
splat_name: Attack Rolls
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Attack Rolls

## Source text

<!-- SRD page 7 -->

An attack roll determines whether an attack hits\
a target. An attack roll hits if the roll equals or\
exceeds the target’s Armor Class. Attack rolls usu-\
ally occur in battle, described in “Combat” later in\
“Playing the Game,” but the GM might also ask for\
an attack roll in other situations, such as an archery\
competition.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_attack_rolls ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_attack_rolls ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` combat_required ` | ` false ` |

#### Hit

<!-- splat:values path=splat_rule/parameters/hit -->

| Field | Value |
| --- | --- |
| ` op ` | ` gte ` |
| ` left ` | ` attack_roll_total ` |
| ` right ` | ` target_armor_class ` |
