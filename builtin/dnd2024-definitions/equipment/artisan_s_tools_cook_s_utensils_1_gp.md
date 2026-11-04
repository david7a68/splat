---
splat_schema: 2
splat_id: ec93020c-d470-5c52-8666-c2fb31533727
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_cook_s_utensils_1_gp
splat_name: Cook’s Utensils (1 GP)
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Cook’s Utensils (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 93 -->

Ability: Wisdom Weight: 8 lb.\
Utilize: Improve food’s flavor (DC 10), or detect spoiled or poisoned food (DC 15)\
Craft: Rations









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_cook_s_utensils_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_cook_s_utensils_1_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: wisdom
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: improve_food_flavor
  check:
    ability: wisdom
    dc: 10
- action: utilize
  operation: detect_spoiled_or_poisoned_food
  check:
    ability: wisdom
    dc: 15
craftable:
- key: dnd2024:equipment/rations_5_sp
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
| ` value ` | ` 8 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 1
> mass:
>   unit: lb
>   value: 8
> ```

