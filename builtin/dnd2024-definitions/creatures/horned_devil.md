---
splat_schema: 2
splat_id: 95f04799-9101-5d44-ae70-280d00dde719
splat_kind: creature
splat_key: dnd2024:creatures/horned_devil
splat_name: Horned Devil
splat_origin: dnd2024:reference/creature/horned_devil
splat_sources:
- SRD 5.2.1, pp. 299, 300, https://www.dndbeyond.com/srd
---

# Horned Devil


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Fiend (Devil) |
| Alignment | Lawful Evil |
| Armor Class | 18 |
| Hit Points | 199 (19d10 + 95) |
| Initiative | +7 (17) |
| Speed | 30 ft., Fly 60 ft. |
| Challenge | 11 (XP 7,200; PB +4) |
| Senses | Darkvision 150 ft. (unimpeded by magical Darkness); Passive Perception 13 |
| Languages | Infernal; telepathy 120 ft. |
| Resistances | Cold |
| Immunities | Fire, Poison; Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 22 | +6 | +10 |
| Dex | 17 | +3 | +7 |
| Con | 21 | +5 | +5 |
| Int | 12 | +1 | +1 |
| Wis | 16 | +3 | +7 |
| Cha | 18 | +4 | +8 |


## Source text

<!-- SRD page 299 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Fiend (Devil), Lawful Evil\
> AC 18 Initiative +7 (17)\
> HP 199 (19d10 + 95)\
> Speed 30 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 22 +6 +10 Dex 17 +3 +7 Con 21 +5 +5\
> Int 12 +1 +1 Wis 16 +3 +7 Cha 18 +4 +8\
> Resistances Cold\
> Immunities Fire, Poison; Poisoned\
> Senses Darkvision 150 ft. (unimpeded by magical Darkness); Passive Perception 13\
> Languages Infernal; telepathy 120 ft.\
> CR 11 (XP 7,200; PB +4)


### Traits

Diabolical Restoration. If the devil dies outside the Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Nine Hells.

Magic Resistance. The devil has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The devil makes three attacks, using Searing Fork or Hurl Flame in any combination. It can replace one attack with a use of Infernal Tail.

<!-- SRD page 300 -->

Searing Fork. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (2d8 + 6) Piercing damage plus 9 (2d8) Fire damage.

Hurl Flame. Ranged Attack Roll: +8, range 150 ft. Hit: 26 (5d8 + 4) Fire damage. If the target is a flammable object that isn’t being worn or carried, it starts burning.

Infernal Tail. Dexterity Saving Throw: DC 17, one creature the devil can see within 10 feet. Failure: 10 (1d8 + 6) Necrotic damage, and the target receives an infernal wound if it doesn’t have one. While wounded, the target loses 10 (3d6) Hit Points at the start of each of its turns. The wound closes after 1 minute, after a spell restores Hit Points to the target, or after the target or a creature within 5 feet of it takes an action to stanch the wound, doing so by succeeding on a DC 17 Wisdom (Medicine) check.

Original layout: [[../../srd-5.2.1/pages/299|page 299]], [[../../srd-5.2.1/pages/300|page 300]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/horned_devil ` |
| ` implementation ` | ` dnd2024:creature_horned_devil ` |

### Parameters

#### Features

##### Magic resistance

<!-- splat:yaml path=splat_rule/parameters/features/magic_resistance -->

```yaml
economy: passive
advantage:
  roll: saving_throw
  against:
  - spell
  - magical_effect
```

##### Searing fork

<!-- splat:yaml path=splat_rule/parameters/features/searing_fork -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 10
targets:
  count: 1
hit:
- op: damage
  average: 15
  amount:
    dice:
      count: 2
      sides: 8
    add: 6
  damage_type: piercing
- op: damage
  average: 9
  amount:
    dice:
      count: 2
      sides: 8
    add: 0
  damage_type: fire
reach_feet: 10
```

##### Hurl flame

<!-- splat:yaml path=splat_rule/parameters/features/hurl_flame -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 8
targets:
  count: 1
hit:
- op: damage
  average: 26
  amount:
    dice:
      count: 5
      sides: 8
    add: 4
  damage_type: fire
- op: if
  test:
    target_kind: object
    flammable: true
    worn: false
    carried: false
  then:
  - op: ignite
range_feet: 150
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 3 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Searing fork ` |
| ` row_2 ` | ` Hurl flame ` |

###### Replacement

<!-- splat:yaml path=splat_rule/parameters/features/multiattack/replacement -->

```yaml
max_attacks: 1
choices:
- feature: infernal_tail
```

##### Diabolical restoration

<!-- splat:yaml path=splat_rule/parameters/features/diabolical_restoration -->

```yaml
economy: passive
resource:
  capacity: 1
  recovery:
  - long_rest
  cost: 1
trigger: self.died
requires:
  death_plane_not: nine_hells
op: revive
new_body: true
old_body: disappears
delay:
  seconds: 0
hit_points: maximum
destination:
  plane: nine_hells
  location_choice: gm
```

##### Infernal tail

<!-- splat:yaml path=splat_rule/parameters/features/infernal_tail -->

```yaml
economy: action
op: save
ability: dexterity
dc: 17
targets:
  kind: creature
  count: 1
  range_feet: 10
  visible_to: self
failure:
- op: damage
  average: 10
  amount:
    dice:
      count: 1
      sides: 8
    add: 6
  damage_type: necrotic
- op: if
  test:
    target_has_infernal_wound: false
  then:
  - op: infernal_wound
    stacking: one_per_target
    periodic:
      event: target.turn.start
      effects:
      - op: lose_hit_points
        average: 10
        amount:
          dice:
            count: 3
            sides: 6
          add: 0
    ends_any:
    - minutes: 1
    - event: spell.restores_hit_points
      target: affected
    - economy: action
      actor: target_or_creature_within_5_feet
      check:
        ability: wisdom
        skill: medicine
        dc: 17
      outcome: success
success: []
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire", "poison"] ` |
| ` conditions ` | ` ["poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet | magical_darkness_unimpeded |
| --- | --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 150 ` | ` true ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["infernal"] ` |
| ` understands ` | ` ["infernal"] ` |

##### Telepathy

<!-- splat:values path=splat_rule/parameters/communication/telepathy -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 120 ` |

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> ```

