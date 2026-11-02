---
splat_schema: 1
splat_id: a33a2ae8-8473-5e89-90b2-20d2665bb846
splat_kind: creature
splat_key: dnd2024:creatures/ice_devil
splat_name: Ice Devil
splat_origin:
  key: dnd2024:reference/creature/ice_devil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 300
  url: https://www.dndbeyond.com/srd
---

# Ice Devil

## Source text

<!-- SRD page 300 -->

Large Fiend (Devil), Lawful Evil\
AC 18 Initiative +7 (17)\
HP 228 (24d10 + 96)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 14 +2 +7 Con 18 +4 +9\
Int 18 +4 +4 Wis 15 +2 +7 Cha 18 +4 +9\
Skills Insight +7, Perception +7, Persuasion +9\
Immunities Cold, Fire, Poison; Poisoned\
Senses Blindsight 120 ft.; Passive Perception 17\
Languages Infernal; telepathy 120 ft.\
CR 14 (XP 11,500; PB +5)

### Traits

Diabolical Restoration. If the devil dies outside the Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Nine Hells.

Magic Resistance. The devil has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The devil makes three Ice Spear attacks. It can replace one attack with a Tail attack.

Ice Spear. Melee or Ranged Attack Roll: +10, reach 5 ft. or range 30/120 ft. Hit: 14 (2d8 + 5) Piercing damage plus 10 (3d6) Cold damage. Until the end of its next turn, the target can’t take a Bonus Action or Reaction, its Speed decreases by 10 feet, and it can move or take one action on its turn, not both. Hit or Miss: The spear magically returns to the devil’s hand immediately after a ranged attack.

Tail. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (3d6 + 5) Bludgeoning damage plus 18 (4d8) Cold damage.

Ice Wall (Recharge 6). The devil casts Wall of Ice (level 8 version), requiring no spell components and using Intelligence as the spellcasting ability (spell save DC 17).

Original layout: [[../../srd-5.2.1/pages/300|page 300]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Devil) ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Insight +7, Perception +7, Persuasion +9 ` |
| ` immunities_text ` | ` Cold, Fire, Poison; Poisoned ` |
| ` senses_text ` | ` Blindsight 120 ft.; Passive Perception 17 ` |
| ` languages_text ` | ` Infernal; telepathy 120 ft. ` |
| ` cr_text ` | ` 14 (XP 11,500; PB +5) ` |
| ` hit_points_text ` | ` 228 (24d10 + 96) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 21 ` | ` 5 ` |
| ` Dexterity ` | ` 14 ` | ` 7 ` |
| ` Constitution ` | ` 18 ` | ` 9 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 15 ` | ` 7 ` |
| ` Charisma ` | ` 18 ` | ` 9 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 228 ` |  |
| ` initiative ` | ` 7 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ice_devil ` |
| ` implementation ` | ` dnd2024:creature_ice_devil ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["cold", "fire", "poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` passive_perception ` | ` 17 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 7 ` |
| ` perception ` | ` 7 ` |
| ` persuasion ` | ` 9 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 120 ` |

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

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Ice spear

<!-- splat:yaml path=splat_rule/parameters/features/ice_spear -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 10
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
range_ft:
  normal: 30
  long: 120
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 8
      add: 5
    average: 14
  - type: cold
    amount:
      dice:
        count: 3
        sides: 6
      add: 0
    average: 10
- op: effect.apply
  duration:
    until: target.next_turn_end
  rules:
    bonus_actions_allowed: false
    reactions_allowed: false
    speed_add_ft: -10
    turn_economy_choice:
    - movement
    - one_action
    both_allowed: false
after_attack:
  when: ranged_attack_hit_or_miss
  op: item.return
  destination: actor_hand
  magical: true
```

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 10
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 6
      add: 5
    average: 15
  - type: cold
    amount:
      dice:
        count: 4
        sides: 8
      add: 0
    average: 18
```

##### Ice wall

<!-- splat:yaml path=splat_rule/parameters/features/ice_wall -->

```yaml
economy: action
op: spellcasting
profile:
  ability: intelligence
  waived_components:
  - verbal
  - somatic
  - material
  save_dc: 17
choose_count: 1
choices:
- op: spell.cast
  spell:
    key: dnd2024:spells/wall_of_ice
  cast_level: 8
recharge:
  die_sides: 6
  success_min: 6
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 3
  feature: ice_spear
replace_attack:
  maximum: 1
  choices:
  - op: feature.use
    feature: tail
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
