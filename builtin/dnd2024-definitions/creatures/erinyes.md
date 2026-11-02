---
splat_schema: 1
splat_id: 9d266575-5494-5489-8e63-5a74a106373f
splat_kind: creature
splat_key: dnd2024:creatures/erinyes
splat_name: Erinyes
splat_origin:
  key: dnd2024:reference/creature/erinyes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 283
  url: https://www.dndbeyond.com/srd
---

# Erinyes

## Source text

<!-- SRD page 283 -->

Medium Fiend (Devil), Lawful Evil\
AC 18 Initiative +7 (17)\
HP 178 (21d8 + 84)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 16 +3 +7 Con 18 +4 +8\
Int 14 +2 +2 Wis 14 +2 +2 Cha 18 +4 +8\
Skills Perception +6, Persuasion +8\
Resistances Cold\
Immunities Fire, Poison; Poisoned\
Senses Truesight 120 ft.; Passive Perception 16\
Languages Infernal; telepathy 120 ft.\
CR 12 (XP 8,400; PB +4)

### Traits

Diabolical Restoration. If the erinyes dies outside the Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Nine Hells.

Magic Resistance. The erinyes has Advantage on saving throws against spells and other magical effects.

Magic Rope. The erinyes has a magic rope. While bearing it, the erinyes can use the Entangling Rope action. The rope has AC 20, HP 90, and Immunity to Poison and Psychic damage. The rope turns to dust if reduced to 0 Hit Points, if it is 5+ feet away from the erinyes for 1 hour or more, or if the erinyes dies. If the rope is damaged or destroyed, the erinyes can fully restore it when finishing a Short or Long Rest.

### Actions

Multiattack. The erinyes makes three Withering Sword attacks and can use Entangling Rope.

Withering Sword. Melee Attack Roll: +8, reach 5 ft. Hit: 13 (2d8 + 4) Slashing damage plus 11 (2d10) Necrotic damage.

Entangling Rope (Requires Magic Rope). Strength Saving Throw: DC 16, one creature the erinyes can see within 120 feet. Failure: 14 (4d6) Force damage, and the target has the Restrained condition until the rope is destroyed, the erinyes uses a Bonus Action to release the target, or the erinyes uses Entangling Rope again.

### Reactions

Parry. Trigger: The erinyes is hit by a melee attack roll while holding a weapon. Response: The erinyes adds 4 to its AC against that attack, possibly causing it to miss.

Original layout: [[../../srd-5.2.1/pages/283|page 283]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Devil) ` |
| ` movement_text ` | ` 30 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Perception +6, Persuasion +8 ` |
| ` resistances_text ` | ` Cold ` |
| ` immunities_text ` | ` Fire, Poison; Poisoned ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 16 ` |
| ` languages_text ` | ` Infernal; telepathy 120 ft. ` |
| ` cr_text ` | ` 12 (XP 8,400; PB +4) ` |
| ` hit_points_text ` | ` 178 (21d8 + 84) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 16 ` | ` 7 ` |
| ` Constitution ` | ` 18 ` | ` 8 ` |
| ` Intelligence ` | ` 14 ` | ` 2 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 18 ` | ` 8 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 178 ` |  |
| ` initiative ` | ` 7 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/erinyes ` |
| ` implementation ` | ` dnd2024:creature_erinyes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold"] ` |
| ` damage_immunities ` | ` ["fire", "poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` passive_perception ` | ` 16 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |
| ` persuasion ` | ` 8 ` |

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

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Withering sword

<!-- splat:yaml path=splat_rule/parameters/features/withering_sword -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 8
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
        sides: 8
      add: 4
    average: 13
  - type: necrotic
    amount:
      dice:
        count: 2
        sides: 10
      add: 0
    average: 11
```

##### Parry

<!-- splat:yaml path=splat_rule/parameters/features/parry -->

```yaml
economy: reaction
trigger: melee_attack.hit_actor
requires:
  holding_weapon: true
op: ac.add
amount: 4
scope: triggering_attack
recheck_hit: true
```

##### Magic rope

<!-- splat:values path=splat_rule/parameters/features/magic_rope -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` equipment.special ` |
| ` item ` | ` magic_rope ` |
| ` ac ` | ` 20 ` |
| ` hp ` | ` 90 ` |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` grants_feature ` | ` entangling_rope ` |
| ` requires_bearing ` | ` true ` |
| ` restore_fully_after ` | ` ["short_rest", "long_rest"] ` |

###### Destroy when

<!-- splat:table
path: splat_rule/parameters/features/magic_rope/destroy_when
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Hp at most:
    path: hp_at_most
    type: integer
    optional: true
  Distance from actor ft at least:
    path: distance_from_actor_ft_at_least
    type: integer
    optional: true
  Duration at least amount:
    path: duration_at_least/amount
    type: integer
    optional: true
  Duration at least unit:
    path: duration_at_least/unit
    type: symbol
    optional: true
  Actor dead:
    path: actor_dead
    type: boolean
    optional: true
-->

| Key | Hp at most | Distance from actor ft at least | Duration at least amount | Duration at least unit | Actor dead |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 0 ` |  |  |  |  |
| ` row_2 ` |  | ` 5 ` | ` 1 ` | ` Hour ` |  |
| ` row_3 ` |  |  |  |  | true |

##### Entangling rope

<!-- splat:yaml path=splat_rule/parameters/features/entangling_rope -->

```yaml
economy: action
op: saving_throw
ability: strength
dc: 16
targets:
  kind: creature
  count: 1
  range_ft: 120
  visible_to_actor: true
failure:
- op: damage
  components:
  - type: force
    amount:
      dice:
        count: 4
        sides: 6
      add: 0
    average: 14
- op: condition.apply
  condition: restrained
  duration:
    until_any:
    - rope.destroyed
    - actor.bonus_action_release
    - actor.uses_entangling_rope_again
success: []
requires_item: magic_rope
release_economy: bonus_action
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
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
  Count:
    path: count
    type: integer
  Optional:
    path: optional
    type: boolean
    optional: true
-->

| Key | Feature | Count | Optional |
| --- | --- | --- | --- |
| ` row_1 ` | ` Withering sword ` | ` 3 ` |  |
| ` row_2 ` | ` Entangling rope ` | ` 1 ` | true |

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
