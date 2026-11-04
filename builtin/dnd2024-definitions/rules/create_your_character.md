---
splat_schema: 2
splat_id: 9b5a401d-e630-5918-81ee-c6cc7d99328d
splat_kind: rule
splat_key: dnd2024:rules/create_your_character
splat_name: Create Your Character
splat_origin: dnd2024:reference/pages/019
splat_sources:
- SRD 5.2.1, pp. 19, https://www.dndbeyond.com/srd
---

# Create Your Character

## Source text

<!-- SRD page 19 -->

Here are the steps to make a character; each step is explored in detail:

1:  Choose a Class. Every adventurer is a member of a class. A class broadly describes a character’s vocation, special talents, and favored tactics.

2:  Determine  Origin. A character’s origin includes two elements: background and species. How did the character spend the years leading up to a life of adventure? Who are the character’s ancestors? You can also choose your character’s languages.

3: Determine Ability Scores. Much of what your character does in the game depends on the character’s six abilities.

4:  Choose an Alignment. Alignment is a shorthand for your character’s moral compass.

5: Fill in Details. Using the choices you’ve made, fill in the remaining details on your character sheet.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character ` |
| ` implementation ` | ` dnd2024:rules_create_your_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sequence ` | ` ["choose_class", "choose_background", "choose_species", "choose_languages", "assign_six_ability_scores", "choose_alignment", "fill_character_details"] ` |
| ` class_count_at_creation ` | ` 1 ` |
| ` origin_components ` | ` ["background", "species"] ` |
