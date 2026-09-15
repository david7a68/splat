---
splat_schema: 1
splat_id: 6a5f535d-ceda-554a-b6af-4db9fc9e48e1
splat_kind: creature
splat_key: dnd2024:creatures/adult_copper_dragon
splat_name: Adult Copper Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_copper_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 276, 277
  url: https://www.dndbeyond.com/srd
---

# Adult Copper Dragon

## Source text

<!-- SRD page 276 -->

Huge Dragon (Metallic), Chaotic Good\
AC 18 Initiative +11 (21)\
HP 184 (16d12 + 80)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 12 +1 +6 Con 21 +5 +5\
Int 18 +4 +4 Wis 15 +2 +7 Cha 18 +4 +4\
Skills Deception +9, Perception +12, Stealth +6\
Immunities Acid\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 22\
Languages Common, Draconic\
CR 14 (XP 11,500, or 13,000 in lair; PB +5)\

### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of (A) Slowing Breath\
or (B) Spellcasting to cast Mind Spike (level 4 version).\
Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 17 (2d10\
+ 6) Slashing damage plus 4 (1d8) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 18, each creature in an 60-foot-long, 5-foot-\
wide Line. Failure: 54 (12d8) Acid damage. Success:\
Half damage.\
Slowing Breath. Constitution Saving Throw: DC 18,\
each creature in a 60-foot Cone. Failure: The target\
can’t take Reactions; its Speed is halved; and it can take\
either an action or a Bonus Action on its turn, not both.\
This effect lasts until the end of its next turn.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 17):\
At Will: Detect Magic, Mind Spike (level 4 version),\
Minor Illusion, Shapechange (Beast or Humanoid\
form only, no Temporary Hit Points gained from the\
spell, and no Concentration or Temporary Hit Points\
required to maintain the spell)\
1/Day Each: Greater Restoration, Major Image\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Giggling Magic. Charisma Saving Throw: DC 17, one\
creature the dragon can see within 90 feet. Failure: 24\
(7d6) Psychic damage. Until the end of its next turn,\
the target rolls 1d6 whenever it makes an ability check\
or attack roll and subtracts the number rolled from the\
D20 Test. Failure or Success: The dragon can’t take this\
action again until the start of its next turn.\
Mind Jolt. The dragon uses Spellcasting to cast Mind\
Spike (level 4 version). The dragon can’t take this action\
again until the start of its next turn.\

<!-- SRD page 277 -->

Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/276|page 276]], [[../../srd-5.2.1/pages/277|page 277]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Good ` |
| ` tags_text ` | ` (Metallic) ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Deception +9, Perception +12, Stealth +6 ` |
| ` immunities_text ` | ` Acid ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 14 (XP 11,500, or 13,000 in lair; PB +5) ` |
| ` hit_points_text ` | ` 184 (16d12 + 80) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 23 ` | ` 6 ` |
| ` Dexterity ` | ` 12 ` | ` 6 ` |
| ` Constitution ` | ` 21 ` | ` 5 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 15 ` | ` 7 ` |
| ` Charisma ` | ` 18 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 184 ` |  |
| ` initiative ` | ` 11 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/adult_copper_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_copper_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["acid"] ` |
| ` passive_perception ` | ` 22 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 9 ` |
| ` perception ` | ` 12 ` |
| ` stealth ` | ` 6 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:table
path: splat_rule/parameters/senses
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
  Range ft:
    path: range_ft
    type: integer
-->

| Key | Sense | Range ft |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 60 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "draconic"] ` |

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
trigger: saving_throw.failed
optional: true
op: saving_throw.set_result
result: success
uses_in_lair: 4
uses:
  count: 3
  recovery: long_rest
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 11
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 10
      add: 6
    average: 17
  - type: acid
    amount:
      dice:
        count: 1
        sides: 8
      add: 0
    average: 4
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
| ` ability ` | ` charisma ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 17 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_magic
- mind_spike
- minor_illusion
- shapechange
- greater_restoration
- major_image
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
  Overrides forms:
    path: overrides/forms
    type: yaml
    optional: true
  Overrides temporary hp granted:
    path: overrides/temporary_hp_granted
    type: integer
    optional: true
  Overrides maintain requires concentration:
    path: overrides/maintain_requires_concentration
    type: boolean
    optional: true
  Overrides maintain requires temporary hp:
    path: overrides/maintain_requires_temporary_hp
    type: boolean
    optional: true
  Uses count:
    path: uses/count
    type: integer
    optional: true
  Uses recovery:
    path: uses/recovery
    type: symbol
    optional: true
