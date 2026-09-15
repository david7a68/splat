---
splat_schema: 1
splat_id: f62865d4-766c-54c4-a3a5-db801da34f80
splat_kind: creature
splat_key: dnd2024:creatures/night_hag
splat_name: Night Hag
splat_origin:
  key: dnd2024:reference/creature/night_hag
splat_sources:
- title: SRD 5.2.1
  locator: pp. 311
  url: https://www.dndbeyond.com/srd
---

# Night Hag

## Source text

<!-- SRD page 311 -->

Medium Fiend, Neutral Evil\
AC 17 Initiative +5 (15)\
HP 112 (15d8 + 45)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 16 +3 +3 Wis 14 +2 +2 Cha 16 +3 +3\
Skills Deception +6, Insight +5, Perception +5, Stealth +5\
Resistances Cold, Fire\
Immunities Charmed\
Senses Darkvision 120 ft.; Passive Perception 15\
Languages Abyssal, Common, Infernal, Primordial\
CR 5 (XP 1,800; PB +3)\

### Traits

Coven Magic. While within 30 feet of at least two hag\
allies, the hag can cast one of the following spells,\
requiring no Material components, using the spell’s\
normal casting time, and using Intelligence as the spell-\
casting ability (spell save DC 14): Augury, Find Familiar,\
Identify, Locate Object, Scrying, or Unseen Servant.\
The hag must finish a Long Rest before using this trait to\
cast that spell again.\
Magic Resistance. The hag has Advantage on saving\
throws against spells and other magical effects.\
Soul Bag. The hag has a soul bag. While holding or\
carrying the bag, the hag can use its Nightmare Haunt-\
ing action.\
The bag has AC 15, HP 20, and Resistance to all dam-\
age. The bag turns to dust if reduced to 0 Hit Points. If\
the bag is destroyed, any souls the bag is holding are\
released. The hag can create a new bag after 7 days.\

### Actions

Multiattack. The hag makes two Claw attacks.\
Claw. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8\
+ 4) Slashing damage.\
Nightmare Haunting (1/Day; Requires Soul Bag).\
While on the Ethereal Plane, the hag casts Dream, us-\
ing the same spellcasting ability as Spellcasting. Only\
the hag can serve as the spell’s messenger, and the tar-\
get must be a creature the hag can see on the Material\
Plane. The spell fails and is wasted if the target is under\
the effect of the Protection from Evil and Good spell or\
within a Magic Circle spell.\
If the target takes damage from the Dream spell, the\
target’s Hit Point maximum decreases by an amount\
equal to that damage. If the spell kills the target, its soul\
is trapped in the hag’s soul bag, and the target can’t be\
raised from the dead until its soul is released.\
Spellcasting. The hag casts one of the following spells,\
requiring no Material components and using Intelli-\
gence as the spellcasting ability (spell save DC 14):\
At Will: Detect Magic, Etherealness, Magic Missile\
(level 4 version)\
2/Day Each: Phantasmal Killer, Plane Shift (self only)\

### Bonus Actions

Shape-Shift. The hag shape-shifts into a Small or Me-\
dium Humanoid, or it returns to its true form. Other\
than its size, its game statistics are the same in each\
form. Any equipment it is wearing or carrying isn’t\
transformed.\

Original layout: [[../../srd-5.2.1/pages/311|page 311]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Deception +6, Insight +5, Perception +5, Stealth +5 ` |
| ` resistances_text ` | ` Cold, Fire ` |
| ` immunities_text ` | ` Charmed ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` Abyssal, Common, Infernal, Primordial ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 112 (15d8 + 45) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 112 ` |  |
| ` initiative ` | ` 5 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/night_hag ` |
| ` implementation ` | ` dnd2024:creature_night_hag ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire"] ` |
| ` condition_immunities ` | ` ["charmed"] ` |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 6 ` |
| ` insight ` | ` 5 ` |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 5 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "infernal", "abyssal", "primordial"] ` |

#### Features

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
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
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
op: spellcasting
profile:
  ability: intelligence
  waived_components:
  - material
  spells:
  - spell:
      key: dnd2024:spells/detect_magic
    uses:
      unlimited: true
  - spell:
      key: dnd2024:spells/etherealness
    uses:
      unlimited: true
  - spell:
      key: dnd2024:spells/magic_missile
    uses:
      unlimited: true
    cast_level: 4
  - spell:
      key: dnd2024:spells/phantasmal_killer
    uses:
      count: 2
      recovery: long_rest
  - spell:
      key: dnd2024:spells/plane_shift
    uses:
      count: 2
      recovery: long_rest
    target_override: self
  save_dc: 14
```

##### Coven magic

<!-- splat:yaml path=splat_rule/parameters/features/coven_magic -->

```yaml
economy: passive
op: spellcasting
requires:
  hag_allies_within_ft: 30
  minimum_allies: 2
ability: intelligence
save_dc: 14
waived_components:
- material
casting_time: spell_normal
spells:
- spell:
    key: dnd2024:spells/augury
  uses:
    count: 1
    recovery: long_rest
- spell:
    key: dnd2024:spells/find_familiar
  uses:
    count: 1
    recovery: long_rest
- spell:
    key: dnd2024:spells/identify
  uses:
    count: 1
    recovery: long_rest
- spell:
    key: dnd2024:spells/locate_object
  uses:
    count: 1
    recovery: long_rest
- spell:
    key: dnd2024:spells/scrying
  uses:
    count: 1
    recovery: long_rest
- spell:
    key: dnd2024:spells/unseen_servant
  uses:
    count: 1
    recovery: long_rest
```

##### Soul bag

<!-- splat:yaml path=splat_rule/parameters/features/soul_bag -->

```yaml
economy: passive
op: equipment.special
item: soul_bag
ac: 15
hp: 20
damage_resistances:
- all
grants_feature: nightmare_haunting
requires_holding_or_carrying: true
destroy_at_hp: 0
on_destroy:
  release_all_souls: true
create_replacement_after:
  amount: 7
  unit: day
```

##### Nightmare haunting

<!-- splat:yaml path=splat_rule/parameters/features/nightmare_haunting -->

```yaml
economy: action
op: spell.cast
spell:
  key: dnd2024:spells/dream
profile: spellcasting
requires:
  actor_plane: ethereal
  holding_or_carrying_item: soul_bag
target:
  kind: creature
  count: 1
  visible_to_actor: true
  plane: material
messenger: actor_only
fails_and_wasted_if_target:
- spell:
    key: dnd2024:spells/protection_from_evil_and_good
- within_spell:
    key: dnd2024:spells/magic_circle
on_damage:
  op: hp_maximum.reduce
  amount:
    event_field: dream_damage_taken
on_kill:
  op: soul.trap
  container: soul_bag
  cannot_raise_until_soul_released: true
uses:
  count: 1
  recovery: long_rest
```

##### Shape shift

<!-- splat:yaml path=splat_rule/parameters/features/shape_shift -->

```yaml
economy: bonus_action
op: shapechange
equipment_transformed: false
statistics_preserved_except:
- size
choices:
- form: humanoid
  sizes:
  - small
  - medium
- form: true_form
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: claw
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
