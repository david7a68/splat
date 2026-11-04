---
splat_schema: 2
splat_id: b796317c-1598-54c7-b51e-9b844d81987f
splat_kind: spell
splat_key: dnd2024:spells/guards_and_wards
splat_name: Guards and Wards
splat_origin: dnd2024:reference/spell/guards_and_wards
splat_sources:
- SRD 5.2.1, pp. 137, 138, https://www.dndbeyond.com/srd
---

# Guards and Wards


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Abjuration |
| Classes | Bard, Wizard |
| Casting Time | 1 hour |
| Range |  |
| Components | V, S, M (a silver rod worth 10+ GP) |
| Duration | 24 hours |


## Source text

<!-- SRD page 137 -->

Level 6 Abjuration (Bard, Wizard)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a silver rod worth 10+ GP)\
Duration: 24 hours

You create a ward that protects up to 2,500 square feet of floor space. The warded area can be up to 20 feet tall, and you shape it as one 50-foot square, one hundred 5-foot squares that are contiguous, or twenty-five 10-foot squares that are contiguous.

When you cast this spell, you can specify individuals that are unaffected by the spell’s effects. You can also specify a password that, when spoken aloud within 5 feet of the warded area, makes the speaker immune to its effects.

<!-- SRD page 138 -->

The spell creates the effects below within the warded area. Dispel Magic has no effect on Guards and Wards itself, but each of the following effects can be dispelled. If all four are dispelled, Guards and Wards ends. If you cast the spell every day for 365 days on the same area, the spell thereafter lasts until all its effects are dispelled.

Corridors. Fog fills all the warded corridors, making them Heavily Obscured. In addition, at each intersection or branching passage offering a choice of direction, there is a 50 percent chance that a creature other than you believes it is going in the opposite direction from the one it chooses.

Doors. All doors in the warded area are magically locked, as if sealed by the Arcane Lock spell. In addition, you can cover up to ten doors with an illusion to make them appear as plain sections of wall.

Stairs. Webs fill all stairs in the warded area from top to bottom, as in the Web spell. These strands regrow in 10 minutes if they are destroyed while Guards and Wards lasts.

Other Spell Effect. Place one of the following magical effects within the warded area:

•  Dancing Lights in four corridors, with a simple program that the lights repeat as long as Guards and Wards lasts

•  Magic Mouth in two locations

•  Stinking Cloud in two locations (the vapors return within 10 minutes if dispersed while Guards and Wards lasts)

•  Gust of Wind in one corridor or room (the wind blows continuously while the spell lasts)

•  Suggestion in one 5-foot square; any creature that enters that square receives the suggestion mentally

Original layout: [[../../srd-5.2.1/pages/137|page 137]], [[../../srd-5.2.1/pages/138|page 138]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/guards_and_wards ` |
| ` implementation ` | ` dnd2024:spell_guards_and_wards ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 86400 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` maximum_square_feet ` | ` 2500 ` |
| ` maximum_height_feet ` | ` 20 ` |

##### Shape choices

<!-- splat:table
path: splat_rule/parameters/area/shape_choices
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Squares:
    path: squares
    type: integer
  Side feet:
    path: side_feet
    type: integer
  Contiguous:
    path: contiguous
    type: boolean
    optional: true
-->

| Key | Squares | Side feet | Contiguous |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 50 ` |  |
| ` row_2 ` | ` 100 ` | ` 5 ` | true |
| ` row_3 ` | ` 25 ` | ` 10 ` | true |

#### Immunity

<!-- splat:yaml path=splat_rule/parameters/immunity -->

```yaml
designated_individuals:
  chooser: caster
  timing: cast
password:
  chooser: caster
  spoken_aloud: true
  within_area_feet: 5
  grant_to: speaker
```

#### Dispelling

<!-- splat:values path=splat_rule/parameters/dispelling -->

| Field | Value |
| --- | --- |
| ` whole_spell_targetable_by_dispel_magic ` | ` false ` |
| ` independently_dispellable_effect_groups ` | ` ["corridors", "doors", "stairs", "other_spell_effect"] ` |
| ` all_four_removed ` | ` end_spell ` |

#### Permanence

<!-- splat:values path=splat_rule/parameters/permanence -->

| Field | Value |
| --- | --- |
| ` repeat_casts ` | ` 365 ` |
| ` frequency ` | ` daily ` |
| ` same_area ` | ` true ` |
| ` result_duration ` | ` until_all_effect_groups_dispelled ` |

#### Corridors

<!-- splat:yaml path=splat_rule/parameters/corridors -->

```yaml
scope: all_warded_corridors
fog_obscurement: heavy
misdirection:
  at: intersection_or_branch_with_direction_choice
  exclude_caster: true
  probability_percent: 50
  perceived_direction: opposite_chosen_direction
```

#### Doors

<!-- splat:yaml path=splat_rule/parameters/doors -->

```yaml
scope: all_warded_doors
lock_spell:
  key: dnd2024:spells/arcane_lock
concealed_doors:
  chooser: caster
  maximum: 10
  appearance: plain_wall
```

#### Stairs

<!-- splat:yaml path=splat_rule/parameters/stairs -->

```yaml
scope: all_warded_stairs_top_to_bottom
effect_spell:
  key: dnd2024:spells/web
destroyed_web_regrowth_seconds: 600
regrowth_requires_spell_active: true
```

#### Other spell effect

<!-- splat:yaml path=splat_rule/parameters/other_spell_effect -->

```yaml
choose_count: 1
chooser: caster
options:
  dancing_lights:
    spell:
      key: dnd2024:spells/dancing_lights
    locations: 4
    location_kind: corridor
    behavior: repeated_simple_program
    duration: guards_and_wards
  magic_mouth:
    spell:
      key: dnd2024:spells/magic_mouth
    locations: 2
  stinking_cloud:
    spell:
      key: dnd2024:spells/stinking_cloud
    locations: 2
    dispersed_vapor_return_seconds_max: 600
    return_requires_spell_active: true
  gust_of_wind:
    spell:
      key: dnd2024:spells/gust_of_wind
    locations: 1
    location_kinds:
    - corridor
    - room
    wind: continuous_for_duration
  suggestion:
    spell:
      key: dnd2024:spells/suggestion
    square_side_feet: 5
    event: creature.entered_area
    delivery: mental
```

#### Nested spell invocation

<!-- splat:values path=splat_rule/parameters/nested_spell_invocation -->

| Field | Value |
| --- | --- |
| ` additional_casting_actions ` | ` false ` |
| ` additional_spell_slots ` | ` false ` |
| ` concentration_owner ` | ` guards_and_wards_lifetime ` |
| ` parent_exemptions_apply ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

