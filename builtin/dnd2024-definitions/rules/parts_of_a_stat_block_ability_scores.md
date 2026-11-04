---
splat_schema: 2
splat_id: 5553e4ad-2727-5db7-a798-e8cd1e2acae7
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_ability_scores
splat_name: Ability Scores
splat_origin: dnd2024:reference/pages/255
splat_sources:
- SRD 5.2.1, pp. 255, https://www.dndbeyond.com/srd
---

# Ability Scores

## Source text

<!-- SRD page 255 -->

Every monster has six ability scores along with corresponding ability score modifiers and saving throw modifiers. For more information on ability scores and saving throws, see “Playing the Game.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_ability_scores ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_ability_scores ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` abilities ` | ` ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"] ` |
| ` each_has ` | ` ["score", "ability_modifier", "saving_throw_modifier"] ` |
| ` source ` | ` stat_block ` |
