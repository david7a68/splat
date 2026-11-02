---
splat_schema: 1
splat_id: 06a61f39-b3d7-50c2-bf50-fc19094ec9b9
splat_kind: spell
splat_key: dnd2024:spells/move_earth
splat_name: Move Earth
splat_origin:
  key: dnd2024:reference/spell/move_earth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
---

# Move Earth

## Source text

<!-- SRD page 151 -->

Level 6 Transmutation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a miniature shovel)\
Duration: Concentration, up to 2 hours

Choose an area of terrain no larger than 40 feet on a side within range. You can reshape dirt, sand, or clay in the area in any manner you choose for the duration. You can raise or lower the area’s elevation, create or fill in a trench, erect or flatten a wall, or form a pillar. The extent of any such changes can’t exceed half the area’s largest dimension. For example, if you affect a 40-foot square, you can create a pillar up to 20 feet high, raise or lower the square’s elevation by up to 20 feet, dig a trench up to 20 feet deep, and so on. It takes 10 minutes for these changes to complete. Because the terrain’s transformation occurs slowly, creatures in the area can’t usually be trapped or injured by the ground’s movement.

At the end of every 10 minutes you spend concentrating on the spell, you can choose a new area of terrain to affect within range.

This spell can’t manipulate natural stone or stone construction. Rocks and structures shift to accommodate the new terrain. If the way you shape the terrain would make a structure unstable, it might collapse.

Similarly, this spell doesn’t directly affect plant growth. The moved earth carries any plants along with it.

Original layout: [[../../srd-5.2.1/pages/151|page 151]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 2 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a miniature shovel ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/move_earth ` |
| ` implementation ` | ` dnd2024:spell_move_earth ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` exclusions ` | ` ["natural_stone_manipulation", "stone_construction_manipulation", "direct_plant_growth_effect"] ` |
| ` carried_with_terrain ` | ` ["rocks", "structures", "plants"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` terrain_area ` |
| ` max_side_feet ` | ` 40 ` |
| ` within_range ` | ` true ` |
| ` materials ` | ` ["dirt", "sand", "clay"] ` |

#### Reshape

<!-- splat:yaml path=splat_rule/parameters/reshape -->

```yaml
chooser: caster
modes:
- raise_elevation
- lower_elevation
- create_trench
- fill_trench
- erect_wall
- flatten_wall
- form_pillar
- other_shape
max_extent:
  fraction_of_largest_area_dimension:
    numerator: 1
    denominator: 2
completion_minutes: 10
ordinary_trapping_or_injury: false
exceptional_trapping_or_injury:
  type: choice
  chooser: gm
  requires: exceptional_terrain_circumstances
  resolution_required: true
```

#### New area

<!-- splat:values path=splat_rule/parameters/new_area -->

| Field | Value |
| --- | --- |
| ` event ` | ` each_10_minutes_of_concentration_completed ` |
| ` optional ` | ` true ` |
| ` within_range ` | ` true ` |

#### Unstable structure

<!-- splat:yaml path=splat_rule/parameters/unstable_structure -->

```yaml
collapse:
  chooser: gm
  type: boolean
  requires: reshaping_makes_structure_unstable
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 2
  unit: hour
```
