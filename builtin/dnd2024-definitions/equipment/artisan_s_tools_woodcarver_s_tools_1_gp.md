---
splat_schema: 2
splat_id: c924f49f-7e03-52f9-bd4a-4e08a5e8cc0f
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_woodcarver_s_tools_1_gp
splat_name: Woodcarver’s Tools (1 GP)
splat_origin: dnd2024:reference/pages/094
splat_sources:
- SRD 5.2.1, pp. 94, https://www.dndbeyond.com/srd
---

# Woodcarver’s Tools (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 94 -->

Ability: Dexterity Weight: 5 lb.\
Utilize: Carve a pattern in wood (DC 10)\
Craft: Club, Greatclub, Quarterstaff, Ranged weapons (except Pistol, Musket, and Sling), Arcane Focus, Arrows, Bolts, Druidic Focus, Ink Pen, Needles









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_woodcarver_s_tools_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_woodcarver_s_tools_1_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: carve_wood_pattern
  check:
    ability: dexterity
    dc: 10
craftable:
- key: dnd2024:equipment/club
- key: dnd2024:equipment/greatclub
- key: dnd2024:equipment/quarterstaff
- key: dnd2024:equipment/arcane_focus_varies
- key: dnd2024:equipment/druidic_focus_varies
- key: dnd2024:equipment/ink_pen_2_cp
- key: dnd2024:equipment/dart
- key: dnd2024:equipment/light_crossbow
- key: dnd2024:equipment/shortbow
- key: dnd2024:equipment/blowgun
- key: dnd2024:equipment/hand_crossbow
- key: dnd2024:equipment/heavy_crossbow
- key: dnd2024:equipment/longbow
- key: dnd2024:equipment/ammunition_varies
  variant: arrows
- key: dnd2024:equipment/ammunition_varies
  variant: bolts
- key: dnd2024:equipment/ammunition_varies
  variant: needles
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 1
> mass:
>   unit: lb
>   value: 5
> ```

