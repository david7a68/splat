---
splat_schema: 2
splat_id: 7365eef7-5968-589a-9784-55a7acda7ef8
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_tinker_s_tools_50_gp
splat_name: Tinker’s Tools (50 GP)
splat_origin: dnd2024:reference/pages/094
splat_sources:
- SRD 5.2.1, pp. 94, https://www.dndbeyond.com/srd
---

# Tinker’s Tools (50 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 94 -->

Ability: Dexterity Weight: 10 lb.\
Utilize: Assemble a Tiny item composed of scrap, which falls apart in 1 minute (DC 20)\
Craft: Musket, Pistol, Bell, Bullseye Lantern, Flask, Hooded Lantern, Hunting Trap, Lock, Manacles, Mirror, Shovel, Signal Whistle, Tinderbox









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_tinker_s_tools_50_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_tinker_s_tools_50_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: assemble_scrap_item
  check:
    ability: dexterity
    dc: 20
  size: tiny
  expires:
    value: 1
    unit: minute
  expiry: falls_apart
craftable:
- key: dnd2024:equipment/musket
- key: dnd2024:equipment/pistol
- key: dnd2024:equipment/bell_1_gp
- key: dnd2024:equipment/lantern_bullseye_10_gp
- key: dnd2024:equipment/flask_2_cp
- key: dnd2024:equipment/lantern_hooded_5_gp
- key: dnd2024:equipment/hunting_trap_5_gp
- key: dnd2024:equipment/lock_10_gp
- key: dnd2024:equipment/manacles_2_gp
- key: dnd2024:equipment/mirror_5_gp
- key: dnd2024:equipment/shovel_2_gp
- key: dnd2024:equipment/signal_whistle_5_cp
- key: dnd2024:equipment/tinderbox_5_sp
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
| ` value ` | ` 10 ` |
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
>   value: 10
> ```

