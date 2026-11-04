---
splat_schema: 2
splat_id: eca12d19-b2e3-5925-b1d6-0dda3bb92d61
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_glassblower_s_tools_30_gp
splat_name: Glassblower’s Tools (30 GP)
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Glassblower’s Tools (30 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 93 -->

Ability: Intelligence Weight: 5 lb.\
Utilize: Discern what a glass object held in the past 24 hours (DC 15)\
Craft: Glass Bottle, Magnifying Glass, Spyglass, Vial









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_glassblower_s_tools_30_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_glassblower_s_tools_30_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: identify_previous_contents
  check:
    ability: intelligence
    dc: 15
  material: glass
  lookback:
    value: 24
    unit: hour
craftable:
- key: dnd2024:equipment/bottle_glass_2_gp
- key: dnd2024:equipment/magnifying_glass_100_gp
- key: dnd2024:equipment/spyglass_1_000_gp
- key: dnd2024:equipment/vial_1_gp
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
>   value: 30
> mass:
>   unit: lb
>   value: 5
> ```

