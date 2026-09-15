---
splat_schema: 1
splat_id: b8714b01-d122-52df-b652-5256384228c8
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_smith_s_tools_20_gp
splat_name: Smith’s Tools (20 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93, 94
  url: https://www.dndbeyond.com/srd
---

# Smith’s Tools (20 GP)

## Source text

<!-- SRD page 93 -->

Ability: Strength   Weight: 8 lb.\
Utilize: Pry open a door or container (DC 20)\

<!-- SRD page 94 -->

Craft: Any Melee weapon (except Club, Greatclub,\
Quarterstaff, and Whip), Medium armor (except Hide),\
Heavy armor, Ball Bearings, Bucket, Caltrops, Chain,\
Crowbar, Firearm Bullets, Grappling Hook, Iron Pot,\
Iron Spikes, Sling Bullets\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 20 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 8 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_smith_s_tools_20_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_smith_s_tools_20_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: strength
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: pry_open_door_or_container
  check:
    ability: strength
    dc: 20
craftable:
- key: dnd2024:equipment/ball_bearings_1_gp
- key: dnd2024:equipment/bucket_5_cp
- key: dnd2024:equipment/caltrops_1_gp
- key: dnd2024:equipment/chain_5_gp
- key: dnd2024:equipment/crowbar_2_gp
- key: dnd2024:equipment/grappling_hook_2_gp
- key: dnd2024:equipment/pot_iron_2_gp
- key: dnd2024:equipment/spikes_iron_1_gp
- key: dnd2024:equipment/dagger
- key: dnd2024:equipment/handaxe
- key: dnd2024:equipment/javelin
- key: dnd2024:equipment/light_hammer
- key: dnd2024:equipment/mace
- key: dnd2024:equipment/sickle
- key: dnd2024:equipment/spear
- key: dnd2024:equipment/battleaxe
- key: dnd2024:equipment/flail
- key: dnd2024:equipment/glaive
- key: dnd2024:equipment/greataxe
- key: dnd2024:equipment/greatsword
- key: dnd2024:equipment/halberd
- key: dnd2024:equipment/lance
- key: dnd2024:equipment/longsword
- key: dnd2024:equipment/maul
- key: dnd2024:equipment/morningstar
- key: dnd2024:equipment/pike
- key: dnd2024:equipment/rapier
- key: dnd2024:equipment/scimitar
- key: dnd2024:equipment/shortsword
- key: dnd2024:equipment/trident
- key: dnd2024:equipment/warhammer
- key: dnd2024:equipment/war_pick
- key: dnd2024:equipment/chain_shirt
- key: dnd2024:equipment/scale_mail
- key: dnd2024:equipment/breastplate
- key: dnd2024:equipment/half_plate_armor
- key: dnd2024:equipment/ring_mail
- key: dnd2024:equipment/chain_mail
- key: dnd2024:equipment/splint_armor
- key: dnd2024:equipment/plate_armor
- key: dnd2024:equipment/ammunition_varies
  variant: firearm_bullets
- key: dnd2024:equipment/ammunition_varies
  variant: sling_bullets
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
