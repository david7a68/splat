---
splat_schema: 2
splat_id: bd4d8a3d-0379-5024-afe8-33b59cf25361
splat_kind: spell
splat_key: dnd2024:spells/control_water
splat_name: Control Water
splat_origin: dnd2024:reference/spell/control_water
splat_sources:
- SRD 5.2.1, pp. 119, 120, https://www.dndbeyond.com/srd
---

# Control Water


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Transmutation |
| Classes | Cleric, Druid, Wizard |
| Casting Time | Action |
| Range | 300 feet |
| Components | V, S, M (a mixture of water and dust) |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 119 -->

Level 4 Transmutation (Cleric, Druid, Wizard)\
Casting Time: Action\
Range: 300 feet\
Components: V, S, M (a mixture of water and dust)\
Duration: Concentration, up to 10 minutes

Until the spell ends, you control any water inside an area you choose that is a Cube up to 100 feet on a side, using one of the following effects. As a Magic action on your later turns, you can repeat the same effect or choose a different one.

Flood. You cause the water level of all standing water in the area to rise by as much as 20 feet. If you choose an area in a large body of water, you instead create a 20-foot tall wave that travels from one side of the area to the other and then crashes. Any Huge or smaller vehicles in the wave’s path are carried with it to the other side. Any Huge or smaller vehicles struck by the wave have a 25 percent chance of capsizing.

<!-- SRD page 120 -->

The water level remains elevated until the spell ends or you choose a different effect. If this effect produced a wave, the wave repeats on the start of your next turn while the flood effect lasts.

Part Water. You part water in the area and create a trench. The trench extends across the spell’s area, and the separated water forms a wall to either side. The trench remains until the spell ends or you choose a different effect. The water then slowly fills in the trench over the course of the next round until the normal water level is restored.

Redirect Flow. You cause flowing water in the area to move in a direction you choose, even if the water has to flow over obstacles, up walls, or in other unlikely directions. The water in the area moves as you direct it, but once it moves beyond the spell’s area, it resumes its flow based on the terrain. The water continues to move in the direction you chose until the spell ends or you choose a different effect.

Whirlpool. You cause a whirlpool to form in the center of the area, which must be at least 50 feet square and 25 feet deep. The whirlpool lasts until you choose a different effect or the spell ends. The whirlpool is 5 feet wide at the base, up to 50 feet wide at the top, and 25 feet tall. Any creature in the water and within 25 feet of the whirlpool is pulled 10 feet toward it. When a creature enters the whirlpool for the first time on a turn or ends its turn there, it makes a Strength saving throw. On a failed save, the creature takes 2d8 Bludgeoning damage. On a successful save, the creature takes half as much damage. A creature can swim away from the whirlpool only if it first takes an action to pull away and succeeds on a Strength (Athletics) check against your spell save DC.

Original layout: [[../../srd-5.2.1/pages/119|page 119]], [[../../srd-5.2.1/pages/120|page 120]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/control_water ` |
| ` implementation ` | ` dnd2024:spell_control_water ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` effect_choice ` | ` ["flood", "part_water", "redirect_flow", "whirlpool"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` water_in_area ` |
| ` shape ` | ` cube ` |
| ` max_side_feet ` | ` 100 ` |
| ` within_range ` | ` true ` |

#### Change or repeat

<!-- splat:values path=splat_rule/parameters/change_or_repeat -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` timing ` | ` later_caster_turns ` |

#### Flood

<!-- splat:yaml path=splat_rule/parameters/flood -->

```yaml
standing_water_rise_max_feet: 20
large_body_alternative:
  wave_height_feet: 20
  travels: across_area
  vehicle_size_max: huge
  vehicles:
    carried_to: opposite_side
    capsize_chance_percent: 25
  repeat: caster_next_turn_start_while_flood
ends_on:
- spell_ends
- different_effect_chosen
```

#### Part water

<!-- splat:values path=splat_rule/parameters/part_water -->

| Field | Value |
| --- | --- |
| ` trench ` | ` across_area ` |
| ` walls ` | ` both_sides ` |
| ` ends_on ` | ` ["spell_ends", "different_effect_chosen"] ` |
| ` refill_duration_rounds ` | ` 1 ` |

#### Redirect flow

<!-- splat:values path=splat_rule/parameters/redirect_flow -->

| Field | Value |
| --- | --- |
| ` direction ` | ` caster_choice ` |
| ` allowed ` | ` ["over_obstacles", "up_walls", "other_directions"] ` |
| ` outside_area ` | ` terrain_normal_flow ` |
| ` ends_on ` | ` ["spell_ends", "different_effect_chosen"] ` |

#### Whirlpool

<!-- splat:yaml path=splat_rule/parameters/whirlpool -->

```yaml
minimum_water_area_feet:
- 50
- 50
minimum_water_depth_feet: 25
geometry:
  base_width_feet: 5
  top_width_max_feet: 50
  height_feet: 25
  location: area_center
pull:
  targets: creatures_in_water
  within_feet: 25
  distance_feet: 10
  toward: whirlpool
damage:
  events:
  - first_entry_each_turn
  - target_turn_end_inside
  save_ability: strength
  failure:
    dice: 2d8
    type: bludgeoning
  success_fraction:
    numerator: 1
    denominator: 2
swim_away:
  prerequisite:
    economy: action
    check:
      ability: strength
      skill: athletics
      dc: spell_save_dc
      success_required: true
ends_on:
- spell_ends
- different_effect_chosen
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 10
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