-->

| Spell | Uses unlimited | Cast level | Overrides forms | Overrides temporary hp granted | Overrides maintain requires concentration | Overrides maintain requires temporary hp | Uses count | Uses recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |  |  |  |  |  |
| [Mind Spike](<../spells/mind_spike.md> "dnd2024:spells/mind_spike") | true | ` 4 ` |  |  |  |  |  |  |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | true |  |  |  |  |  |  |  |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | true |  | ` ["beast", "humanoid"] ` | ` 0 ` | false | false |  |  |
| [Greater Restoration](<../spells/greater_restoration.md> "dnd2024:spells/greater_restoration") |  |  |  |  |  |  | ` 1 ` | ` Long rest ` |
| [Major Image](<../spells/major_image.md> "dnd2024:spells/major_image") |  |  |  |  |  |  | ` 1 ` | ` Long rest ` |

##### Acid breath

<!-- splat:yaml path=splat_rule/parameters/features/acid_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 18
targets:
  kind: creature
  count: all
  area:
    shape: line
    origin: actor
    size_ft: 60
    width_ft: 5
failure:
- op: damage
  components:
  - type: acid
    amount:
      dice:
        count: 12
        sides: 8
      add: 0
    average: 54
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
recharge:
  die_sides: 6
  success_min: 5
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Slowing breath

<!-- splat:yaml path=splat_rule/parameters/features/slowing_breath -->

```yaml
economy: action
op: saving_throw
ability: constitution
dc: 18
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 60
failure:
- op: effect.apply
  duration:
    until: target.next_turn_end
  rules:
    reactions_allowed: false
    speed_multiplier: 0.5
    turn_economy_choice:
    - action
    - bonus_action
    both_allowed: false
success: []
```

##### Giggling magic

<!-- splat:yaml path=splat_rule/parameters/features/giggling_magic -->

```yaml
economy: legendary_action
op: saving_throw
ability: charisma
dc: 17
targets:
  kind: creature
  count: 1
  range_ft: 90
  visible_to_actor: true
failure:
- op: damage
  components:
  - type: psychic
    amount:
      dice:
        count: 7
        sides: 6
      add: 0
    average: 24
- op: roll.add
  amount:
    negate:
      dice:
        count: 1
        sides: 6
      add: 0
  rolls:
  - ability_check
  - attack
  roll_each_test: true
  duration:
    until: target.next_turn_end
success: []
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Mind jolt

<!-- splat:yaml path=splat_rule/parameters/features/mind_jolt -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: spell.cast
  spell:
    key: dnd2024:spells/mind_spike
  profile: spellcasting
  cast_level: 4
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Pounce

<!-- splat:yaml path=splat_rule/parameters/features/pounce -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: movement
  maximum:
    multiply:
    - 0.5
    - actor_stat: speed
- op: feature.use
  feature: rend
legendary_cost: 1
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 3
  feature: rend
replace_attack:
  maximum: 1
  choices:
  - op: feature.use
    feature: slowing_breath
  - op: spell.cast
    spell:
      key: dnd2024:spells/mind_spike
    profile: spellcasting
    cast_level: 4
```

#### Legendary actions

<!-- splat:values path=splat_rule/parameters/legendary_actions -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 3 ` |
| ` uses_in_lair ` | ` 4 ` |
| ` trigger ` | ` another_creature.turn_end_immediately_after ` |
| ` cost_per_action ` | ` 1 ` |
| ` recover_all ` | ` actor.turn_start ` |
| ` maximum_per_trigger ` | ` 1 ` |
| ` requires_actions_available ` | ` true ` |
| ` excluded_conditions ` | ` ["incapacitated"] ` |

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
