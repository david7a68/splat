---
splat_schema: 1
splat_id: d10b400e-bc61-5fe1-99a4-c864afac3c95
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_cobbler_s_tools_5_gp
splat_name: Cobbler’s Tools (5 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Cobbler’s Tools (5 GP)

## Source text

<!-- SRD page 93 -->

Ability: Dexterity Weight: 5 lb.\
Utilize: Modify footwear to give Advantage on the wearer’s next Dexterity (Acrobatics) check (DC 10)\
Craft: Climber’s Kit

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
| ` value ` | ` 5 ` |
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
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_cobbler_s_tools_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_cobbler_s_tools_5_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: modify_footwear
  check:
    ability: dexterity
    dc: 10
  effect:
    roll: ability_check
    mode: advantage
    ability: dexterity
    skill: acrobatics
    target: wearer
    uses: 1
craftable:
- key: dnd2024:equipment/climber_s_kit_25_gp
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
