---
splat_schema: 1
splat_id: 50248aa3-89ce-55b8-8c50-74dfde4d4220
splat_kind: creature
splat_key: dnd2024:creatures/lemure
splat_name: Lemure
splat_origin:
  key: dnd2024:reference/creature/lemure
splat_sources:
- title: SRD 5.2.1
  locator: pp. 304
  url: https://www.dndbeyond.com/srd
---

# Lemure

## Source text

<!-- SRD page 304 -->

Medium Fiend (Devil), Lawful Evil\
AC 9 Initiative −3 (7)\
HP 9 (2d8)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 5 −3 −3 Con 11 +0 +0\
Int 1 −5 −5 Wis 11 +0 +0 Cha 3 −4 −4\
Resistances Cold\
Immunities Fire, Poison; Charmed, Frightened, Poisoned\
Senses Darkvision 120 ft. (unimpeded by magical\
Darkness); Passive Perception 10\
Languages Understands Infernal but can’t speak\
CR 0 (XP 10; PB +2)\

### Traits

Hellish Restoration. If the lemure dies in the Nine\
Hells, it revives with all its Hit Points in 1d10 days un-\
less it is killed by a creature under the effects of a Bless\
spell or its remains are sprinkled with Holy Water.\

### Actions

Vile Slime. Melee Attack Roll: +2, reach 5 ft. Hit: 2\
(1d4) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/304|page 304]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Devil) ` |
| ` movement_text ` | ` 20 ft. ` |
| ` resistances_text ` | ` Cold ` |
| ` immunities_text ` | ` Fire, Poison; Charmed, Frightened, Poisoned ` |
| ` senses_text ` | ` Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 10 ` |
| ` languages_text ` | ` Understands Infernal but can’t speak ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 9 (2d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 5 ` | ` -3 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 9 ` |  |
| ` max_hp ` | ` 9 ` |  |
| ` initiative ` | ` -3 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/lemure ` |
| ` implementation ` | ` dnd2024:creature_lemure ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold"] ` |
| ` damage_immunities ` | ` ["fire", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "frightened", "poisoned"] ` |
| ` passive_perception ` | ` 10 ` |
| ` darkvision_ignores_magical_darkness ` | ` true ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` false ` |
| ` known ` | ` ["infernal"] ` |

#### Features

##### Vile slime

<!-- splat:yaml path=splat_rule/parameters/features/vile_slime -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 2
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: poison
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    average: 2
```

##### Hellish restoration

<!-- splat:yaml path=splat_rule/parameters/features/hellish_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane: nine_hells
revive:
  delay:
    amount:
      dice:
        count: 1
        sides: 10
      add: 0
    unit: day
  hp: maximum
prevented_by:
- killer_has_spell:
    key: dnd2024:spells/bless
- remains_sprinkled_with: holy_water
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
