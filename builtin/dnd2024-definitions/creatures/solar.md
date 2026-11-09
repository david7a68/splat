---
splat_schema: 2
splat_id: 9732fc05-ce97-5ce0-b0fb-baadafc54573
splat_kind: creature
splat_key: dnd2024:creatures/solar
splat_name: Solar
splat_origin: dnd2024:reference/creature/solar
splat_sources:
- SRD 5.2.1, pp. 326, 327, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 288, Habitat
---

# Solar


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Celestial (Angel) |
| Alignment | Lawful Good |
| Armor Class | 21 |
| Hit Points | 297 (22d10 + 176) |
| Initiative | +20 (30) |
| Speed | 50 ft., Fly 150 ft. (hover) |
| Challenge | 21 (XP 33,000; PB +7) |
| Skills | Perception +14 |
| Senses | Truesight 120 ft.; Passive Perception 24 |
| Languages | All; telepathy 120 ft. |
| Immunities | Poison, Radiant; Charmed, Exhaustion, Frightened, Poisoned |
| Habitat | Planar (Upper Planes) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 26 | +8 | +8 |
| Dex | 22 | +6 | +6 |
| Con | 26 | +8 | +8 |
| Int | 25 | +7 | +7 |
| Wis | 25 | +7 | +7 |
| Cha | 30 | +10 | +10 |


## Source text

<!-- SRD page 326 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Celestial (Angel), Lawful Good\
> AC 21 Initiative +20 (30)\
> HP 297 (22d10 + 176)\
> Speed 50 ft., Fly 150 ft. (hover)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 26 +8 +8 Dex 22 +6 +6 Con 26 +8 +8\
> Int 25 +7 +7 Wis 25 +7 +7 Cha 30 +10 +10\
> Skills Perception +14\
> Immunities Poison, Radiant; Charmed, Exhaustion, Frightened, Poisoned\
> Senses Truesight 120 ft.; Passive Perception 24\
> Languages All; telepathy 120 ft.\
> CR 21 (XP 33,000; PB +7)


### Traits

Divine Awareness. The solar knows if it hears a lie.

Exalted Restoration. If the solar dies outside Mount Celestia, its body disappears, and it gains a new body instantly, reviving with all its Hit Points somewhere in Mount Celestia.

Legendary Resistance (4/Day). If the solar fails a saving throw, it can choose to succeed instead.

Magic Resistance. The solar has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The solar makes two Flying Sword attacks. It can replace one attack with a use of Slaying Bow.

Flying Sword. Melee or Ranged Attack Roll: +15, reach 10 ft. or range 120 ft. Hit: 22 (4d6 + 8) Slashing damage plus 36 (8d8) Radiant damage. Hit or Miss: The sword magically returns to the solar’s hand or hovers within 5 feet of the solar immediately after a ranged attack.

Slaying Bow. Dexterity Saving Throw: DC 21, one creature the solar can see within 600 feet. Failure: If the creature has 100 Hit Points or fewer, it dies. It otherwise takes 24 (4d8 + 6) Piercing damage plus 36 (8d8) Radiant damage.

Spellcasting. The solar casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 25):

At Will: Detect Evil and Good\
1/Day Each: Commune, Control Weather, Dispel Evil and Good, Resurrection

### Bonus Actions

Divine Aid (3/Day). The solar casts Cure Wounds (level 2 version), Lesser Restoration, or Remove Curse, using the same spellcasting ability as Spellcasting.

<!-- SRD page 327 -->

### Legendary Actions

Legendary Action Uses: 3. Immediately after another creature’s turn, the solar can expend a use to take one of the following actions. The solar regains all expended uses at the start of each of its turns.

Blinding Gaze. Constitution Saving Throw: DC 25, one creature the solar can see within 120 feet. Failure: The target has the Blinded condition for 1 minute. Failure or Success: The solar can’t take this action again until the start of its next turn.

