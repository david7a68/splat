---
splat_schema: 2
splat_id: e085ecb5-245e-57d4-a4bb-67796e1be499
splat_kind: rule
splat_key: dnd2024:glossary/search_action
splat_name: Search [Action]
splat_origin: dnd2024:reference/glossary/search_action
splat_sources:
- SRD 5.2.1, pp. 187, https://www.dndbeyond.com/srd
---

# Search [Action]

## Source text

<!-- SRD page 187 -->

When you take the Search action, you make a Wisdom check to discern something that isn’t obvious. The Search table suggests which skills are applicable when you take this action, depending on what you’re trying to detect.

**Search**

| Skill | Thing to Detect |
| --- | --- |
| Insight | Creature’s state of mind |
| Medicine | Creature’s ailment or cause of death |
| Perception | Concealed creature or object |
| Survival | Tracks or food |

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/search_action ` |
| ` implementation ` | ` dnd2024:glossary_search_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` ability ` | ` wisdom ` |
| ` purpose ` | ` discern_nonobvious_information ` |

#### Suggested skill by target

<!-- splat:values path=splat_rule/parameters/suggested_skill_by_target -->

| Field | Value |
| --- | --- |
| ` creature_state_of_mind ` | ` insight ` |
| ` creature_ailment ` | ` medicine ` |
| ` cause_of_death ` | ` medicine ` |
| ` concealed_creature ` | ` perception ` |
| ` concealed_object ` | ` perception ` |
| ` tracks ` | ` survival ` |
| ` food ` | ` survival ` |
