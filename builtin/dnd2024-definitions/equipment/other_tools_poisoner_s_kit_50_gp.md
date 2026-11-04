---
splat_schema: 2
splat_id: 2077d049-21be-5fc9-9fe8-c1f7788dc692
splat_kind: item
splat_key: dnd2024:equipment/other_tools_poisoner_s_kit_50_gp
splat_name: Poisoner’s Kit (50 GP)
splat_origin: dnd2024:reference/pages/094
splat_sources:
- SRD 5.2.1, pp. 94, https://www.dndbeyond.com/srd
---

# Poisoner’s Kit (50 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 94 -->

Ability: Intelligence Weight: 2 lb.\
Utilize: Detect a poisoned object (DC 10)\
Craft: Basic Poison









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_poisoner_s_kit_50_gp ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_poisoner_s_kit_50_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: detect_poisoned_object
  check:
    ability: intelligence
    dc: 10
craftable:
- key: dnd2024:equipment/poison_basic_100_gp
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
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 50
> mass:
>   unit: lb
>   value: 2
> ```

