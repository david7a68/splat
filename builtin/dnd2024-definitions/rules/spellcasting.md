---
splat_schema: 1
splat_id: 4bb066b4-2070-57bd-abe5-b452534cad3b
splat_kind: rule
splat_key: dnd2024:rules/spellcasting
splat_name: Spellcasting
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
---

# Spellcasting

## Source text

<!-- SRD page 102 -->

Most settlements contain individuals who are will-\
ing to cast spells in exchange for payment. If a spell\
has expensive components, add the cost of those\
components to the cost listed in the Spellcasting\
Services table. The higher the level of a desired\
spell, the harder it is to find someone to cast it.\
Spellcasting Services\
Spell Level Availability Cost\
Cantrip Village, town, or city 30 GP\
1 Village, town, or city 50 GP\
2 Village, town, or city 200 GP\
3 Town or city only 300 GP\
4–5 Town or city only 2,000 GP\
6–8 City only 20,000 GP\
9 City only 100,000 GP\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/spellcasting ` |
| ` implementation ` | ` dnd2024:rules_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` component_cost_added ` | ` true ` |

#### Service costs

<!-- splat:rows path=splat_rule/parameters/service_costs -->

| Row | spell_levels | settlements | cost_gp |
| --- | --- | --- | --- |
| ` 0 ` | ` [0, 0] ` | ` ["village", "town", "city"] ` | ` 30 ` |
| ` 1 ` | ` [1, 1] ` | ` ["village", "town", "city"] ` | ` 50 ` |
| ` 2 ` | ` [2, 2] ` | ` ["village", "town", "city"] ` | ` 200 ` |
| ` 3 ` | ` [3, 3] ` | ` ["town", "city"] ` | ` 300 ` |
| ` 4 ` | ` [4, 5] ` | ` ["town", "city"] ` | ` 2000 ` |
| ` 5 ` | ` [6, 8] ` | ` ["city"] ` | ` 20000 ` |
| ` 6 ` | ` [9, 9] ` | ` ["city"] ` | ` 100000 ` |
