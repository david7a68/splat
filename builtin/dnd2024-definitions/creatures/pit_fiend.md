---
splat_schema: 2
splat_id: 92583734-40ca-5435-8b21-6b5d73e2cfd5
splat_kind: creature
splat_key: dnd2024:creatures/pit_fiend
splat_name: Pit Fiend
splat_origin: dnd2024:reference/creature/pit_fiend
splat_sources:
- SRD 5.2.1, pp. 314, 315, https://www.dndbeyond.com/srd
---

# Pit Fiend


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Fiend (Devil) |
| Alignment | Lawful Evil |
| Armor Class | 21 |
| Hit Points | 337 (27d10 + 189) |
| Initiative | +14 (24) |
| Speed | 30 ft., Fly 60 ft. |
| Challenge | 20 (XP 25,000; PB +6) |
| Skills | Perception +10, Persuasion +19 |
| Senses | Truesight 120 ft.; Passive Perception 20 |
| Languages | Infernal; telepathy 120 ft. |
| Resistances | Cold |
| Immunities | Fire, Poison; Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 26 | +8 | +8 |
| Dex | 14 | +2 | +8 |
| Con | 24 | +7 | +7 |
| Int | 22 | +6 | +6 |
| Wis | 18 | +4 | +10 |
| Cha | 24 | +7 | +7 |


## Source text

<!-- SRD page 314 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Fiend (Devil), Lawful Evil\
> AC 21 Initiative +14 (24)\
> HP 337 (27d10 + 189)\
> Speed 30 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 26 +8 +8 Dex 14 +2 +8 Con 24 +7 +7\
> Int 22 +6 +6 Wis 18 +4 +10 Cha 24 +7 +7\
> Skills Perception +10, Persuasion +19\
> Resistances Cold\
> Immunities Fire, Poison; Poisoned\
> Senses Truesight 120 ft.; Passive Perception 20\
> Languages Infernal; telepathy 120 ft.\
> CR 20 (XP 25,000; PB +6)


<!-- SRD page 315 -->

### Traits

Diabolical Restoration. If the pit fiend dies outside the Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Nine Hells.

Fear Aura. The pit fiend emanates an aura in a 20-foot Emanation while it doesn’t have the Incapacitated condition. Wisdom Saving Throw: DC 21, any enemy that starts its turn in the aura. Failure: The target has the Frightened condition until the start of its next turn. Success: The target is immune to this pit fiend’s aura for 24 hours.

Legendary Resistance (4/Day). If the pit fiend fails a saving throw, it can choose to succeed instead.

Magic Resistance. The pit fiend has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The pit fiend makes one Bite attack, two Devilish Claw attacks, and one Fiery Mace attack.

Bite. Melee Attack Roll: +14, reach 10 ft. Hit: 18 (3d6 + 8) Piercing damage. If the target is a creature, it must make the following saving throw. Constitution Saving Throw: DC 21. Failure: The target has the Poisoned condition. While Poisoned, the target can’t regain Hit Points and takes 21 (6d6) Poison damage at the start of each of its turns, and it repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

Devilish Claw. Melee Attack Roll: +14, reach 10 ft. Hit: 26 (4d8 + 8) Necrotic damage.

Fiery Mace. Melee Attack Roll: +14, reach 10 ft. Hit: 22 (4d6 + 8) Force damage plus 21 (6d6) Fire damage.

Hellfire Spellcasting (Recharge 4–6). The pit fiend casts Fireball (level 5 version) twice, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 21). It can replace one Fireball with Hold Monster (level 7 version) or Wall of Fire.

Original layout: [[../../srd-5.2.1/pages/314|page 314]], [[../../srd-5.2.1/pages/315|page 315]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/pit_fiend ` |
| ` implementation ` | ` dnd2024:creature_pit_fiend ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold"] ` |
| ` damage_immunities ` | ` ["fire", "poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` passive_perception ` | ` 20 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 10 ` |
| ` persuasion ` | ` 19 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` truesight ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["infernal"] ` |

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` range_ft ` | ` 120 ` |
| ` two_way ` | ` true ` |

#### Features

##### Diabolical restoration

<!-- splat:yaml path=splat_rule/parameters/features/diabolical_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane_not: nine_hells
body: disappears
revive:
  new_body: true
  plane: nine_hells
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

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 14
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 3
        sides: 6
      add: 8
    average: 18
- op: saving_throw
  ability: constitution
  dc: 21
  targets:
    kind: target
    requires_creature: true
  failure:
  - op: condition.apply
    condition: poisoned
    duration:
      amount: 1
      unit: minute
    while_active:
    - op: healing.prohibit
    on_target_turn_start:
    - op: damage
      components:
      - type: poison
        amount:
          dice:
            count: 6
            sides: 6
          add: 0
        average: 21
    repeat_save:
      event: target.turn_end
      ability: constitution
      dc: 21
      success: effect.end
      automatic_success_after:
        amount: 1
        unit: minute
  success: []
```

##### Devilish claw

<!-- splat:yaml path=splat_rule/parameters/features/devilish_claw -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 14
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: necrotic
    amount:
      dice:
        count: 4
        sides: 8
      add: 8
    average: 26
```

##### Fiery mace

<!-- splat:yaml path=splat_rule/parameters/features/fiery_mace -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 14
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: force
    amount:
      dice:
        count: 4
        sides: 6
      add: 8
    average: 22
  - type: fire
    amount:
      dice:
        count: 6
        sides: 6
      add: 0
    average: 21
```

##### Fear aura

<!-- splat:yaml path=splat_rule/parameters/features/fear_aura -->

```yaml
economy: passive
trigger: enemy.turn_start
requires:
  actor_excluded_conditions:
  - incapacitated
effect:
  op: saving_throw
  ability: wisdom
  dc: 21
  targets:
    kind: triggering_enemy
    count: all
    area:
      shape: emanation
      origin: actor
      size_ft: 20
  failure:
  - op: condition.apply
    condition: frightened
    duration:
      until: target.next_turn_start
  success:
  - op: feature.immunity
    source: actor
    feature: fear_aura
    duration:
      amount: 24
      unit: hour
```

##### Hellfire spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/hellfire_spellcasting -->

```yaml
economy: action
op: spell_sequence
ability: charisma
save_dc: 21
waived_components:
- material
base:
- op: spell.cast
  spell:
    key: dnd2024:spells/fireball
  cast_level: 5
  count: 2
replacement:
  maximum: 1
  choices:
  - op: spell.cast
    spell:
      key: dnd2024:spells/hold_monster
    cast_level: 7
  - op: spell.cast
    spell:
      key: dnd2024:spells/wall_of_fire
recharge:
  die_sides: 6
  success_min: 4
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Count:
    path: count
    type: integer
  Feature:
    path: feature
    type: symbol
-->

| Key | Count | Feature |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` Bite ` |
| ` row_2 ` | ` 2 ` | ` Devilish claw ` |
| ` row_3 ` | ` 1 ` | ` Fiery mace ` |

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
>   value: 60
> ```

