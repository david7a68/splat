---
splat_schema: 2
splat_id: 9aba72ea-1243-5435-b57d-530d93f4bf5b
splat_kind: creature
splat_key: dnd2024:creatures/gray_ooze
splat_name: Gray Ooze
splat_origin: dnd2024:reference/creature/gray_ooze
splat_sources:
- SRD 5.2.1, pp. 293, https://www.dndbeyond.com/srd
---

# Gray Ooze


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Ooze |
| Alignment | Unaligned |
| Armor Class | 9 |
| Hit Points | 22 (3d8 + 9) |
| Initiative | -2 (8) |
| Speed | 10 ft., Climb 10 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Stealth +2 |
| Senses | Blindsight 60 ft.; Passive Perception 8 |
| Languages | None |
| Resistances | Acid, Cold, Fire |
| Immunities | Blinded, Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 12 | +1 | +1 |
| Dex | 6 | -2 | -2 |
| Con | 16 | +3 | +3 |
| Int | 1 | -5 | -5 |
| Wis | 6 | -2 | -2 |
| Cha | 2 | -4 | -4 |


## Source text

<!-- SRD page 293 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Ooze, Unaligned\
> AC 9 Initiative −2 (13)\
> HP 22 (3d8 + 9)\
> Speed 10 ft., Climb 10 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 12 +1 +1 Dex 6 −2 −2 Con 16 +3 +3\
> Int 1 −5 −5 Wis 6 −2 −2 Cha 2 −4 −4\
> Skills Stealth +2\
> Resistances Acid, Cold, Fire\
> Immunities Blinded, Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained\
> Senses Blindsight 60 ft.; Passive Perception 8\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Traits

Amorphous. The ooze can move through a space as narrow as 1 inch without expending extra movement to do so.

Corrosive Form. Nonmagical ammunition is destroyed immediately after hitting the ooze and dealing any damage. Any nonmagical weapon takes a cumulative −1 penalty to attack rolls immediately after dealing damage to the ooze and coming into contact with it. The weapon is destroyed if the penalty reaches −5. The penalty can be removed by casting the Mending spell on the weapon.

The ooze can eat through 2-inch-thick, nonmagical metal or wood in 1 round.

### Actions

Pseudopod. Melee Attack Roll: +3, reach 5 ft. Hit: 10 (2d8 + 1) Acid damage. Nonmagical armor worn by the target takes a −1 penalty to the AC it offers. The armor is destroyed if the penalty reduces its AC to 10. The penalty can be removed by casting the Mending spell on the armor.

Original layout: [[../../srd-5.2.1/pages/293|page 293]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/gray_ooze ` |
| ` implementation ` | ` dnd2024:creature_gray_ooze ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["acid", "cold", "fire"] ` |
| ` condition_immunities ` | ` ["blinded", "charmed", "deafened", "exhaustion", "frightened", "grappled", "prone", "restrained"] ` |
| ` passive_perception ` | ` 8 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 2 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Amorphous

<!-- splat:values path=splat_rule/parameters/features/amorphous -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.form ` |
| ` minimum_opening_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |

##### Pseudopod

<!-- splat:yaml path=splat_rule/parameters/features/pseudopod -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: acid
    amount:
      dice:
        count: 2
        sides: 8
      add: 1
    average: 10
- op: item.ac_penalty
  target:
    category: armor
    worn_by: target
    magical: false
  add: -1
  cumulative: true
  destroy_at_ac: 10
  remove_penalty_spell:
    key: dnd2024:spells/mending
```

##### Corrosive form

<!-- splat:yaml path=splat_rule/parameters/features/corrosive_form -->

```yaml
economy: passive
ammunition:
  trigger: attack.resolved
  op: item.destroy
  when:
    item_category: ammunition
    item_magical: false
    hit_actor: true
    damage_dealt_to_actor_above: 0
weapon:
  trigger: damage.applied
  op: item.attack_penalty
  add: -1
  cumulative: true
  destroy_at_penalty: -5
  remove_penalty_spell:
    key: dnd2024:spells/mending
  when:
    item_category: weapon
    item_magical: false
    damage_dealt_to_actor_above: 0
    item_contacts_actor: true
eat_material:
  materials:
  - nonmagical_metal
  - nonmagical_wood
  thickness_inches: 2
  duration:
    amount: 1
    unit: round
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
> climb_speed:
>   unit: ft
>   value: 10
> ```

