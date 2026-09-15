---
splat_schema: 1
splat_id: 8454f260-b423-5188-80dc-5bf56ebb1537
splat_kind: rule
splat_key: dnd2024:glossary/reaction
splat_name: Reaction
splat_origin:
  key: dnd2024:reference/glossary/reaction
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
---

# Reaction

## Source text

<!-- SRD page 186 -->

A Reaction is a special action taken in response to\
a trigger defined in the Reaction’s description. You\
can take a Reaction on another creature’s turn, and\
if you take it on your turn, you can do so even if you\
also take an action, a Bonus Action, or both. Once\
you take a Reaction, you can’t take another one\
until the start of your next turn. The Opportunity\
Attack is a Reaction available to all creatures. See\
also “Opportunity Attacks” and “Playing the Game”\
(“Actions”).\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/reaction ` |
| ` implementation ` | ` dnd2024:glossary_reaction ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` reaction_description_defined ` |
| ` own_or_other_turn ` | ` true ` |
| ` own_turn_compatible_with ` | ` ["action", "bonus_action", "both"] ` |

#### Budget

<!-- splat:values path=splat_rule/parameters/budget -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` reset ` | ` own_next_turn_start ` |

#### Available to all

<!-- splat:values path=splat_rule/parameters/available_to_all -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/opportunity_attacks ` |
