---
splat_schema: 2
splat_id: 73bffff6-cb8d-5944-afd7-117612ef8d4b
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_carpenter_s_tools_8_gp
splat_name: Carpenter’s Tools (8 GP)
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Carpenter’s Tools (8 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 93 -->

Ability: Strength Weight: 6 lb.\
Utilize: Seal or pry open a door or container (DC 20)\
Craft: Club, Greatclub, Quarterstaff, Barrel, Chest, Ladder, Pole, Portable Ram, Torch









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_carpenter_s_tools_8_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_carpenter_s_tools_8_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: strength
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: seal_door_or_container
  check:
    ability: strength
    dc: 20
- action: utilize
  operation: pry_open_door_or_container
  check:
    ability: strength
    dc: 20
craftable:
- key: dnd2024:equipment/club
- key: dnd2024:equipment/greatclub
- key: dnd2024:equipment/quarterstaff
- key: dnd2024:equipment/barrel_2_gp
- key: dnd2024:equipment/chest_5_gp
- key: dnd2024:equipment/ladder_1_sp
- key: dnd2024:equipment/pole_5_cp
- key: dnd2024:equipment/ram_portable_4_gp
- key: dnd2024:equipment/torch_1_cp
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
| ` value ` | ` 6 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 8
> mass:
>   unit: lb
>   value: 6
> ```

