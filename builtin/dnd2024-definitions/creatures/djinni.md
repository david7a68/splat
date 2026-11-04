---
splat_schema: 2
splat_id: 7adeeade-0bcf-5c22-a420-eb750d4da8f6
splat_kind: creature
splat_key: dnd2024:creatures/djinni
splat_name: Djinni
splat_origin: dnd2024:reference/creature/djinni
splat_sources:
- SRD 5.2.1, pp. 280, https://www.dndbeyond.com/srd
---

# Djinni


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Elemental (Genie) |
| Alignment | Neutral |
| Armor Class | 17 |
| Hit Points | 218 (19d10 + 114) |
| Initiative | +2 (12) |
| Speed | 30 ft., Fly 90 ft. (hover) |
| Challenge | 11 (XP 7,200; PB +4) |
| Senses | Darkvision 120 ft.; Passive Perception 13 |
| Languages | Primordial (Auran) |
| Immunities | Lightning, Thunder |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 21 | +5 | +5 |
| Dex | 15 | +2 | +6 |
| Con | 22 | +6 | +6 |
| Int | 15 | +2 | +2 |
| Wis | 16 | +3 | +7 |
| Cha | 20 | +5 | +5 |


## Source text

<!-- SRD page 280 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Elemental (Genie), Neutral\
> AC 17  Initiative +2 (12)\
> HP 218 (19d10 + 114)\
> Speed 30 ft., Fly 90 ft. (hover)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 21 +5 +5 Dex 15 +2 +6 Con 22 +6 +6\
> Int 15 +2 +2 Wis 16 +3 +7 Cha 20 +5 +5\
> Immunities Lightning, Thunder\
> Senses Darkvision 120 ft.; Passive Perception 13\
> Languages Primordial (Auran)\
> CR 11 (XP 7,200; PB +4)


### Traits

Elemental Restoration. If the djinni dies outside the Elemental Plane of Air, its body dissolves into mist, and it gains a new body in 1d4 days, reviving with all its Hit Points somewhere on the Plane of Air.

Magic Resistance. The djinni has Advantage on saving throws against spells and other magical effects.

Wishes. The djinni has a 30 percent chance of knowing the Wish spell. If the djinni knows it, the djinni can cast it only on behalf of a non-genie creature who communicates a wish in a way the djinni can understand. If the djinni casts the spell for the creature, the djinni suffers none of the spell’s stress. Once the djinni has cast it three times, the djinni can’t do so again for 365 days.

### Actions

Multiattack. The djinni makes three attacks, using Storm Blade or Storm Bolt in any combination.

Storm Blade. Melee Attack Roll: +9, reach 5 feet. Hit: 12 (2d6 + 5) Slashing damage plus 7 (2d6) Lightning damage.

Storm Bolt. Ranged Attack Roll: +9, range 120 feet. Hit: 13 (3d8) Thunder damage. If the target is a Large or smaller creature, it has the Prone condition.

Create Whirlwind. The djinni conjures a whirlwind at a point it can see within 120 feet. The whirlwind fills a 20-foot-radius, 60-foot-high Cylinder centered on that point. The whirlwind lasts until the djinni’s Concentration on it ends. The djinni can move the whirlwind up to 20 feet at the start of each of its turns.

Whenever the whirlwind enters a creature’s space or a creature enters the whirlwind, that creature is subjected to the following effect. Strength Saving Throw: DC 17 (a creature makes this save only once per turn, and the djinni is unaffected). Failure: While in the whirlwind, the target has the Restrained condition and moves with the whirlwind. At the start of each of its turns, the Restrained target takes 21 (6d6) Thunder damage. At the end of each of its turns, the target repeats the save, ending the effect on itself on a success.

Spellcasting. The djinni casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 17):

At Will: Detect Evil and Good, Detect Magic\
2/Day Each: Create Food and Water (can create wine instead of water), Tongues, Wind Walk\
1/Day Each: Creation, Gaseous Form, Invisibility, Major Image, Plane Shift

