---
splat_schema: 2
splat_id: f7aee4ca-8b37-5611-9756-d44db9129b95
splat_kind: rule
splat_key: dnd2024:rules/combat_encounters_step_1_choose_a_difficulty
splat_name: 'Step 1: Choose a Difficulty'
splat_origin: dnd2024:reference/pages/202
splat_sources:
- SRD 5.2.1, pp. 202, https://www.dndbeyond.com/srd
---

# Step 1: Choose a Difficulty

## Source text

<!-- SRD page 202 -->

Three categories describe the range of encounter difficulty:

Low Difficulty. An encounter of low difficulty is likely to have one or two scary moments for the players, but their characters should emerge victorious with no casualties. One or more of them might need to use healing resources, however. As a rough guideline, a single monster generally presents a low-difficulty challenge for a party of four characters whose level equals the monster’s Challenge Rating.

Moderate Difficulty. Absent healing and other resources, an encounter of moderate difficulty could go badly for the adventurers. Weaker characters might get taken out of the fight, and there’s a slim chance that one or more characters might die.

High Difficulty. A high-difficulty encounter could be lethal for one or more characters. To survive it, the characters will need smart tactics, quick thinking, and maybe even a little luck.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_encounters_step_1_choose_a_difficulty ` |
| ` implementation ` | ` dnd2024:rules_combat_encounters_step_1_choose_a_difficulty ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` difficulty_choice ` | ` ["low", "moderate", "high"] ` |

#### Low difficulty guideline

<!-- splat:values path=splat_rule/parameters/low_difficulty_guideline -->

| Field | Value |
| --- | --- |
| ` monster_count ` | ` 1 ` |
| ` party_size ` | ` 4 ` |
| ` monster_cr ` | ` character_level ` |
| ` binding_rule ` | ` false ` |
