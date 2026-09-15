---
splat_schema: 1
splat_id: 120168ec-27c4-5105-8786-d038c5fd2e5f
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
splat_name: 'Level 3: Circle of the Land Spells'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
---

# Level 3: Circle of the Land Spells

## Source text

<!-- SRD page 46 -->

Whenever you finish a Long Rest, choose one type\
of land: arid, polar, temperate, or tropical. Consult\
the table below that corresponds to the chosen\
type; you have the spells listed for your Druid level\
and lower prepared.\
Arid Land\
Druid Level Circle Spells\
3 Blur, Burning Hands, Fire Bolt\
5 Fireball\
7 Blight\
9 Wall of Stone\
Polar Land\
Druid Level Circle Spells\
3 Fog Cloud, Hold Person, Ray of Frost\
5 Sleet Storm\
7 Ice Storm\
9 Cone of Cold\
Temperate Land\
Druid Level Circle Spells\
3 Misty Step, Shocking Grasp, Sleep\
5 Lightning Bolt\
7 Freedom of Movement\
9 Tree Stride\
Tropical Land\
Druid Level Circle Spells\
3 Acid Splash, Ray of Sickness, Web\
5 Stinking Cloud\
7 Polymorph\
9 Insect Plague\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells ` |
| ` implementation ` | ` dnd2024:class_features_circle_of_the_land_level_3_circle_of_the_land_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` active_list ` | ` chosen_land ` |
| ` include_lower_levels ` | ` true ` |
| ` counts_against_prepared_limit ` | ` false ` |
| ` spell_class ` | ` druid ` |

#### Land choice

<!-- splat:values path=splat_rule/parameters/land_choice -->

| Field | Value |
| --- | --- |
| ` event ` | ` rest.long_completed ` |
| ` count ` | ` 1 ` |
| ` options ` | ` ["arid", "polar", "temperate", "tropical"] ` |

#### Always prepared

<!-- splat:yaml path=splat_rule/parameters/always_prepared -->

```yaml
arid:
- minimum_druid_level: 3
  spells:
  - key: dnd2024:spells/blur
  - key: dnd2024:spells/burning_hands
  - key: dnd2024:spells/fire_bolt
- minimum_druid_level: 5
  spells:
  - key: dnd2024:spells/fireball
- minimum_druid_level: 7
  spells:
  - key: dnd2024:spells/blight
- minimum_druid_level: 9
  spells:
  - key: dnd2024:spells/wall_of_stone
polar:
- minimum_druid_level: 3
  spells:
  - key: dnd2024:spells/fog_cloud
  - key: dnd2024:spells/hold_person
  - key: dnd2024:spells/ray_of_frost
- minimum_druid_level: 5
  spells:
  - key: dnd2024:spells/sleet_storm
- minimum_druid_level: 7
  spells:
  - key: dnd2024:spells/ice_storm
- minimum_druid_level: 9
  spells:
  - key: dnd2024:spells/cone_of_cold
temperate:
- minimum_druid_level: 3
  spells:
  - key: dnd2024:spells/misty_step
  - key: dnd2024:spells/shocking_grasp
  - key: dnd2024:spells/sleep
- minimum_druid_level: 5
  spells:
  - key: dnd2024:spells/lightning_bolt
- minimum_druid_level: 7
  spells:
  - key: dnd2024:spells/freedom_of_movement
- minimum_druid_level: 9
  spells:
  - key: dnd2024:spells/tree_stride
tropical:
- minimum_druid_level: 3
  spells:
  - key: dnd2024:spells/acid_splash
  - key: dnd2024:spells/ray_of_sickness
  - key: dnd2024:spells/web
- minimum_druid_level: 5
  spells:
  - key: dnd2024:spells/stinking_cloud
- minimum_druid_level: 7
  spells:
  - key: dnd2024:spells/polymorph
- minimum_druid_level: 9
  spells:
  - key: dnd2024:spells/insect_plague
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 3 ` |
