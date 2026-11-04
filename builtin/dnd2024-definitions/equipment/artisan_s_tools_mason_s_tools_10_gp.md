---
splat_schema: 2
splat_id: 5086c6de-d84a-5b21-a5e5-cfad2cedeeae
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_mason_s_tools_10_gp
splat_name: Mason’s Tools (10 GP)
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Mason’s Tools (10 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 93 -->

Ability: Strength Weight: 8 lb.\
Utilize: Chisel a symbol or hole in stone (DC 10)\
Craft: Block and Tackle









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_mason_s_tools_10_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_mason_s_tools_10_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: strength
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: chisel_symbol_in_stone
  check:
    ability: strength
    dc: 10
- action: utilize
  operation: chisel_hole_in_stone
  check:
    ability: strength
    dc: 10
craftable:
- key: dnd2024:equipment/block_and_tackle_1_gp
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
>   value: 10
> mass:
>   unit: lb
>   value: 8
> ```

