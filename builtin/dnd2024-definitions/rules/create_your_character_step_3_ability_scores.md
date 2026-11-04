---
splat_schema: 2
splat_id: 4e28e57f-bfd5-5688-afe1-58d7a4414d03
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_step_3_ability_scores
splat_name: 'Step 3: Ability Scores'
splat_origin: dnd2024:reference/pages/020
splat_sources:
- SRD 5.2.1, pp. 20, https://www.dndbeyond.com/srd
---

# Step 3: Ability Scores

## Source text

<!-- SRD page 20 -->

To determine your character’s ability scores, you first generate a set of six numbers using the instructions below and then assign them to your six abilities. “Playing the Game” explains what each ability means.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_step_3_ability_scores ` |
| ` implementation ` | ` dnd2024:rule_create_your_character_step_3_ability_scores ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` generate_count ` | ` 6 ` |

#### Assign

<!-- splat:values path=splat_rule/parameters/assign -->

| Field | Value |
| --- | --- |
| ` abilities ` | ` ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"] ` |
| ` each_number_once ` | ` true ` |
