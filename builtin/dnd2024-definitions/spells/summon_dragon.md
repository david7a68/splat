---
splat_schema: 1
splat_id: 1189aa3b-a3c9-5830-a440-c389b56be2f6
splat_kind: spell
splat_key: dnd2024:spells/summon_dragon
splat_name: Summon Dragon
splat_origin:
  key: dnd2024:reference/spell/summon_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 166
  url: https://www.dndbeyond.com/srd
---

# Summon Dragon

## Source text

<!-- SRD page 166 -->

Level 5 Conjuration (Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (an object with the image of a\
dragon engraved on it worth 500+ GP)\
Duration: Concentration, up to 1 hour\
You call forth a Dragon spirit. It manifests in an un-\
occupied space that you can see within range and\
uses the Draconic Spirit stat block. The creature\
disappears when it drops to 0 Hit Points or when\
the spell ends.\
The creature is an ally to you and your allies. In\
combat, the creature shares your Initiative count,\
but it takes its turn immediately after yours. It\
obeys your verbal commands (no action required by\
you). If you don’t issue any, it takes the Dodge action\
and uses its movement to avoid danger.\
Using a Higher-Level Spell Slot. Use the spell\
slot’s level for the spell’s level in the stat block.\

### Draconic Spirit

Large Dragon, Neutral\
AC 14 + the spell’s level\
HP 50 + 10 for each spell level above 5\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +2 Con17 +3 +3\
Int 10 +0 +0 Wis 14 +2 +2 Cha 14 +2 +2\
Resistances Acid, Cold, Fire, Lightning, Poison\
Immunities Charmed, Frightened, Poisoned\
Senses Blindsight 30 ft., Darkvision 60 ft.;\
Passive Perception 12\
Languages Draconic, understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Shared Resistances. When you summon the spirit,\
choose one of its Resistances. You have Resistance to\
the chosen damage type until the spell ends.\

### Actions

Multiattack. The spirit makes a number of Rend attacks\
equal to half the spell’s level (round down), and it uses\
Breath Weapon.\
Rend. Melee Attack Roll: Bonus equals your spell attack\
modifier, reach 10 feet. Hit: 1d6 + 4 + the spell’s level\
Piercing damage.\
Breath Weapon. Dexterity Saving Throw: DC equals\
your spell save DC, each creature in a 30-foot Cone.\
Failure: 2d6 damage of a type this spirit has Resistance\
to (your choice when you cast the spell). Success:\
Half damage.\

Original layout: [[../../srd-5.2.1/pages/166|page 166]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` an object with the image of a dragon engraved on it worth 500+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/summon_dragon ` |
| ` implementation ` | ` dnd2024:spell_summon_dragon ` |

### Parameters

#### Summon

<!-- splat:yaml path=splat_rule/parameters/summon -->

```yaml
definition:
  key: dnd2024:creatures/draconic_spirit
initial_space: visible_unoccupied_in_range
ally_to:
- caster
- caster_allies
disappear_on:
- hp_zero
- spell_end
```

#### Control

<!-- splat:yaml path=splat_rule/parameters/control -->

```yaml
initiative: caster_count
turn: immediately_after_caster
verbal_commands: true
command_cost: none
no_command:
  action: dodge
  movement: avoid_danger
```

#### Spirit

<!-- splat:yaml path=splat_rule/parameters/spirit -->

```yaml
size: large
type: dragon
alignment: neutral
ac:
  op: add
  args:
  - 14
  - binding: cast_level
hp:
  op: add
  args:
  - 50
  - op: multiply
    args:
    - 10
    - op: subtract
      args:
      - binding: cast_level
      - 5
speeds_feet:
  walk: 30
  fly: 60
  swim: 30
abilities:
  strength: 19
  dexterity: 14
  constitution: 17
  intelligence: 10
  wisdom: 14
  charisma: 14
save_modifiers:
  strength: 4
  dexterity: 2
  constitution: 3
  intelligence: 0
  wisdom: 2
  charisma: 2
resistances:
- acid
- cold
- fire
- lightning
- poison
condition_immunities:
- charmed
- frightened
- poisoned
senses_feet:
  blindsight: 30
  darkvision: 60
passive_perception: 12
languages:
- draconic
understands: caster_languages
cr: none
xp: 0
proficiency_bonus:
  stat:
    of: caster
    key: proficiency_bonus
```

#### Shared resistance

<!-- splat:values path=splat_rule/parameters/shared_resistance -->

| Field | Value |
| --- | --- |
| ` choice_at_summon ` | ` one_spirit_resistance ` |
| ` beneficiary ` | ` caster ` |
| ` duration ` | ` spell ` |

#### Actions

<!-- splat:yaml path=splat_rule/parameters/actions -->

```yaml
multiattack:
  rend_count:
    op: floor
    args:
    - op: multiply
      args:
      - binding: cast_level
      - numerator: 1
        denominator: 2
  also_breath_weapon: true
rend:
  kind: melee_attack
  attack_bonus:
    binding: spell_attack_modifier
  reach_feet: 10
  damage:
    amount:
      dice:
        count: 1
        sides: 6
        add:
          op: add
          args:
          - 4
          - binding: cast_level
    type: piercing
breath_weapon:
  area:
    shape: cone
    length_feet: 30
  damage_type_choice_at_cast: one_spirit_resistance
  save:
    ability: dexterity
    dc:
      binding: save_dc
    failure:
      damage:
        amount:
          dice:
            count: 2
            sides: 6
            add: 0
        type: chosen
    success:
      damage_fraction:
        numerator: 1
        denominator: 2
```
