---
splat_schema: 1
splat_id: 0b6c115a-41ea-526f-85af-8df2e8181de1
splat_kind: rule
splat_key: dnd2024:rules/combat_encounters_many_creatures
splat_name: Many Creatures
splat_origin:
  key: dnd2024:reference/pages/203
splat_sources:
- title: SRD 5.2.1
  locator: pp. 203
  url: https://www.dndbeyond.com/srd
---

# Many Creatures

## Source text

<!-- SRD page 203 -->

The more creatures in an encounter, the higher\
the risk that a lucky streak on their part could deal\
more damage to the characters than you expect. If\
your encounter includes more than two creatures\
per character, include fragile creatures that can be\
defeated quickly. This guideline is especially im-\
portant for characters of level 1 or 2.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_encounters_many_creatures ` |
| ` implementation ` | ` dnd2024:rules_combat_encounters_many_creatures ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` guideline ` | ` true ` |
| ` if_creatures_per_character_greater_than ` | ` 2 ` |
| ` recommendation ` | ` include_quickly_defeated_fragile_creatures ` |
| ` priority_character_levels ` | ` [1, 2] ` |
| ` mandatory_combat_modifier ` | ` false ` |
