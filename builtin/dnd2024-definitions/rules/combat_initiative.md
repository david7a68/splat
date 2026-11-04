---
splat_schema: 2
splat_id: cf9dd38b-9f7f-5049-a177-221ade70596b
splat_kind: rule
splat_key: dnd2024:rules/combat_initiative
splat_name: Initiative
splat_origin: dnd2024:reference/pages/013
splat_sources:
- SRD 5.2.1, pp. 13, https://www.dndbeyond.com/srd
---

# Initiative

## Source text

<!-- SRD page 13 -->

Initiative determines the order of turns during combat. When combat starts, every participant rolls Initiative; they make a Dexterity check that determines their place in the Initiative order. The GM rolls for monsters. For a group of identical creatures, the GM makes a single roll, so each member of the group has the same Initiative.

Surprise. If a combatant is surprised by combat starting, that combatant has Disadvantage on their Initiative roll. For example, if an ambusher starts combat while hidden from a foe who is unaware that combat is starting, that foe is surprised.

Initiative Order. A combatant’s check total is called their Initiative count, or Initiative for short. The GM ranks the combatants, from highest to lowest Initiative. This is the order in which they act during each round. The Initiative order remains the same from round to round.

Ties. If a tie occurs, the GM decides the order among tied monsters, and the players decide the order among tied characters. The GM decides the order if the tie is between a monster and a player character.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_initiative ` |
| ` implementation ` | ` dnd2024:rules_combat_initiative ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` identical_monster_group ` | ` one_shared_roll ` |
| ` order ` | ` descending_check_total ` |
| ` order_persists_each_round ` | ` true ` |

#### Check

<!-- splat:values path=splat_rule/parameters/check -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` kind ` | ` ability_check ` |

#### Roll controller

<!-- splat:values path=splat_rule/parameters/roll_controller -->

| Field | Value |
| --- | --- |
| ` player_character ` | ` player ` |
| ` monster ` | ` gm ` |

#### Surprised

<!-- splat:values path=splat_rule/parameters/surprised -->

| Field | Value |
| --- | --- |
| ` initiative_disadvantage ` | ` true ` |

#### Ties

<!-- splat:values path=splat_rule/parameters/ties -->

| Field | Value |
| --- | --- |
| ` monsters_only ` | ` gm_choice ` |
| ` characters_only ` | ` players_choice ` |
| ` mixed ` | ` gm_choice ` |
