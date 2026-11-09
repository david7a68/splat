---
splat_schema: 2
splat_id: f62865d4-766c-54c4-a3a5-db801da34f80
splat_kind: creature
splat_key: dnd2024:creatures/night_hag
splat_name: Night Hag
splat_origin: dnd2024:reference/creature/night_hag
splat_sources:
- SRD 5.2.1, pp. 311, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 225, Habitat
---

# Night Hag


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fiend |
| Alignment | Neutral Evil |
| Armor Class | 17 |
| Hit Points | 112 (15d8 + 45) |
| Initiative | +5 (15) |
| Speed | 30 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Deception +6, Insight +5, Perception +5, Stealth +5 |
| Senses | Darkvision 120 ft.; Passive Perception 15 |
| Languages | Abyssal, Common, Infernal, Primordial |
| Resistances | Cold, Fire |
| Immunities | Charmed |
| Habitat | Planar (Lower Planes) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 15 | +2 | +2 |
| Con | 16 | +3 | +3 |
| Int | 16 | +3 | +3 |
| Wis | 14 | +2 | +2 |
| Cha | 16 | +3 | +3 |


## Source text

<!-- SRD page 311 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fiend, Neutral Evil\
> AC 17 Initiative +5 (15)\
> HP 112 (15d8 + 45)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
> Int 16 +3 +3 Wis 14 +2 +2 Cha 16 +3 +3\
> Skills Deception +6, Insight +5, Perception +5, Stealth +5\
> Resistances Cold, Fire\
> Immunities Charmed\
> Senses Darkvision 120 ft.; Passive Perception 15\
> Languages Abyssal, Common, Infernal, Primordial\
> CR 5 (XP 1,800; PB +3)


### Traits

Coven Magic. While within 30 feet of at least two hag allies, the hag can cast one of the following spells, requiring no Material components, using the spell’s normal casting time, and using Intelligence as the spellcasting ability (spell save DC 14): Augury, Find Familiar, Identify, Locate Object, Scrying, or Unseen Servant. The hag must finish a Long Rest before using this trait to cast that spell again.

Magic Resistance. The hag has Advantage on saving throws against spells and other magical effects.

Soul Bag. The hag has a soul bag. While holding or carrying the bag, the hag can use its Nightmare Haunting action.

The bag has AC 15, HP 20, and Resistance to all damage. The bag turns to dust if reduced to 0 Hit Points. If the bag is destroyed, any souls the bag is holding are released. The hag can create a new bag after 7 days.

### Actions

Multiattack. The hag makes two Claw attacks.

Claw. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 + 4) Slashing damage.

Nightmare Haunting (1/Day; Requires Soul Bag). While on the Ethereal Plane, the hag casts Dream, using the same spellcasting ability as Spellcasting. Only the hag can serve as the spell’s messenger, and the target must be a creature the hag can see on the Material Plane. The spell fails and is wasted if the target is under the effect of the Protection from Evil and Good spell or within a Magic Circle spell.

If the target takes damage from the Dream spell, the target’s Hit Point maximum decreases by an amount equal to that damage. If the spell kills the target, its soul is trapped in the hag’s soul bag, and the target can’t be raised from the dead until its soul is released.

Spellcasting. The hag casts one of the following spells, requiring no Material components and using Intelligence as the spellcasting ability (spell save DC 14):

At Will: Detect Magic, Etherealness, Magic Missile (level 4 version)\
2/Day Each: Phantasmal Killer, Plane Shift (self only)

### Bonus Actions

Shape-Shift. The hag shape-shifts into a Small or Medium Humanoid, or it returns to its true form. Other than its size, its game statistics are the same in each form. Any equipment it is wearing or carrying isn’t transformed.

Original layout: [[../../srd-5.2.1/pages/311|page 311]].







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

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` spellcasting ` |

###### Profile

<!-- splat:values path=splat_rule/parameters/features/spellcasting/profile -->

