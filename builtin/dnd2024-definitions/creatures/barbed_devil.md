---
splat_schema: 2
splat_id: fac6b339-4dd8-5a57-9047-a586aea5960b
splat_kind: creature
splat_key: dnd2024:creatures/barbed_devil
splat_name: Barbed Devil
splat_origin: dnd2024:reference/creature/barbed_devil
splat_sources:
- SRD 5.2.1, pp. 262, https://www.dndbeyond.com/srd
---

# Barbed Devil


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fiend (Devil) |
| Alignment | Lawful Evil |
| Armor Class | 15 |
| Hit Points | 110 (13d8 + 52) |
| Initiative | +3 (13) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Deception +5, Insight +5, Perception +8 |
| Senses | Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 18 |
| Languages | Infernal; telepathy 120 ft. |
| Resistances | Cold |
| Immunities | Fire, Poison; Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +6 |
| Dex | 17 | +3 | +3 |
| Con | 18 | +4 | +7 |
| Int | 12 | +1 | +1 |
| Wis | 14 | +2 | +5 |
| Cha | 14 | +2 | +5 |


## Source text

<!-- SRD page 262 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fiend (Devil), Lawful Evil\
> AC 15 Initiative +3 (13)\
> HP 110 (13d8 + 52)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +6 Dex 17 +3 +3 Con 18 +4 +7\
> Int 12 +1 +1 Wis 14 +2 +5 Cha 14 +2 +5\
> Skills Deception +5, Insight +5, Perception +8\
> Resistances Cold\
> Immunities Fire, Poison; Poisoned\
> Senses Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 18\
> Languages Infernal; telepathy 120 ft.\
> CR 5 (XP 1,800; PB +3)


### Traits

Barbed Hide. At the start of each of its turns, the devil deals 5 (1d10) Piercing damage to any creature it is grappling or any creature grappling it.

Diabolical Restoration. If the devil dies outside the Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Nine Hells.

Magic Resistance. The devil has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The devil makes one Claws attack and one Tail attack, or it makes two Hurl Flame attacks.

Claws. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 13) from both claws.

Tail. Melee Attack Roll: +6, reach 10 ft. Hit: 14 (2d10 + 3) Slashing damage.

Hurl Flame. Ranged Attack Roll: +5, range 150 ft. Hit: 17 (5d6) Fire damage. If the target is a flammable object that isn’t being worn or carried, it starts burning.

Original layout: [[../../srd-5.2.1/pages/262|page 262]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/barbed_devil ` |
| ` implementation ` | ` dnd2024:creature_barbed_devil ` |

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

##### Claws

<!-- splat:yaml path=splat_rule/parameters/features/claws -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 10
  amount:
    dice:
      count: 2
      sides: 6
    add: 3
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: grappled
    escape_dc: 13
    source: self
    appendages:
      kind: claw
      available: 2
      used: 2
reach_feet: 5
```

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 14
  amount:
    dice:
      count: 2
      sides: 10
    add: 3
  damage_type: slashing
reach_feet: 10
```

##### Hurl flame

<!-- splat:yaml path=splat_rule/parameters/features/hurl_flame -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 17
  amount:
    dice:
      count: 5
      sides: 6
    add: 0
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

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: choose_sequence
choices:
- - op: use_feature
    feature: claws
    count: 1
  - op: use_feature
    feature: tail
    count: 1
- - op: use_feature
    feature: hurl_flame
    count: 2
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

##### Barbed hide

<!-- splat:yaml path=splat_rule/parameters/features/barbed_hide -->

```yaml
economy: passive
trigger: self.turn.start
targets:
  kind: creature
  relation_any:
  - grappled_by_self
  - grappling_self
  count: all
effects:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 10
    add: 0
  damage_type: piercing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 5 ` |
| ` insight ` | ` 5 ` |
| ` perception ` | ` 8 ` |

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
| ` passive_perception ` | ` 18 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet | magical_darkness_unimpeded |
| --- | --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` | ` true ` |

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
> climb_speed:
>   unit: ft
>   value: 30
> ```

