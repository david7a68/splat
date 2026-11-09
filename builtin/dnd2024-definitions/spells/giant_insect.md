---
splat_schema: 2
splat_id: 78708bf7-1722-50ee-b63e-52553566fb9a
splat_kind: spell
splat_key: dnd2024:spells/giant_insect
splat_name: Giant Insect
splat_origin: dnd2024:reference/spell/giant_insect
splat_sources:
- SRD 5.2.1, pp. 135, 136, https://www.dndbeyond.com/srd
---

# Giant Insect


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Conjuration |
| Classes | Druid |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 135 -->

Level 4 Conjuration (Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You summon a giant centipede, spider, or wasp (chosen when you cast the spell). It manifests in an unoccupied space you can see within range and uses the Giant Insect stat block. The form you choose determines certain details in its stat block. The creature disappears when it drops to 0 Hit Points or when the spell ends.

The creature is an ally to you and your allies. In combat, the creature shares your Initiative count, but it takes its turn immediately after yours. It obeys your verbal commands (no action required by you). If you don’t issue any, it takes the Dodge action and uses its movement to avoid danger.

<!-- SRD page 136 -->

Using a Higher-Level Spell Slot. Use the spell slot’s level for the spell’s level in the stat block.

### Giant Insect

Large Beast, Unaligned\
AC 11 + the spell’s level\
HP 30 + 10 for each spell level above 4\
Speed 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con 15 +2 +2\
Int 4 −3 −3 Wis 14 +2 +2 Cha 3 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)

### Traits

Spider Climb. The insect can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Multiattack. The insect makes a number of attacks equal to half this spell’s level (round down).

Poison Jab. Melee Attack Roll: Bonus equals your spell attack modifier, reach 10 ft. Hit: 1d6 + 3 plus the spell’s level Piercing damage plus 1d4 Poison damage.

Web Bolt (Spider Only). Ranged Attack Roll: Bonus equals your spell attack modifier, range 60 ft. Hit: 1d10 + 3 plus the spell’s level Bludgeoning damage, and the target’s Speed is reduced to 0 until the start of the insect’s next turn.

### Bonus Actions

Venomous Spew (Centipede Only). Constitution Saving Throw: Your spell save DC, one creature the insect can see within 10 feet. Failure: The target has the Poisoned condition until the start of the insect’s next turn.

Original layout: [[../../srd-5.2.1/pages/135|page 135]], [[../../srd-5.2.1/pages/136|page 136]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/giant_insect ` |
| ` implementation ` | ` dnd2024:spell_giant_insect ` |

### Parameters

#### Summon

<!-- splat:yaml path=splat_rule/parameters/summon -->

```yaml
form_choice:
- centipede
- spider
- wasp
space:
  unoccupied: true
  visible: true
  within_spell_range: true
ally_to:
- caster
- caster_allies
disappear:
- hp_zero
- spell_ends
```

#### Combat

<!-- splat:yaml path=splat_rule/parameters/combat -->

```yaml
initiative: caster_count
turn: immediately_after_caster
verbal_commands_cost: none
without_command:
  action: dodge
  movement: avoid_danger
```

#### Statistics

<!-- splat:yaml path=splat_rule/parameters/statistics -->

```yaml
size: large
type: beast
alignment: unaligned
ac:
  op: add
  args:
  - 11
  - binding: cast_level
hp:
  op: add
  args:
  - 30
  - op: multiply
    args:
    - 10
    - op: subtract
      args:
      - binding: cast_level
      - 4
speed_feet: 40
climb_speed_feet: 40
fly:
  form: wasp
  speed_feet: 40
abilities:
  strength: 17
  dexterity: 13
  constitution: 15
  intelligence: 4
  wisdom: 14
  charisma: 3
save_modifiers:
  strength: 3
  dexterity: 1
  constitution: 2
  intelligence: -3
  wisdom: 2
  charisma: -4
darkvision_feet: 60
passive_perception: 12
understood_languages: caster_known_languages
challenge_rating: null
xp: 0
proficiency_bonus:
  binding: caster_proficiency_bonus
```

#### Traits

<!-- splat:yaml path=splat_rule/parameters/traits -->

```yaml
spider_climb:
  climb_difficult_surfaces: true
  ceilings: true
  requires_ability_check: false
```

#### Actions

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/actions/multiattack -->

```yaml
attack_count:
  op: floor
  args:
  - op: multiply
    args:
    - binding: cast_level
    - numerator: 1
      denominator: 2
choices:
- poison_jab
- web_bolt_if_spider
```

##### Poison jab

<!-- splat:values path=splat_rule/parameters/actions/poison_jab -->

| Field | Value |
| --- | --- |
| ` attack ` | ` melee ` |
| ` reach_feet ` | ` 10 ` |

###### Bonus

<!-- splat:values path=splat_rule/parameters/actions/poison_jab/bonus -->

| Field | Value |
| --- | --- |
| ` binding ` | ` spell_attack_modifier ` |

###### Damage

<!-- splat:table
path: splat_rule/parameters/actions/poison_jab/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice count:
    path: dice/count
    type: integer
  Dice sides:
    path: dice/sides
    type: integer
  Dice add op:
    path: dice/add/op
    type: symbol
    optional: true
  Type:
    path: type
    type: symbol
  Details:
    type: section
    optional: true
-->

| Key | Dice count | Dice sides | Dice add op | Type | Details |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 6 ` | ` Add ` | ` Piercing ` | [Rules](#^splat-5d609e49-row_1 "section:row_1") |
| ` row_2 ` | ` 1 ` | ` 4 ` |  | ` Poison ` | [Rules](#^splat-5d609e49-row_2 "section:row_2") |

###### Row 1

^splat-5d609e49-row_1

###### Args

<!-- splat:yaml path=splat_rule/parameters/actions/poison_jab/damage/@row_1/dice/add/args -->

```yaml
- 3
- binding: cast_level
```

###### Row 2

^splat-5d609e49-row_2

###### Add

<!-- splat:yaml path=splat_rule/parameters/actions/poison_jab/damage/@row_2/dice/add -->

```yaml
0
```

##### Web bolt

<!-- splat:yaml path=splat_rule/parameters/actions/web_bolt -->

```yaml
form: spider
attack: ranged
bonus:
  binding: spell_attack_modifier
range_feet: 60
damage:
  dice:
    count: 1
    sides: 10
    add:
      op: add
      args:
      - 3
      - binding: cast_level
  type: bludgeoning
hit_effect:
  speed: 0
  until: insect_next_turn_start
```

#### Bonus actions

<!-- splat:yaml path=splat_rule/parameters/bonus_actions -->

```yaml
venomous_spew:
  form: centipede
  target:
    kind: creature
    count: 1
    visible_to_insect: true
    range_feet: 10
  save:
    ability: constitution
    dc:
      binding: save_dc
    failure:
      condition: poisoned
      until: insect_next_turn_start
    success:
      effect: none
```
