---
splat_schema: 1
splat_id: 72a0fcde-868b-5416-9191-9d3f82755294
splat_kind: spell
splat_key: dnd2024:spells/find_steed
splat_name: Find Steed
splat_origin:
  key: dnd2024:reference/spell/find_steed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 130, 131
  url: https://www.dndbeyond.com/srd
---

# Find Steed

## Source text

<!-- SRD page 130 -->

Level 2 Conjuration (Paladin)\
Casting Time: Action\
Range: 30 feet\
Component: V, S\
Duration: Instantaneous\
You summon an otherworldly being that appears as\
a loyal steed in an unoccupied space of your choice\
within range. This creature uses the Otherworldly\
Steed stat block. If you already have a steed from\
this spell, the steed is replaced by the new one.\
The steed resembles a Large, rideable animal of\
your choice, such as a horse, a camel, a dire wolf,\
or an elk. Whenever you cast the spell, choose the\
steed’s creature type—Celestial, Fey, or Fiend—\
which determines certain traits in the stat block.\
Combat. The steed is an ally to you and your al-\
lies. In combat, it shares your Initiative count, and\
it functions as a controlled mount while you ride it\
(as defined in the rules on mounted combat). If you\
have the Incapacitated condition, the steed takes\
its turn immediately after yours and acts inde-\
pendently, focusing on protecting you.\
Disappearance of the Steed. The steed disap-\
pears if it drops to 0 Hit Points or if you die. When\
it disappears, it leaves behind anything it was\
wearing or carrying. If you cast this spell again, you\
decide whether you summon the steed that disap-\
peared or a different one.\
Using a Higher-Level Spell Slot. Use the spell\
slot’s level for the spell’s level in the stat block.\

<!-- SRD page 131 -->


### Otherworldly Steed

Large Celestial, Fey, or Fiend (Your Choice), Neutral\
AC 10 + 1 per spell level\
HP 5 + 10 per spell level (the steed has a number of Hit\
Dice [d10s] equal to the spell’s level)\
Speed 60 ft., Fly 60 ft. (requires level 4+ spell)\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con14 +2 +2\
Int 6 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Senses Passive Perception 11\
Languages Telepathy 1 mile (works only with you)\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Life Bond. When you regain Hit Points from a level 1+\
spell, the steed regains the same number of Hit Points if\
you’re within 5 feet of it.\

### Actions

Otherworldly Slam. Melee Attack Roll: Bonus equals\
your spell attack modifier, reach 5 ft. Hit: 1d8 plus the\
spell’s level of Radiant (Celestial), Psychic (Fey), or Ne-\
crotic (Fiend) damage.\

### Bonus Actions

Fell Glare (Fiend Only; Recharges after a Long Rest).\
Wisdom Saving Throw: DC equals your spell save DC,\
one creature within 60 feet the steed can see. Failure:\
The target has the Frightened condition until the end of\
your next turn.\
Fey Step (Fey Only; Recharges after a Long Rest). The\
steed teleports, along with its rider, to an unoccupied\
space of your choice up to 60 feet away from itself.\
Healing Touch (Celestial Only; Recharges after a Long\
Rest). One creature within 5 feet of the steed regains a\
number of Hit Points equal to 2d8 plus the spell’s level.\

Original layout: [[../../srd-5.2.1/pages/130|page 130]], [[../../srd-5.2.1/pages/131|page 131]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["paladin"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/find_steed ` |
| ` implementation ` | ` dnd2024:spell_find_steed ` |

### Parameters

#### Summon

<!-- splat:yaml path=splat_rule/parameters/summon -->

```yaml
space:
  unoccupied: true
  within_spell_range: true
  chooser: caster
replace_existing_steed_from_spell: true
appearance:
  size: large
  form: rideable_animal
  chooser: caster
creature_type_choice:
- celestial
- fey
- fiend
alignment: neutral
ally_to:
- caster
- caster_allies
returning_steed_identity_choice:
- previous
- different
```

#### Combat

<!-- splat:yaml path=splat_rule/parameters/combat -->

```yaml
initiative: caster_count
while_ridden: controlled_mount
while_caster_incapacitated:
  turn: immediately_after_caster
  control: independent
  priority: protect_caster
```

#### Disappear

<!-- splat:values path=splat_rule/parameters/disappear -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["steed_hp_zero", "caster_death"] ` |
| ` leave_worn_and_carried_items ` | ` true ` |

#### Statistics

<!-- splat:yaml path=splat_rule/parameters/statistics -->

```yaml
ac:
  op: add
  args:
  - 12
  - op: multiply
    args:
    - 1
    - op: subtract
      args:
      - binding: cast_level
      - 2
hp:
  op: add
  args:
  - 25
  - op: multiply
    args:
    - 10
    - op: subtract
      args:
      - binding: cast_level
      - 2
hit_dice:
  dice:
    count:
      binding: cast_level
    sides: 10
    add: 0
speed_feet: 60
fly:
  speed_feet: 60
  minimum_cast_level: 4
abilities:
  strength: 18
  dexterity: 12
  constitution: 14
  intelligence: 6
  wisdom: 12
  charisma: 8
save_modifiers:
  strength: 4
  dexterity: 1
  constitution: 2
  intelligence: -2
  wisdom: 1
  charisma: -1
passive_perception: 11
telepathy:
  miles: 1
  only_with: caster
challenge_rating: null
xp: 0
proficiency_bonus:
  binding: caster_proficiency_bonus
```

#### Life bond

<!-- splat:values path=splat_rule/parameters/life_bond -->

| Field | Value |
| --- | --- |
| ` event ` | ` caster_regains_hp_from_level_1_plus_spell ` |
| ` caster_distance_maximum_feet ` | ` 5 ` |
| ` steed_healing ` | ` same_hp_amount ` |

#### Actions

<!-- splat:yaml path=splat_rule/parameters/actions -->

```yaml
otherworldly_slam:
  attack: melee
  attack_bonus:
    binding: spell_attack_modifier
  reach_feet: 5
  damage:
    dice:
      count: 1
      sides: 8
      add:
        binding: cast_level
    type: creature_type_dependent
  damage_type_by_creature_type:
    celestial: radiant
    fey: psychic
    fiend: necrotic
```

#### Bonus actions

<!-- splat:yaml path=splat_rule/parameters/bonus_actions -->

```yaml
fell_glare:
  creature_type: fiend
  uses: 1
  recharge: long_rest
  target:
    count: 1
    visible_to_steed: true
    range_feet: 60
  save:
    ability: wisdom
    dc:
      binding: save_dc
    failure:
      condition: frightened
      until: caster_next_turn_end
    success:
      effect: none
fey_step:
  creature_type: fey
  uses: 1
  recharge: long_rest
  teleport:
    subjects:
    - steed
    - rider
    range_feet: 60
    destination_unoccupied: true
    chooser: caster
healing_touch:
  creature_type: celestial
  uses: 1
  recharge: long_rest
  target:
    count: 1
    range_feet: 5
  healing:
    dice:
      count: 2
      sides: 8
      add:
        binding: cast_level
```
