---
splat_schema: 2
splat_id: 3d1a30f2-af5e-562a-9759-c5b10bf4d467
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_initiative
splat_name: Initiative
splat_origin: dnd2024:reference/pages/254
splat_sources:
- SRD 5.2.1, pp. 254, https://www.dndbeyond.com/srd
---

# Initiative

## Source text

<!-- SRD page 254 -->

The Initiative entry specifies the monster’s Initiative modifier followed by the monster’s Initiative score in parentheses. Use the modifier when you roll to determine a monster’s Initiative. A monster’s Initiative modifier is typically equal to its Dexterity modifier, but some monsters have additional modifiers, such as Proficiency Bonus, applied to that number.

If you don’t want to roll a monster’s Initiative, use the Initiative score as the monster’s Initiative in combat. Initiative is further detailed in “Playing the Game.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_initiative ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_initiative ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` resolution_choice ` | ` ["roll_with_modifier", "use_fixed_score"] ` |
| ` default_modifier ` | ` dexterity_modifier ` |
| ` additional_modifiers ` | ` stat_block ` |

#### Initiative entry

<!-- splat:values path=splat_rule/parameters/initiative_entry -->

| Field | Value |
| --- | --- |
| ` first ` | ` modifier ` |
| ` parenthetical ` | ` score ` |
