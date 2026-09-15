---
splat_schema: 1
splat_id: 77348a61-de0f-5b72-ae1d-3dd0ebee7b6f
splat_kind: rule
splat_key: dnd2024:glossary/condition
splat_name: Condition
splat_origin:
  key: dnd2024:reference/glossary/condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
---

# Condition

## Source text

<!-- SRD page 179 -->

A condition is a temporary game state. The defini-\
tion of a condition says how it affects its recipient,\
and various rules define how to end a condition.\
This glossary defines these conditions:\
Blinded Grappled Poisoned\
Charmed Incapacitated Prone\
Deafened Invisible Restrained\
Exhaustion Paralyzed Stunned\
Frightened Petrified Unconscious\
A condition doesn’t stack with itself; a recipient\
either has a condition or doesn’t. The Exhaustion\
condition is an exception to that rule.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/condition ` |
| ` implementation ` | ` dnd2024:glossary_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` default_stacking ` | ` false ` |
| ` stacking_exception ` | ` exhaustion ` |
| ` duration_and_end ` | ` source_effect_defined ` |

#### Members

<!-- splat:rows path=splat_rule/parameters/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/blinded_condition ` |
| ` 1 ` | ` dnd2024:glossary/charmed_condition ` |
| ` 2 ` | ` dnd2024:glossary/deafened_condition ` |
| ` 3 ` | ` dnd2024:glossary/exhaustion_condition ` |
| ` 4 ` | ` dnd2024:glossary/frightened_condition ` |
| ` 5 ` | ` dnd2024:glossary/grappled_condition ` |
| ` 6 ` | ` dnd2024:glossary/incapacitated_condition ` |
| ` 7 ` | ` dnd2024:glossary/invisible_condition ` |
| ` 8 ` | ` dnd2024:glossary/paralyzed_condition ` |
| ` 9 ` | ` dnd2024:glossary/petrified_condition ` |
| ` 10 ` | ` dnd2024:glossary/poisoned_condition ` |
| ` 11 ` | ` dnd2024:glossary/prone_condition ` |
| ` 12 ` | ` dnd2024:glossary/restrained_condition ` |
| ` 13 ` | ` dnd2024:glossary/stunned_condition ` |
| ` 14 ` | ` dnd2024:glossary/unconscious_condition ` |
