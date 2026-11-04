---
splat_schema: 2
splat_id: 4bb066b4-2070-57bd-abe5-b452534cad3b
splat_kind: rule
splat_key: dnd2024:rules/spellcasting
splat_name: Spellcasting
splat_origin: dnd2024:reference/pages/102
splat_sources:
- SRD 5.2.1, pp. 102, https://www.dndbeyond.com/srd
---

# Spellcasting

## Source text

<!-- SRD page 102 -->

Most settlements contain individuals who are willing to cast spells in exchange for payment. If a spell has expensive components, add the cost of those components to the cost listed in the Spellcasting Services table. The higher the level of a desired spell, the harder it is to find someone to cast it.

**Spellcasting Services**

| Spell Level | Availability | Cost |
| --- | --- | --- |
| Cantrip | Village, town, or city | 30 GP |
| 1 | Village, town, or city | 50 GP |
| 2 | Village, town, or city | 200 GP |
| 3 | Town or city only | 300 GP |
| 4–5 | Town or city only | 2,000 GP |
| 6–8 | City only | 20,000 GP |
| 9 | City only | 100,000 GP |

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

<!-- splat:table
path: splat_rule/parameters/service_costs
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
columns:
  Key:
    type: key
  Spell levels:
    path: spell_levels
    type: yaml
  Settlements:
    path: settlements
    type: yaml
  Cost gp:
    path: cost_gp
    type: integer
-->

| Key | Spell levels | Settlements | Cost gp |
| --- | --- | --- | --- |
| ` row_1 ` | ` [0, 0] ` | ` ["village", "town", "city"] ` | ` 30 ` |
| ` row_2 ` | ` [1, 1] ` | ` ["village", "town", "city"] ` | ` 50 ` |
| ` row_3 ` | ` [2, 2] ` | ` ["village", "town", "city"] ` | ` 200 ` |
| ` row_4 ` | ` [3, 3] ` | ` ["town", "city"] ` | ` 300 ` |
| ` row_5 ` | ` [4, 5] ` | ` ["town", "city"] ` | ` 2000 ` |
| ` row_6 ` | ` [6, 8] ` | ` ["city"] ` | ` 20000 ` |
| ` row_7 ` | ` [9, 9] ` | ` ["city"] ` | ` 100000 ` |
