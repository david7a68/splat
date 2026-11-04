---
splat_schema: 2
splat_id: d10b400e-bc61-5fe1-99a4-c864afac3c95
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_cobbler_s_tools_5_gp
splat_name: Cobbler’s Tools (5 GP)
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Cobbler’s Tools (5 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 93 -->

Ability: Dexterity Weight: 5 lb.\
Utilize: Modify footwear to give Advantage on the wearer’s next Dexterity (Acrobatics) check (DC 10)\
Craft: Climber’s Kit









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

