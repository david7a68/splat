---
splat_schema: 2
splat_id: 146adcd4-4b17-5d9c-99cd-e91d38094b0b
splat_kind: rule
splat_key: dnd2024:glossary/hit_point_dice
splat_name: Hit Point Dice
splat_origin: dnd2024:reference/glossary/hit_point_dice
splat_sources:
- SRD 5.2.1, pp. 183, https://www.dndbeyond.com/srd
---

# Hit Point Dice

## Source text

<!-- SRD page 183 -->

Hit Point Dice, or Hit Dice for short, help determine a player character’s Hit Point maximum, as explained in “Character Creation.” Most monsters also have Hit Dice. A creature can spend Hit Dice during a Short Rest to regain Hit Points. See also “Short Rest.”

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/hit_point_dice ` |
| ` implementation ` | ` dnd2024:glossary_hit_point_dice ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` player_character_hp_maximum_uses_hit_dice ` | ` true ` |
| ` short_rest_spending_allowed ` | ` true ` |

#### Spending rule

<!-- splat:values path=splat_rule/parameters/spending_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/short_rest ` |
