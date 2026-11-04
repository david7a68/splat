---
splat_schema: 2
splat_id: d64bea02-ec56-53e9-a241-6854a12e1fe5
splat_kind: spell
splat_key: dnd2024:spells/animate_objects
splat_name: Animate Objects
splat_origin: dnd2024:reference/spell/animate_objects
splat_sources:
- SRD 5.2.1, pp. 108, 109, https://www.dndbeyond.com/srd
---

# Animate Objects


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Transmutation |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 108 -->

Level 5 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

Objects animate at your command. Choose a number of nonmagical objects within range that aren’t being worn or carried, aren’t fixed to a surface, and aren’t Gargantuan. The maximum number of objects is equal to your spellcasting ability modifier; for this number, a Medium or smaller target counts as one object, a Large target counts as two, and a Huge target counts as three.

<!-- SRD page 109 -->

Each target animates, sprouts legs, and becomes a Construct that uses the Animated Object stat block; this creature is under your control until the spell ends or until it is reduced to 0 Hit Points. Each creature you make with this spell is an ally to you and your allies. In combat, it shares your Initiative count and takes its turn immediately after yours.

Until the spell ends, you can take a Bonus Action to mentally command any creature you made with this spell if the creature is within 500 feet of you (if you control multiple creatures, you can command any of them at the same time, issuing the same command to each one). If you issue no commands, the creature takes the Dodge action and moves only to avoid harm. When the creature drops to 0 Hit Points, it reverts to its object form, and any remaining damage carries over to that form.

Using a Higher-Level Spell Slot. The creature’s Slam damage increases by 1d4 (Medium or smaller), 1d6 (Large), or 1d12 (Huge) for each spell slot level above 5.

### Animated Object

Huge or Smaller Construct, Unaligned\
AC 15\
HP 10 (Medium or smaller), 20 (Large), 40 (Huge)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 10 +0 +0 Con 10 +0 +0\
Int 3 −4 −4 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned\
Senses Blindsight 30 ft.; Passive Perception 6\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)

### Actions

Slam. Melee Attack Roll: Bonus equals your spell attack modifier, reach 5 ft. Hit: Force damage equal to 1d4 + 3 (Medium or smaller), 2d6 + 3 + your spellcasting ability modifier (Large), or 2d12 + 3 + your spellcasting ability modifier (Huge).

Original layout: [[../../srd-5.2.1/pages/108|page 108]], [[../../srd-5.2.1/pages/109|page 109]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/animate_objects ` |
| ` implementation ` | ` dnd2024:spell_animate_objects ` |

### Parameters

#### Targeting

<!-- splat:yaml path=splat_rule/parameters/targeting -->

```yaml
kind: objects
magical: false
worn: false
carried: false
fixed_to_surface: false
excluded_sizes:
- gargantuan
selection_budget:
  stat: spellcasting_ability_modifier
costs:
  tiny: 1
  small: 1
  medium: 1
  large: 2
  huge: 3
```

#### Animation

<!-- splat:yaml path=splat_rule/parameters/animation -->

```yaml
creature_type: construct
alignment: unaligned
sprouts_legs: true
ally_of:
- caster
- caster_allies
duration:
  until_any:
  - spell_ends
  - zero_hit_points
initiative:
  same_count_as: caster
  turn_order: immediately_after_caster
control:
  economy: bonus_action
  channel: mental
  range_feet: 500
  targets: any_subset
  shared_command: true
  no_command:
    action: dodge
    movement: avoid_harm_only
reversion_at_zero_hp:
  form: original_object
  excess_damage: carries_over
stats:
  ac: 15
  hp_by_size:
    tiny: 10
    small: 10
    medium: 10
    large: 20
    huge: 40
  speed_feet: 30
  abilities:
    strength: 16
    dexterity: 10
    constitution: 10
    intelligence: 3
    wisdom: 3
    charisma: 1
  saving_throw_modifiers:
    strength: 3
    dexterity: 0
    constitution: 0
    intelligence: -4
    wisdom: -4
    charisma: -5
  damage_immunities:
  - poison
  - psychic
  condition_immunities:
  - charmed
  - exhaustion
  - frightened
  - paralyzed
  - poisoned
  blindsight_feet: 30
  passive_perception: 6
  understands_languages: caster_known
  challenge_rating: null
  xp: 0
  proficiency_bonus:
    stat: caster_proficiency_bonus
slam:
  economy: action
  attack:
    kind: melee
    modifier: caster_spell_attack_modifier
    reach_feet: 5
  damage_type: force
  damage_by_size:
    medium_or_smaller:
      dice: 1d4
      add: 3
      extra_dice_per_slot_above_5: 1d4
    large:
      dice: 2d6
      add:
        op: add
        args:
        - 3
        - stat: caster_spellcasting_ability_modifier
      extra_dice_per_slot_above_5: 1d6
    huge:
      dice: 2d12
      add:
        op: add
        args:
        - 3
        - stat: caster_spellcasting_ability_modifier
      extra_dice_per_slot_above_5: 1d12
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
