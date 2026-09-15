---
splat_schema: 1
splat_id: 4aa5c603-b801-5b3f-af76-41fed93d29d7
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary
splat_name: 'Level 14: Nature’s Sanctuary'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
---

# Level 14: Nature’s Sanctuary

## Source text

<!-- SRD page 46 -->

As a Magic action, you can expend a use of your Wild\
Shape and cause spectral trees and vines to appear\
in a 15-foot Cube on the ground within 120 feet of\
yourself. They last there for 1 minute or until you\
have the Incapacitated condition or die. You and\
your allies have Half Cover while in that area, and\
your allies gain the current Resistance of your Na-\
ture’s Ward while there.\
As a Bonus Action, you can move the Cube up to\
60 feet to ground within 120 feet of yourself.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary ` |
| ` implementation ` | ` dnd2024:class_features_circle_of_the_land_level_14_nature_s_sanctuary ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` ends_on ` | ` ["actor.incapacitated", "actor.died"] ` |

#### Resource cost

<!-- splat:values path=splat_rule/parameters/resource_cost -->

| Field | Value |
| --- | --- |
| ` id ` | ` druid.wild_shape ` |
| ` amount ` | ` 1 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 15 ` |
| ` placement ` | ` ground ` |
| ` range_feet ` | ` 120 ` |
| ` geometry_validation ` | ` explicit_location_choice ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` minutes ` | ` 1 ` |

#### Within area

<!-- splat:yaml path=splat_rule/parameters/within_area -->

```yaml
actor_and_allies:
  cover: half
allies:
  resistance:
    var: actor.natures_ward.current_resistance
```

#### Move area

<!-- splat:yaml path=splat_rule/parameters/move_area -->

```yaml
action: bonus_action
maximum_feet: 60
destination:
  ground: true
  within_actor_feet: 120
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 14 ` |
