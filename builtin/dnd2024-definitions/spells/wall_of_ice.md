---
splat_schema: 2
splat_id: 70ca2817-b63c-53bb-afea-e4dc74389ef2
splat_kind: spell
splat_key: dnd2024:spells/wall_of_ice
splat_name: Wall of Ice
splat_origin: dnd2024:reference/spell/wall_of_ice
splat_sources:
- SRD 5.2.1, pp. 172, 173, https://www.dndbeyond.com/srd
---

# Wall of Ice


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Evocation |
| Classes | Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (a piece of quartz) |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 172 -->

Level 6 Evocation (Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a piece of quartz)\
Duration: Concentration, up to 10 minutes

You create a wall of ice on a solid surface within range. You can form it into a hemispherical dome or a globe with a radius of up to 10 feet, or you can shape a flat surface made up of ten 10-foot-square panels. Each panel must be contiguous with another panel. In any form, the wall is 1 foot thick and lasts for the duration.

If the wall cuts through a creature’s space when it appears, the creature is pushed to one side of the wall (you choose which side) and makes a Dexterity saving throw, taking 10d6 Cold damage on a failed save or half as much damage on a successful one.

The wall is an object that can be damaged and thus breached. It has AC 12 and 30 Hit Points per 10-foot section, and it has Immunity to Cold, Poison, and Psychic damage and Vulnerability to Fire damage. Reducing a 10-foot section of wall to 0 Hit Points destroys it and leaves behind a sheet of frigid air in the space the wall occupied.

A creature moving through the sheet of frigid air for the first time on a turn makes a Constitution saving throw, taking 5d6 Cold damage on a failed save or half as much damage on a successful one.

<!-- SRD page 173 -->

Using a Higher-Level Spell Slot. The damage the wall deals when it appears increases by 2d6 and the damage from passing through the sheet of frigid air increases by 1d6 for each spell slot level above 6.

Original layout: [[../../srd-5.2.1/pages/172|page 172]], [[../../srd-5.2.1/pages/173|page 173]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/wall_of_ice ` |
| ` implementation ` | ` dnd2024:spell_wall_of_ice ` |

### Parameters

#### Creation

<!-- splat:yaml path=splat_rule/parameters/creation -->

```yaml
kind: object
surface: solid
within_spell_range: true
thickness_feet: 1
duration: spell_duration
shape_choice:
  hemispherical_dome:
    radius_feet_maximum: 10
  globe:
    radius_feet_maximum: 10
  flat_surface:
    panels: 10
    panel_side_feet: 10
    each_panel_contiguous_with_another: true
ac: 12
hp_per_10_foot_section: 30
damage_immunities:
- cold
- poison
- psychic
damage_vulnerabilities:
- fire
```

#### Initial intersection

<!-- splat:yaml path=splat_rule/parameters/initial_intersection -->

```yaml
target: creature_whose_space_wall_cuts
push_to_side:
  chooser: caster
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage:
      dice:
        count: &id001
          op: add
          args:
          - 10
          - op: multiply
            args:
            - 2
            - op: subtract
              args:
              - binding: cast_level
              - 6
        sides: 6
        add: 0
      type: cold
  success:
    damage:
      dice:
        count: *id001
        sides: 6
        add: 0
      type: cold
    damage_fraction:
      numerator: 1
      denominator: 2
```

#### Section destroyed

<!-- splat:values path=splat_rule/parameters/section_destroyed -->

| Field | Value |
| --- | --- |
| ` hp ` | ` 0 ` |
| ` replace_with ` | ` sheet_of_frigid_air ` |
| ` location ` | ` former_section_space ` |

#### Frigid air

<!-- splat:yaml path=splat_rule/parameters/frigid_air -->

```yaml
trigger: creature_moves_through_first_time_per_turn
save:
  ability: constitution
  dc:
    binding: save_dc
  failure:
    damage:
      dice:
        count:
          op: add
          args:
          - 5
          - op: multiply
            args:
            - 1
            - op: subtract
              args:
              - binding: cast_level
              - 6
        sides: 6
        add: 0
      type: cold
  success:
    damage:
      dice:
        count:
          op: add
          args:
          - 5
          - op: multiply
            args:
            - 1
            - op: subtract
              args:
              - binding: cast_level
              - 6
        sides: 6
        add: 0
      type: cold
    damage_fraction:
      numerator: 1
      denominator: 2
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

