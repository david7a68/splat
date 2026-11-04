---
splat_schema: 2
splat_id: 675c7a41-552f-588c-8c8b-132b716a8f85
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_leatherworker_s_tools_5_gp
splat_name: Leatherworker’s Tools (5 GP)
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Leatherworker’s Tools (5 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 93 -->

Ability: Dexterity Weight: 5 lb.\
Utilize: Add a design to a leather item (DC 10)\
Craft: Sling, Whip, Hide Armor, Leather Armor, Studded Leather Armor, Backpack, Crossbow Bolt Case, Map or Scroll Case, Parchment, Pouch, Quiver, Waterskin









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_leatherworker_s_tools_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_leatherworker_s_tools_5_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: decorate_leather
  check:
    ability: dexterity
    dc: 10
craftable:
- key: dnd2024:equipment/sling
- key: dnd2024:equipment/whip
- key: dnd2024:equipment/hide_armor
- key: dnd2024:equipment/leather_armor
- key: dnd2024:equipment/studded_leather_armor
- key: dnd2024:equipment/backpack_2_gp
- key: dnd2024:equipment/case_crossbow_bolt_1_gp
- key: dnd2024:equipment/case_map_or_scroll_1_gp
- key: dnd2024:equipment/parchment_1_sp
- key: dnd2024:equipment/pouch_5_sp
- key: dnd2024:equipment/quiver_1_gp
- key: dnd2024:equipment/waterskin_2_sp
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
>   value: 5
> mass:
>   unit: lb
>   value: 5
> ```

