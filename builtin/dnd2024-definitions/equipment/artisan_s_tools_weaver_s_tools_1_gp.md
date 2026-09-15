---
splat_schema: 1
splat_id: e1d15a2f-7172-557d-b647-188b224a322e
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_weaver_s_tools_1_gp
splat_name: Weaver’s Tools (1 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Weaver’s Tools (1 GP)

## Source text

<!-- SRD page 94 -->

Ability:  Dexterity  Weight: 5 lb.\
Utilize: Mend a tear in clothing (DC 10), or sew a Tiny\
design (DC 10)\
Craft: Padded Armor, Basket, Bedroll, Blanket, Fine\
Clothes, Net, Robe, Rope, Sack, String, Tent, Traveler’s\
Clothes\

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
| ` value ` | ` 1 ` |
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
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_weaver_s_tools_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_weaver_s_tools_1_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: mend_clothing_tear
  check:
    ability: dexterity
    dc: 10
- action: utilize
  operation: sew_design
  check:
    ability: dexterity
    dc: 10
  size: tiny
craftable:
- key: dnd2024:equipment/padded_armor
- key: dnd2024:equipment/basket_4_sp
- key: dnd2024:equipment/bedroll_1_gp
- key: dnd2024:equipment/blanket_5_sp
- key: dnd2024:equipment/clothes_fine_15_gp
- key: dnd2024:equipment/net_1_gp
- key: dnd2024:equipment/robe_1_gp
- key: dnd2024:equipment/rope_1_gp
- key: dnd2024:equipment/sack_1_cp
- key: dnd2024:equipment/string_1_sp
- key: dnd2024:equipment/tent_2_gp
- key: dnd2024:equipment/clothes_traveler_s_2_gp
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