Radiant Teleport. The solar teleports up to 60 feet to an unoccupied space it can see. Dexterity Saving Throw: DC 25, each creature in a 10-foot Emanation originating from the solar at its destination space. Failure: 11 (2d10) Radiant damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/326|page 326]], [[../../srd-5.2.1/pages/327|page 327]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/solar ` |
| ` implementation ` | ` dnd2024:creature_solar ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison", "radiant"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "poisoned"] ` |
| ` passive_perception ` | ` 24 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 14 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` truesight ` | ` 120 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
all: true
```

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` range_ft ` | ` 120 ` |
| ` two_way ` | ` true ` |

#### Features

##### Divine awareness

<!-- splat:values path=splat_rule/parameters/features/divine_awareness -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` lie.detect ` |
| ` requires ` | ` hear_lie ` |
| ` automatic ` | ` true ` |

##### Exalted restoration

<!-- splat:yaml path=splat_rule/parameters/features/exalted_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane_not: mount_celestia
body: disappears
revive:
  new_body: true
  plane: mount_celestia
  location_choice: gm
  hp: maximum
  delay:
    amount: 0
    unit: second
```

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
trigger: saving_throw.failed
optional: true
op: saving_throw.set_result
result: success
uses:
  count: 4
  recovery: long_rest
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Flying sword

<!-- splat:yaml path=splat_rule/parameters/features/flying_sword -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 15
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
range_ft:
  normal: 120
  long: 120
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 4
        sides: 6
      add: 8
    average: 22
  - type: radiant
    amount:
      dice:
        count: 8
        sides: 8
      add: 0
    average: 36
after_attack:
  when: ranged_attack_hit_or_miss
  op: item.return
  destination: actor_hand
  magical: true
  destination_choice:
  - actor_hand
  - hover_within_5_ft
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
| ` save_dc ` | ` 25 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_evil_and_good
- commune
- control_weather
- dispel_evil_and_good
- resurrection
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
-->

| Spell | Uses unlimited | Uses count | Uses recovery |
| --- | --- | --- | --- |
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | true |  |  |
| [Commune](<../spells/commune.md> "dnd2024:spells/commune") |  | ` 1 ` | ` Long rest ` |
| [Control Weather](<../spells/control_weather.md> "dnd2024:spells/control_weather") |  | ` 1 ` | ` Long rest ` |
| [Dispel Evil and Good](<../spells/dispel_evil_and_good.md> "dnd2024:spells/dispel_evil_and_good") |  | ` 1 ` | ` Long rest ` |
| [Resurrection](<../spells/resurrection.md> "dnd2024:spells/resurrection") |  | ` 1 ` | ` Long rest ` |

##### Slaying bow

<!-- splat:yaml path=splat_rule/parameters/features/slaying_bow -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 21
targets:
  kind: creature
  count: 1
  range_ft: 600
  visible_to_actor: true
failure:
- op: branch
  if:
    target_hp_at_most: 100
  then:
  - op: death
  else:
  - op: damage
    components:
    - type: piercing
      amount:
        dice:
          count: 4
          sides: 8
        add: 6
      average: 24
    - type: radiant
      amount:
        dice:
          count: 8
          sides: 8
        add: 0
      average: 36
success: []
```

##### Divine aid

<!-- splat:yaml path=splat_rule/parameters/features/divine_aid -->

```yaml
economy: bonus_action
op: spellcasting
profile:
  ability: same_as_spellcasting
  waived_components: []
choose_count: 1
choices:
- op: spell.cast
  spell:
    key: dnd2024:spells/cure_wounds
  cast_level: 2
- op: spell.cast
  spell:
    key: dnd2024:spells/lesser_restoration
- op: spell.cast
  spell:
    key: dnd2024:spells/remove_curse
uses:
  count: 3
  recovery: long_rest
```

##### Blinding gaze

<!-- splat:yaml path=splat_rule/parameters/features/blinding_gaze -->

```yaml
economy: legendary_action
op: saving_throw
ability: constitution
dc: 25
targets:
  kind: creature
  count: 1
  range_ft: 120
  visible_to_actor: true
failure:
- op: condition.apply
  condition: blinded
  duration:
    amount: 1
    unit: minute
success: []
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Radiant teleport

<!-- splat:yaml path=splat_rule/parameters/features/radiant_teleport -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: teleport
  maximum_ft: 60
  destination:
    unoccupied: true
    visible_to_actor: true
- op: saving_throw
  ability: dexterity
  dc: 25
  targets:
    kind: creature
    count: all
    area:
      shape: emanation
      origin: actor
      size_ft: 10
  failure:
  - op: damage
    components:
    - type: radiant
      amount:
        dice:
          count: 2
          sides: 10
        add: 0
      average: 11
  success:
  - op: damage
    from: failure_damage
    multiplier: 0.5
    round: down
legendary_cost: 1
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: flying_sword
replace_attack:
  maximum: 1
  choices:
  - op: feature.use
    feature: slaying_bow
```

#### Legendary actions

<!-- splat:values path=splat_rule/parameters/legendary_actions -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 3 ` |
| ` uses_in_lair ` | ` 3 ` |
| ` trigger ` | ` another_creature.turn_end_immediately_after ` |
| ` cost_per_action ` | ` 1 ` |
| ` recover_all ` | ` actor.turn_start ` |
| ` maximum_per_trigger ` | ` 1 ` |
| ` requires_actions_available ` | ` true ` |
| ` excluded_conditions ` | ` ["incapacitated"] ` |

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
>   value: 150
> ```

