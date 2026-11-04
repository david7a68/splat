---
splat_schema: 2
splat_id: fdee7df5-944b-5b70-80bc-e693430ff2fd
splat_kind: creature
splat_key: dnd2024:creatures/rakshasa
splat_name: Rakshasa
splat_origin: dnd2024:reference/creature/rakshasa
splat_sources:
- SRD 5.2.1, pp. 317, 318, https://www.dndbeyond.com/srd
---

# Rakshasa


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fiend |
| Alignment | Lawful Evil |
| Armor Class | 17 |
| Hit Points | 221 (26d8 + 104) |
| Initiative | +8 (18) |
| Speed | 40 ft. |
| Challenge | 13 (XP 10,000; PB +5) |
| Skills | Deception +10, Insight +8, Perception +8 |
| Senses | Truesight 60 ft.; Passive Perception 18 |
| Languages | Common, Infernal |
| Immunities | Charmed, Frightened |
| Vulnerabilities | Piercing damage from weapons wielded by creatures under the effect of a Bless spell |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 17 | +3 | +3 |
| Con | 18 | +4 | +4 |
| Int | 13 | +1 | +1 |
| Wis | 16 | +3 | +3 |
| Cha | 20 | +5 | +5 |


## Source text

<!-- SRD page 317 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fiend, Lawful Evil\
> AC 17 Initiative +8 (18)\
> HP 221 (26d8 + 104)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 17 +3 +3 Con 18 +4 +4\
> Int 13 +1 +1 Wis 16 +3 +3 Cha 20 +5 +5\
> Skills Deception +10, Insight +8, Perception +8\
> Vulnerabilities Piercing damage from weapons wielded by creatures under the effect of a Bless spell\
> Immunities Charmed, Frightened\
> Senses Truesight 60 ft.; Passive Perception 18\
> Languages Common, Infernal\
> CR 13 (XP 10,000; PB +5)


<!-- SRD page 318 -->

### Traits

Greater Magic Resistance. The rakshasa automatically succeeds on saving throws against spells and other magical effects, and the attack rolls of spells automatically miss it. Without the rakshasa’s permission, no spell can observe the rakshasa remotely or detect its thoughts, creature type, or alignment.

Fiendish Restoration. If the rakshasa dies outside the Nine Hells, its body turns to ichor, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Nine Hells.

### Actions

Multiattack. The rakshasa makes three Cursed Touch attacks.

Cursed Touch. Melee Attack Roll: +10, reach 5 ft. Hit: 12 (2d6 + 5) Slashing damage plus 19 (3d12) Necrotic damage. If the target is a creature, it is cursed. While cursed, the target gains no benefit from finishing a Short or Long Rest.

Baleful Command (Recharge 5–6). Wisdom Saving Throw: DC 18, each enemy in a 30-foot Emanation originating from the rakshasa. Failure: 28 (8d6) Psychic damage, and the target has the Frightened and Incapacitated conditions until the start of the rakshasa’s next turn.

Spellcasting. The rakshasa casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 18):

At Will: Detect Magic, Detect Thoughts, Disguise Self, Mage Hand, Minor Illusion\
1/Day Each: Fly, Invisibility, Major Image, Plane Shift

Original layout: [[../../srd-5.2.1/pages/317|page 317]], [[../../srd-5.2.1/pages/318|page 318]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/rakshasa ` |
| ` implementation ` | ` dnd2024:creature_rakshasa ` |

### Parameters

#### Features

##### Cursed touch

<!-- splat:yaml path=splat_rule/parameters/features/cursed_touch -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 10
targets:
  count: 1
hit:
- op: damage
  average: 12
  amount:
    dice:
      count: 2
      sides: 6
    add: 5
  damage_type: slashing
- op: damage
  average: 19
  amount:
    dice:
      count: 3
      sides: 12
    add: 0
  damage_type: necrotic
- op: if
  test:
    target_kind: creature
  then:
  - op: curse
    effects:
    - op: disable_rest_benefits
      rests:
      - short
      - long
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: cursed_touch
  count: 3
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 18 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- detect_magic
- detect_thoughts
- disguise_self
- mage_hand
- minor_illusion
- fly
- invisibility
- major_image
- plane_shift
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Cast level:
    path: cast_level
    type: symbol
  Uses:
    path: uses
    type: integer
    optional: true
  Resource scope:
    path: resource_scope
    type: symbol
    optional: true
  Recovery:
    path: recovery
    type: symbol
    optional: true
-->

| Spell | Frequency | Cast level | Uses | Resource scope | Recovery |
| --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Disguise Self](<../spells/disguise_self.md> "dnd2024:spells/disguise_self") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Fly](<../spells/fly.md> "dnd2024:spells/fly") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Major Image](<../spells/major_image.md> "dnd2024:spells/major_image") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Fiendish restoration

<!-- splat:yaml path=splat_rule/parameters/features/fiendish_restoration -->

```yaml
economy: passive
resource:
  capacity: 1
  recovery:
  - long_rest
  cost: 1
trigger: self.died
requires:
  death_plane_not: nine_hells
op: revive
new_body: true
old_body: disappears
delay:
  seconds: 0
hit_points: maximum
destination:
  plane: nine_hells
  location_choice: gm
```

##### Greater magic resistance

<!-- splat:yaml path=splat_rule/parameters/features/greater_magic_resistance -->

```yaml
economy: passive
auto_success:
  roll: saving_throw
  against:
  - spell
  - magical_effect
auto_miss:
  incoming_roll: spell_attack
permission_required_for:
  spell_detection:
  - remote_observation
  - thoughts
  - creature_type
  - alignment
```

##### Baleful command

<!-- splat:yaml path=splat_rule/parameters/features/baleful_command -->

```yaml
economy: action
resource:
  capacity: 1
  initial: 1
  recharge:
    event: turn.start
    participant: self
    roll:
      dice:
        count: 1
        sides: 6
      add: 0
    succeeds_on:
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: wisdom
dc: 18
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 30
    origin: self
  relation: enemy
failure:
- op: damage
  average: 28
  amount:
    dice:
      count: 8
      sides: 6
    add: 0
  damage_type: psychic
- op: condition
  condition: frightened
  ends:
    event: turn.start
    participant: self
    occurrence: next
- op: condition
  condition: incapacitated
  ends:
    event: turn.start
    participant: self
    occurrence: next
success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 10 ` |
| ` insight ` | ` 8 ` |
| ` perception ` | ` 8 ` |

#### Defenses

##### Vulnerabilities

<!-- splat:yaml path=splat_rule/parameters/defenses/vulnerabilities -->

```yaml
conditional:
- damage_type: piercing
  source_kind: weapon
  weapon_wielder_has_spell_effect:
    key: dnd2024:spells/bless
```

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["charmed", "frightened"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 18 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` truesight ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "infernal"] ` |
| ` understands ` | ` ["common", "infernal"] ` |

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
