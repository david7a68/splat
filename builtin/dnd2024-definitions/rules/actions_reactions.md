---
splat_schema: 1
splat_id: cb4d373c-9890-5a96-ab17-86bf65a3f03b
splat_kind: rule
splat_key: dnd2024:rules/actions_reactions
splat_name: Reactions
splat_origin:
  key: dnd2024:reference/pages/010
splat_sources:
- title: SRD 5.2.1
  locator: pp. 10
  url: https://www.dndbeyond.com/srd
---

# Reactions

## Source text

<!-- SRD page 10 -->

Certain special abilities, spells, and situations al-\
low you to take a special action called a Reaction. A\
Reaction is an instant response to a trigger of some\
kind, which can occur on your turn or on someone\
else’s. The Opportunity Attack, described later in\
“Playing the Game,” is the most common type of\
Reaction.\
When you take a Reaction, you can’t take another\
one until the start of your next turn. If the reaction\
interrupts another creature’s turn, that creature\
can continue its turn right after the Reaction.\
In terms of timing, a Reaction takes place immedi-\
ately after its trigger unless the Reaction’s descrip-\
tion says otherwise.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/actions_reactions ` |
| ` implementation ` | ` dnd2024:rules_actions_reactions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_grant_and_trigger ` | ` true ` |
| ` allowed_turn ` | ` any ` |
| ` default_timing ` | ` immediately_after_trigger ` |
| ` specific_timing_overrides_default ` | ` true ` |
| ` interrupted_turn ` | ` resume_after_reaction ` |

#### Uses

<!-- splat:values path=splat_rule/parameters/uses -->

| Field | Value |
| --- | --- |
| ` max ` | ` 1 ` |
| ` reset ` | ` subject_turn_start ` |
