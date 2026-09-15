---
splat_schema: 1
splat_id: 9f2b6828-f8ba-5d78-a57c-4ca5b1bbe0a9
splat_kind: item
splat_key: dnd2024:equipment/other_tools_forgery_kit_15_gp
splat_name: Forgery Kit (15 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Forgery Kit (15 GP)

## Source text

<!-- SRD page 94 -->

Ability:  Dexterity  Weight: 5 lb.\
Utilize: Mimic 10 or fewer words of someone else’s\
handwriting (DC 15), or duplicate a wax seal (DC 20)\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 15 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_forgery_kit_15_gp ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_forgery_kit_15_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: mimic_handwriting
  check:
    ability: dexterity
    dc: 15
  maximum_words: 10
- action: utilize
  operation: duplicate_wax_seal
  check:
    ability: dexterity
    dc: 20
craftable: []
monster_proficient_if_tool_in_stat_block: true
proficiency_rule:
  key: dnd2024:rules/tools_tool_proficiency
crafting_rule:
  key: dnd2024:rules/crafting_nonmagical_items_tools
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` lb ` |