Original layout: [[../../srd-5.2.1/pages/280|page 280]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/djinni ` |
| ` implementation ` | ` dnd2024:creature_djinni ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["lightning", "thunder"] ` |
| ` passive_perception ` | ` 13 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["auran"] ` |

#### Features

##### Elemental restoration

<!-- splat:yaml path=splat_rule/parameters/features/elemental_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane_not: elemental_plane_of_air
body: disappears
revive:
  new_body: true
  plane: elemental_plane_of_air
  location_choice: gm
  hp: maximum
  delay:
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    unit: day
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Wishes

<!-- splat:yaml path=splat_rule/parameters/features/wishes -->

```yaml
economy: passive
known_chance_percent: 30
spell:
  key: dnd2024:spells/wish
requester:
  excluded_subtype: genie
  wish_communicated_and_understood: true
caster_stress: false
uses: 3
after_third_cast_lockout:
  amount: 365
  unit: day
reset_uses_after_lockout: true
```

##### Storm blade

<!-- splat:yaml path=splat_rule/parameters/features/storm_blade -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 9
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 6
      add: 5
    average: 12
  - type: lightning
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
```

##### Storm bolt

<!-- splat:yaml path=splat_rule/parameters/features/storm_bolt -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 9
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 120
  long: 120
hit:
- op: damage
  components:
  - type: thunder
    amount:
      dice:
        count: 3
        sides: 8
      add: 0
    average: 13
- op: condition.apply
  condition: prone
  when:
    target_kind: creature
    target_maximum_size: large
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` spellcasting ` |

###### Profile

<!-- splat:values path=splat_rule/parameters/features/spellcasting/profile -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 17 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_evil_and_good
- detect_magic
- create_food_and_water
- tongues
- wind_walk
- creation
- gaseous_form
- invisibility
- major_image
- plane_shift
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
    optional: true
  Uses count:
    path: uses/count
    type: integer
    optional: true
  Uses recovery:
    path: uses/recovery
    type: symbol
    optional: true
  Overrides water replacement choice:
    path: overrides/water_replacement_choice
    type: yaml
    optional: true
-->

| Spell | Uses unlimited | Uses count | Uses recovery | Overrides water replacement choice |
| --- | --- | --- | --- | --- |
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | true |  |  |  |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |  |
| [Create Food and Water](<../spells/create_food_and_water.md> "dnd2024:spells/create_food_and_water") |  | ` 2 ` | ` Long rest ` | ` ["water", "wine"] ` |
| [Tongues](<../spells/tongues.md> "dnd2024:spells/tongues") |  | ` 2 ` | ` Long rest ` |  |
| [Wind Walk](<../spells/wind_walk.md> "dnd2024:spells/wind_walk") |  | ` 2 ` | ` Long rest ` |  |
| [Creation](<../spells/creation.md> "dnd2024:spells/creation") |  | ` 1 ` | ` Long rest ` |  |
| [Gaseous Form](<../spells/gaseous_form.md> "dnd2024:spells/gaseous_form") |  | ` 1 ` | ` Long rest ` |  |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") |  | ` 1 ` | ` Long rest ` |  |
| [Major Image](<../spells/major_image.md> "dnd2024:spells/major_image") |  | ` 1 ` | ` Long rest ` |  |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") |  | ` 1 ` | ` Long rest ` |  |

##### Create whirlwind

<!-- splat:yaml path=splat_rule/parameters/features/create_whirlwind -->

```yaml
economy: action
op: area.create
target_location:
  range_ft: 120
  visible_to_actor: true
area:
  shape: cylinder
  radius_ft: 20
  height_ft: 60
duration:
  until: actor.concentration_end
movement:
  event: actor.turn_start
  maximum_ft: 20
  optional: true
entry_events:
- area.enters_creature_space
- creature.enters_area
entry_save:
  op: saving_throw
  ability: strength
  dc: 17
  targets:
    kind: creature
    exclude: actor
    maximum_checks_per_turn: 1
  failure:
  - op: condition.apply
    condition: restrained
    duration:
      until: leaves_area_or_successful_repeat_save
    moves_with_area: true
    on_target_turn_start:
    - op: damage
      components:
      - type: thunder
        amount:
          dice:
            count: 6
            sides: 6
          add: 0
        average: 21
    repeat_save:
      event: target.turn_end
      ability: strength
      dc: 17
      success: effect.end
  success: []
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 3
  choices:
  - storm_blade
  - storm_bolt
  combination: any
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 90
> ```

