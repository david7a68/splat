---
splat_schema: 2
splat_id: 06649811-baac-5864-9a11-e2a7d49c340c
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp
splat_name: Brewer’s Supplies (20 GP)
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Brewer’s Supplies (20 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 93 -->

Ability: Intelligence Weight: 9 lb.\
Utilize: Detect poisoned drink (DC 15), or identify alcohol (DC 10)\
Craft: Antitoxin









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: detect_poisoned_drink
  check:
    ability: intelligence
    dc: 15
- action: utilize
  operation: identify_alcohol
  check:
    ability: intelligence
    dc: 10
craftable:
- key: dnd2024:equipment/antitoxin_50_gp
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
| ` value ` | ` 9 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 20
> mass:
>   unit: lb
>   value: 9
> ```

