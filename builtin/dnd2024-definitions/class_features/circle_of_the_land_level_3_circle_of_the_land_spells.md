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

##### Arid

<!-- splat:table
path: splat_rule/parameters/always_prepared/arid
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum druid level:
    path: minimum_druid_level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Minimum druid level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` | [Rules](#^splat-1fe33818-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | [Rules](#^splat-1fe33818-row_2 "section:row_2") |
| ` row_3 ` | ` 7 ` | [Rules](#^splat-1fe33818-row_3 "section:row_3") |
| ` row_4 ` | ` 9 ` | [Rules](#^splat-1fe33818-row_4 "section:row_4") |

###### Row 1

^splat-1fe33818-row_1

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/arid/@row_1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/blur ` |
| ` 1 ` | ` dnd2024:spells/burning_hands ` |
| ` 2 ` | ` dnd2024:spells/fire_bolt ` |

###### Row 2

^splat-1fe33818-row_2

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/arid/@row_2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fireball ` |

###### Row 3

^splat-1fe33818-row_3

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/arid/@row_3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/blight ` |

###### Row 4

^splat-1fe33818-row_4

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/arid/@row_4/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/wall_of_stone ` |

##### Polar

<!-- splat:table
path: splat_rule/parameters/always_prepared/polar
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum druid level:
    path: minimum_druid_level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Minimum druid level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` | [Rules](#^splat-2cc156f3-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | [Rules](#^splat-2cc156f3-row_2 "section:row_2") |
| ` row_3 ` | ` 7 ` | [Rules](#^splat-2cc156f3-row_3 "section:row_3") |
| ` row_4 ` | ` 9 ` | [Rules](#^splat-2cc156f3-row_4 "section:row_4") |

###### Row 1

^splat-2cc156f3-row_1

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/polar/@row_1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fog_cloud ` |
| ` 1 ` | ` dnd2024:spells/hold_person ` |
| ` 2 ` | ` dnd2024:spells/ray_of_frost ` |

###### Row 2

^splat-2cc156f3-row_2

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/polar/@row_2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/sleet_storm ` |

###### Row 3

^splat-2cc156f3-row_3

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/polar/@row_3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/ice_storm ` |

###### Row 4

^splat-2cc156f3-row_4

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/polar/@row_4/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/cone_of_cold ` |

##### Temperate

<!-- splat:table
path: splat_rule/parameters/always_prepared/temperate
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum druid level:
    path: minimum_druid_level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Minimum druid level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` | [Rules](#^splat-7740fe79-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | [Rules](#^splat-7740fe79-row_2 "section:row_2") |
| ` row_3 ` | ` 7 ` | [Rules](#^splat-7740fe79-row_3 "section:row_3") |
| ` row_4 ` | ` 9 ` | [Rules](#^splat-7740fe79-row_4 "section:row_4") |

###### Row 1

^splat-7740fe79-row_1

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/temperate/@row_1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/misty_step ` |
| ` 1 ` | ` dnd2024:spells/shocking_grasp ` |
| ` 2 ` | ` dnd2024:spells/sleep ` |

###### Row 2

^splat-7740fe79-row_2

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/temperate/@row_2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/lightning_bolt ` |

###### Row 3

^splat-7740fe79-row_3

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/temperate/@row_3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/freedom_of_movement ` |

###### Row 4

^splat-7740fe79-row_4

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/temperate/@row_4/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/tree_stride ` |

##### Tropical

<!-- splat:table
path: splat_rule/parameters/always_prepared/tropical
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum druid level:
    path: minimum_druid_level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Minimum druid level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` | [Rules](#^splat-c378aa69-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | [Rules](#^splat-c378aa69-row_2 "section:row_2") |
| ` row_3 ` | ` 7 ` | [Rules](#^splat-c378aa69-row_3 "section:row_3") |
| ` row_4 ` | ` 9 ` | [Rules](#^splat-c378aa69-row_4 "section:row_4") |

###### Row 1

^splat-c378aa69-row_1

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/tropical/@row_1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/acid_splash ` |
| ` 1 ` | ` dnd2024:spells/ray_of_sickness ` |
| ` 2 ` | ` dnd2024:spells/web ` |

###### Row 2

^splat-c378aa69-row_2

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/tropical/@row_2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/stinking_cloud ` |

###### Row 3

^splat-c378aa69-row_3

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/tropical/@row_3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/polymorph ` |

###### Row 4

^splat-c378aa69-row_4

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/tropical/@row_4/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/insect_plague ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 3 ` |
