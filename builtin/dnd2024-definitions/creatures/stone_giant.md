---
splat_schema: 1
splat_id: a00af030-27f8-5d34-99f9-5b74dc3ce8e9
splat_kind: creature
splat_key: dnd2024:creatures/stone_giant
splat_name: Stone Giant
splat_origin:
  key: dnd2024:reference/creature/stone_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 330
  url: https://www.dndbeyond.com/srd
---

# Stone Giant

## Source text

<!-- SRD page 330 -->

Huge Giant, Neutral\
AC 17 Initiative +5 (15)\
HP 126 (11d12 + 55)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 15 +2 +5 Con 20 +5 +8\
Int 10 +0 +0 Wis 12 +1 +4 Cha 9 −1 −1\
Skills Athletics +12, Perception +4, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Giant\
CR 7 (XP 2,900; PB +3)

### Actions

Multiattack. The giant makes two attacks, using Stone Club or Boulder in any combination.

Stone Club. Melee Attack Roll: +9, reach 15 ft. Hit: 22 (3d10 + 6) Bludgeoning damage.

Boulder. Ranged Attack Roll: +9, range 60/240 ft. Hit: 15 (2d8 + 6) Bludgeoning damage. If the target is a Large or smaller creature, it has the Prone condition.

### Reactions

Deflect Missile (Recharge 5–6). Trigger: The giant is hit by a ranged attack roll and takes Bludgeoning, Piercing, or Slashing damage from it. Response: The giant reduces the damage it takes from the attack by 11 (1d10 + 6), and if that damage is reduced to 0, the giant can redirect some of the attack’s force. Dexterity Saving Throw: DC 17, one creature the giant can see within 60 feet. Failure: 11 (1d10 + 6) Force damage.

Original layout: [[../../srd-5.2.1/pages/330|page 330]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` giant ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Athletics +12, Perception +4, Stealth +5 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Giant ` |
| ` cr_text ` | ` 7 (XP 2,900; PB +3) ` |
| ` hit_points_text ` | ` 126 (11d12 + 55) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 23 ` | ` 6 ` |
| ` Dexterity ` | ` 15 ` | ` 5 ` |
| ` Constitution ` | ` 20 ` | ` 8 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 12 ` | ` 4 ` |
| ` Charisma ` | ` 9 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 126 ` |  |
| ` initiative ` | ` 5 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/stone_giant ` |
| ` implementation ` | ` dnd2024:creature_stone_giant ` |

### Parameters

#### Features

##### Stone club

<!-- splat:yaml path=splat_rule/parameters/features/stone_club -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 9
targets:
  count: 1
hit:
- op: damage
  average: 22
  amount:
    dice:
      count: 3
      sides: 10
    add: 6
  damage_type: bludgeoning
reach_feet: 15
```

##### Boulder

<!-- splat:yaml path=splat_rule/parameters/features/boulder -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 9
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
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: prone
range_feet: 60
long_range_feet: 240
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 2 ` |

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
| ` row_1 ` | ` Stone club ` |
| ` row_2 ` | ` Boulder ` |

##### Deflect missile

<!-- splat:yaml path=splat_rule/parameters/features/deflect_missile -->

```yaml
economy: reaction
resource:
  capacity: 1
  initial: 1
  recharge:
    event: turn.start
    participant: self
    roll:
      dice:
        count: 1
        sides: 6
      add: 0
    succeeds_on:
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
trigger: ranged_attack.hits_self_and_deals_damage
requires:
  damage_type_any:
  - bludgeoning
  - piercing
  - slashing
effects:
- op: reduce_damage
  average: 11
  amount:
    dice:
      count: 1
      sides: 10
    add: 6
- op: if
  test:
    remaining_attack_damage: 0
  then:
  - op: optional
    effect:
      op: save
      ability: dexterity
      dc: 17
      targets:
        kind: creature
        count: 1
        range_feet: 60
        visible_to: self
      failure:
      - op: damage
        average: 11
        amount:
          dice:
            count: 1
            sides: 10
          add: 6
        damage_type: force
      success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 12 ` |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["giant"] ` |
| ` understands ` | ` ["giant"] ` |

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