| Field | Value |
| --- | --- |
| ` ability ` | ` intelligence ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 14 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_magic
- etherealness
- magic_missile
- phantasmal_killer
- plane_shift
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
    optional: true
  Cast level:
    path: cast_level
    type: integer
    optional: true
  Uses count:
    path: uses/count
    type: integer
    optional: true
  Uses recovery:
    path: uses/recovery
    type: symbol
    optional: true
  Target override:
    path: target_override
    type: symbol
    optional: true
-->

| Spell | Uses unlimited | Cast level | Uses count | Uses recovery | Target override |
| --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |  |  |
| [Etherealness](<../spells/etherealness.md> "dnd2024:spells/etherealness") | true |  |  |  |  |
| [Magic Missile](<../spells/magic_missile.md> "dnd2024:spells/magic_missile") | true | ` 4 ` |  |  |  |
| [Phantasmal Killer](<../spells/phantasmal_killer.md> "dnd2024:spells/phantasmal_killer") |  |  | ` 2 ` | ` Long rest ` |  |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") |  |  | ` 2 ` | ` Long rest ` | ` Self ` |

##### Coven magic

<!-- splat:values path=splat_rule/parameters/features/coven_magic -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` spellcasting ` |
| ` ability ` | ` intelligence ` |
| ` save_dc ` | ` 14 ` |
| ` waived_components ` | ` ["material"] ` |
| ` casting_time ` | ` spell_normal ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/coven_magic/requires -->

| Field | Value |
| --- | --- |
| ` hag_allies_within_ft ` | ` 30 ` |
| ` minimum_allies ` | ` 2 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/coven_magic/spells
key: Spell
order:
- augury
- find_familiar
- identify
- locate_object
- scrying
- unseen_servant
columns:
  Spell:
    path: spell
    type: reference
  Uses count:
    path: uses/count
    type: integer
  Uses recovery:
    path: uses/recovery
    type: symbol
-->

| Spell | Uses count | Uses recovery |
| --- | --- | --- |
| [Augury](<../spells/augury.md> "dnd2024:spells/augury") | ` 1 ` | ` Long rest ` |
| [Find Familiar](<../spells/find_familiar.md> "dnd2024:spells/find_familiar") | ` 1 ` | ` Long rest ` |
| [Identify](<../spells/identify.md> "dnd2024:spells/identify") | ` 1 ` | ` Long rest ` |
| [Locate Object](<../spells/locate_object.md> "dnd2024:spells/locate_object") | ` 1 ` | ` Long rest ` |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` 1 ` | ` Long rest ` |
| [Unseen Servant](<../spells/unseen_servant.md> "dnd2024:spells/unseen_servant") | ` 1 ` | ` Long rest ` |

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

<!-- splat:values path=splat_rule/parameters/features/nightmare_haunting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` spell.cast ` |
| ` profile ` | ` spellcasting ` |
| ` messenger ` | ` actor_only ` |

###### Spell

<!-- splat:values path=splat_rule/parameters/features/nightmare_haunting/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dream ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/nightmare_haunting/requires -->

| Field | Value |
| --- | --- |
| ` actor_plane ` | ` ethereal ` |
| ` holding_or_carrying_item ` | ` soul_bag ` |

###### Target

<!-- splat:values path=splat_rule/parameters/features/nightmare_haunting/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_actor ` | ` true ` |
| ` plane ` | ` material ` |

###### Fails and wasted if target

<!-- splat:table
path: splat_rule/parameters/features/nightmare_haunting/fails_and_wasted_if_target
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Spell:
    path: spell
    type: reference
    optional: true
  Within spell:
    path: within_spell
    type: reference
    optional: true
-->

| Key | Spell | Within spell |
| --- | --- | --- |
| ` row_1 ` | [Protection from Evil and Good](<../spells/protection_from_evil_and_good.md> "dnd2024:spells/protection_from_evil_and_good") |  |
| ` row_2 ` |  | [Magic Circle](<../spells/magic_circle.md> "dnd2024:spells/magic_circle") |

###### On damage

<!-- splat:yaml path=splat_rule/parameters/features/nightmare_haunting/on_damage -->

```yaml
op: hp_maximum.reduce
amount:
  event_field: dream_damage_taken
```

###### On kill

<!-- splat:values path=splat_rule/parameters/features/nightmare_haunting/on_kill -->

| Field | Value |
| --- | --- |
| ` op ` | ` soul.trap ` |
| ` container ` | ` soul_bag ` |
| ` cannot_raise_until_soul_released ` | ` true ` |

###### Uses

<!-- splat:values path=splat_rule/parameters/features/nightmare_haunting/uses -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` recovery ` | ` long_rest ` |

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` shapechange ` |
| ` equipment_transformed ` | ` false ` |
| ` statistics_preserved_except ` | ` ["size"] ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Form:
    path: form
    type: symbol
  Sizes:
    path: sizes
    type: yaml
    optional: true
-->

| Key | Form | Sizes |
| --- | --- | --- |
| ` row_1 ` | ` Humanoid ` | ` ["small", "medium"] ` |
| ` row_2 ` | ` True form ` |  |

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
