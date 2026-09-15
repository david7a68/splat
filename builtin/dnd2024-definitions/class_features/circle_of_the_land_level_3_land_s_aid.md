---
splat_schema: 1
splat_id: 4ea24045-6ee6-51ce-9c40-3968055ba7fc
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_3_land_s_aid
splat_name: 'Level 3: Land’s Aid'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
---

# Level 3: Land’s Aid

## Source text

<!-- SRD page 46 -->

As a Magic action, you can expend a use of your Wild\
Shape and choose a point within 60 feet of yourself.\
Vitality-giving flowers and life-draining thorns\
appear for a moment in a 10-foot-radius Sphere cen-\
tered on that point. Each creature of your choice in\
the Sphere must make a Constitution saving throw\
against your spell save DC, taking 2d6 Necrotic\
damage on a failed save or half as much damage on\
a successful one. One creature of your choice in that\
area regains 2d6 Hit Points.\
The damage and healing increase by 1d6 when\
you reach Druid levels 10 (3d6) and 14 (4d6).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/circle_of_the_land_level_3_land_s_aid ` |
| ` implementation ` | ` dnd2024:class_features_circle_of_the_land_level_3_land_s_aid ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` wild_shape ` |
| ` amount ` | ` 1 ` |

#### Origin

<!-- splat:values path=splat_rule/parameters/origin -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 60 ` |
| ` type ` | ` point ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 10 ` |

#### Damage targets

<!-- splat:values path=splat_rule/parameters/damage_targets -->

| Field | Value |
| --- | --- |
| ` creatures ` | ` chosen_in_area ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  stat: spell_save_dc
success_damage_multiplier: 0.5
```

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
type: necrotic
dice_by_druid_level:
  '3':
    dice:
      count: 2
      sides: 6
  '10':
    dice:
      count: 3
      sides: 6
  '14':
    dice:
      count: 4
      sides: 6
```

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
target_count: 1
target_choice: creature_in_area
dice_by_druid_level:
  '3':
    dice:
      count: 2
      sides: 6
  '10':
    dice:
      count: 3
      sides: 6
  '14':
    dice:
      count: 4
      sides: 6
```
