---
splat_schema: 1
splat_id: ae18da0f-a055-5ce5-be8f-dffb5019d6c9
splat_kind: creature
splat_key: dnd2024:creatures/sphinx_of_lore
splat_name: Sphinx of Lore
splat_origin:
  key: dnd2024:reference/creature/sphinx_of_lore
splat_sources:
- title: SRD 5.2.1
  locator: pp. 327, 328
  url: https://www.dndbeyond.com/srd
---

# Sphinx of Lore

## Source text

<!-- SRD page 327 -->

Large Celestial, Lawful Neutral\
AC 17 Initiative +10 (20)\
HP 170 (20d10 + 60)\
Speed 40 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 18 +4 +4 Wis 18 +4 +4 Cha 18 +4 +4\
Skills Arcana +12, History +12, Perception +8, Religion +12\
Resistances Necrotic, Radiant\
Immunities Psychic; Charmed, Frightened\
Senses Truesight 120 ft.; Passive Perception 18\
Languages Celestial, Common\
CR 11 (XP 7,200, or 8,400 in lair; PB +4)

### Traits

Inscrutable. No magic can observe the sphinx remotely or detect its thoughts without its permission. Wisdom (Insight) checks made to ascertain its intentions or sincerity are made with Disadvantage.

Legendary Resistance (3/Day, or 4/Day in Lair). If the sphinx fails a saving throw, it can choose to succeed instead.

<!-- SRD page 328 -->

### Actions

Multiattack. The sphinx makes three Claw attacks.

Claw. Melee Attack Roll: +8, reach 5 ft. Hit: 14 (3d6 + 4) Slashing damage.

Mind-Rending Roar (Recharge 5–6). Wisdom Saving Throw: DC 16, each enemy in a 300-foot Emanation originating from the sphinx. Failure: 35 (10d6) Psychic damage, and the target has the Incapacitated condition until the start of the sphinx’s next turn.

Spellcasting. The sphinx casts one of the following spells, requiring no Material components and using Intelligence as the spellcasting ability (spell save DC 16):

At Will: Detect Magic, Identify, Mage Hand, Minor Illusion, Prestidigitation\
1/Day Each: Dispel Magic, Legend Lore, Locate Object, Plane Shift, Remove Curse, Tongues

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the sphinx can expend a use to take one of the following actions. The sphinx regains all expended uses at the start of each of its turns.

Arcane Prowl. The sphinx can teleport up to 30 feet to an unoccupied space it can see, and it makes one Claw attack.

Weight of Years. Constitution Saving Throw: DC 16, one creature the sphinx can see within 120 feet. Failure: The target gains 1 Exhaustion level. While the target has any Exhaustion levels, it appears 3d10 years older. Failure or Success: The sphinx can’t take this action again until the start of its next turn.

Original layout: [[../../srd-5.2.1/pages/327|page 327]], [[../../srd-5.2.1/pages/328|page 328]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Lawful Neutral ` |
| ` movement_text ` | ` 40 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Arcana +12, History +12, Perception +8, Religion +12 ` |
| ` resistances_text ` | ` Necrotic, Radiant ` |
| ` immunities_text ` | ` Psychic; Charmed, Frightened ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 18 ` |
| ` languages_text ` | ` Celestial, Common ` |
| ` cr_text ` | ` 11 (XP 7,200, or 8,400 in lair; PB +4) ` |
| ` hit_points_text ` | ` 170 (20d10 + 60) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 18 ` | ` 4 ` |
| ` Charisma ` | ` 18 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 170 ` |  |
| ` initiative ` | ` 10 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/sphinx_of_lore ` |
| ` implementation ` | ` dnd2024:creature_sphinx_of_lore ` |

### Parameters

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
resource:
  capacity: 3
  capacity_in_lair: 4
  recovery: long_rest
  cost: 1
trigger: saving_throw.failed
optional: true
op: replace_save_outcome
outcome: success
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 8
targets:
  count: 1
hit:
- op: damage
  average: 14
  amount:
    dice:
      count: 3
      sides: 6
    add: 4
  damage_type: slashing
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: claw
  count: 3
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` intelligence ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 16 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- detect_magic
- identify
- mage_hand
- minor_illusion
- prestidigitation
- dispel_magic
- legend_lore
- locate_object
- plane_shift
- remove_curse
- tongues
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
| [Identify](<../spells/identify.md> "dnd2024:spells/identify") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Prestidigitation](<../spells/prestidigitation.md> "dnd2024:spells/prestidigitation") | ` At will ` | ` Spell minimum ` |  |  |  |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Legend Lore](<../spells/legend_lore.md> "dnd2024:spells/legend_lore") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Locate Object](<../spells/locate_object.md> "dnd2024:spells/locate_object") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Remove Curse](<../spells/remove_curse.md> "dnd2024:spells/remove_curse") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Tongues](<../spells/tongues.md> "dnd2024:spells/tongues") | ` Daily ` | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Inscrutable

<!-- splat:yaml path=splat_rule/parameters/features/inscrutable -->

```yaml
economy: passive
remote_magical_observation:
  requires_permission: true
magical_thought_detection:
  requires_permission: true
insight_disadvantage:
  purpose:
  - ascertain_intentions
  - ascertain_sincerity
```

##### Mind rending roar

<!-- splat:yaml path=splat_rule/parameters/features/mind_rending_roar -->

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
dc: 16
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 300
    origin: self
  relation: enemy
failure:
- op: damage
  average: 35
  amount:
    dice:
      count: 10
      sides: 6
    add: 0
  damage_type: psychic
- op: condition
  condition: incapacitated
  ends:
    event: turn.start
    participant: self
    occurrence: next
success: []
```

##### Weight of years

<!-- splat:yaml path=splat_rule/parameters/features/weight_of_years -->

```yaml
economy: legendary_action
cooldown_ends:
  event: turn.start
  participant: self
  occurrence: next
op: save
ability: constitution
dc: 16
targets:
  kind: creature
  count: 1
  range_feet: 120
  visible_to: self
failure:
- op: exhaustion
  levels: 1
- op: apparent_age_increase
  years:
    dice:
      count: 3
      sides: 10
    add: 0
  ends:
    target_exhaustion_levels: 0
success: []
```

##### Arcane prowl

<!-- splat:yaml path=splat_rule/parameters/features/arcane_prowl -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: optional
  effect:
    op: teleport
    range_feet: 30
    destination:
      unoccupied: true
      visible_to: self
- op: use_feature
  feature: claw
  count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 12 ` |
| ` history ` | ` 12 ` |
| ` perception ` | ` 8 ` |
| ` religion ` | ` 12 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["necrotic", "radiant"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["psychic"] ` |
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
| ` 0 ` | ` truesight ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["celestial", "common"] ` |
| ` understands ` | ` ["celestial", "common"] ` |

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
uses_in_lair: 4
cost_per_action: 1
trigger: another_creature.turn.end
recovery:
  event: turn.start
  participant: self
  amount: all
maximum_per_trigger: 1
requires:
  self_incapacitated: false
  self_can_take_actions: true
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
