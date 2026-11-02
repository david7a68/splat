---
splat_schema: 1
splat_id: 447f69de-fddc-5113-8938-50e6f1ebe886
splat_kind: creature
splat_key: dnd2024:creatures/draconic_spirit
splat_name: Draconic Spirit
splat_origin:
  key: dnd2024:reference/creature/draconic_spirit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 166
  url: https://www.dndbeyond.com/srd
---

# Draconic Spirit

## Source text

<!-- SRD page 166 -->

Large Dragon, Neutral\
AC 14 + the spell’s level\
HP 50 + 10 for each spell level above 5\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +2 Con 17 +3 +3\
Int 10 +0 +0 Wis 14 +2 +2 Cha 14 +2 +2\
Resistances Acid, Cold, Fire, Lightning, Poison\
Immunities Charmed, Frightened, Poisoned\
Senses Blindsight 30 ft., Darkvision 60 ft.; Passive Perception 12\
Languages Draconic, understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)

### Traits

Shared Resistances. When you summon the spirit, choose one of its Resistances. You have Resistance to the chosen damage type until the spell ends.

### Actions

Multiattack. The spirit makes a number of Rend attacks equal to half the spell’s level (round down), and it uses Breath Weapon.

Rend. Melee Attack Roll: Bonus equals your spell attack modifier, reach 10 feet. Hit: 1d6 + 4 + the spell’s level Piercing damage.

Breath Weapon. Dexterity Saving Throw: DC equals your spell save DC, each creature in a 30-foot Cone. Failure: 2d6 damage of a type this spirit has Resistance to (your choice when you cast the spell). Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/166|page 166]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 30 ft., Fly 60 ft., Swim 30 ft. ` |
| ` resistances_text ` | ` Acid, Cold, Fire, Lightning, Poison ` |
| ` immunities_text ` | ` Charmed, Frightened, Poisoned ` |
| ` senses_text ` | ` Blindsight 30 ft., Darkvision 60 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Draconic, understands the languages you know ` |
| ` cr_text ` | ` None (XP 0; PB equals your Proficiency Bonus) ` |
| ` hit_points_text ` | ` 50 + 10 for each spell level above 5 ` |
| ` armor_class_text ` | ` 14 + the spell’s level ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 14 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |
| ` swim_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/draconic_spirit ` |
| ` implementation ` | ` dnd2024:creature_draconic_spirit ` |

### Parameters

#### Features

##### Shared resistances

<!-- splat:yaml path=splat_rule/parameters/features/shared_resistances -->

```yaml
economy: passive
trigger: summon.created
choice:
  chooser: summoner
  options:
  - acid
  - cold
  - fire
  - lightning
  - poison
  count: 1
grant_to: summoner
resistance: chosen_type
ends:
  event: summoning_spell.end
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus:
  input: summoner_spell_attack_modifier
reach_feet: 10
targets:
  count: 1
hit:
- op: damage
  damage_type: piercing
  amount:
    add:
    - dice:
        count: 1
        sides: 6
      add: 4
    - input: spell_level
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: rend
  count:
    floor_divide:
    - input: spell_level
    - 2
- op: use_feature
  feature: breath_weapon
  count: 1
```

##### Breath weapon

<!-- splat:yaml path=splat_rule/parameters/features/breath_weapon -->

```yaml
economy: action
damage_type_choice:
  chooser: summoner
  timing: spell.cast
  options:
  - acid
  - cold
  - fire
  - lightning
  - poison
op: save
ability: dexterity
dc:
  input: summoner_spell_save_dc
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: damage
  damage_type:
    choice: breath_damage_type
  amount:
    add:
    - dice:
        count: 2
        sides: 6
      add: 0
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["acid", "cold", "fire", "lightning", "poison"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["charmed", "frightened", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

##### Special

<!-- splat:table
path: splat_rule/parameters/senses/special
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Sense:
    path: sense
    type: symbol
  Range feet:
    path: range_feet
    type: integer
-->

| Key | Sense | Range feet |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 30 ` |
| ` row_2 ` | ` Darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["draconic"] ` |

##### Understands

<!-- splat:yaml path=splat_rule/parameters/communication/understands -->

```yaml
union:
- - draconic
- languages_known_by: summoner
```

#### Proficiency bonus

<!-- splat:values path=splat_rule/parameters/proficiency_bonus -->

| Field | Value |
| --- | --- |
| ` input ` | ` summoner_proficiency_bonus ` |

#### Scaling stats

<!-- splat:yaml path=splat_rule/parameters/scaling_stats -->

```yaml
ac:
  add:
  - 14
  - input: spell_level
hit_points:
  add:
  - 50
  - multiply:
    - 10
    - subtract:
      - input: spell_level
      - 5
```

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
