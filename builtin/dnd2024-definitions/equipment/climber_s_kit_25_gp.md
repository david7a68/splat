---
splat_schema: 2
splat_id: 0096b952-f968-54f0-966c-338f69a01e15
splat_kind: item
splat_key: dnd2024:equipment/climber_s_kit_25_gp
splat_name: Climber’s Kit (25 GP)
splat_origin: dnd2024:reference/pages/097
splat_sources:
- SRD 5.2.1, pp. 97, https://www.dndbeyond.com/srd
---

# Climber’s Kit (25 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 97 -->

A Climber’s Kit includes boot tips, gloves, pitons, and a harness. As a Utilize action, you can use the Climber’s Kit to anchor yourself; when you do, you can’t fall more than 25 feet from the anchor point, and you can’t move more than 25 feet from there without undoing the anchor as a Bonus Action.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/climber_s_kit_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_climber_s_kit_25_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` components ` | ` ["boot_tips", "gloves", "pitons", "harness"] ` |

#### Anchor

<!-- splat:values path=splat_rule/parameters/anchor -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` target ` | ` self ` |
| ` maximum_fall_ft ` | ` 25 ` |
| ` maximum_distance_from_anchor_ft ` | ` 25 ` |

#### Unanchor

<!-- splat:values path=splat_rule/parameters/unanchor -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 12.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 25
> mass:
>   unit: lb
>   value: 12.0
> ```

