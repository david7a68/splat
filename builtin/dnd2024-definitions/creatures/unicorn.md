---
splat_schema: 1
splat_id: 3dcfc58e-7518-58cb-a708-6e67789cc063
splat_kind: creature
splat_key: dnd2024:creatures/unicorn
splat_name: Unicorn
splat_origin:
  key: dnd2024:reference/creature/unicorn
splat_sources:
- title: SRD 5.2.1
  locator: pp. 334
  url: https://www.dndbeyond.com/srd
---

# Unicorn

## Source text

<!-- SRD page 334 -->

Large Celestial, Lawful Good\
AC 12 Initiative +8 (18)\
HP 97 (13d10 + 26)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 15 +2 +2\
Int 11 +0 +0 Wis 17 +3 +3 Cha 16 +3 +3\
Immunities Poison; Charmed, Paralyzed, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages Celestial, Elvish, Sylvan; telepathy 120 ft.\
CR 5 (XP 1,800; PB +3)

### Traits

Legendary Resistance (3/Day). If the unicorn fails a saving throw, it can choose to succeed instead.

Magic Resistance. The unicorn has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The unicorn makes one Hooves attack and one Radiant Horn attack.

Hooves. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (2d6 + 4) Bludgeoning damage.

Radiant Horn. Melee Attack Roll: +7, reach 5 ft. Hit: 9 (1d10 + 4) Radiant damage.

Spellcasting. The unicorn casts one of the following spells, requiring no spell components and using Charisma as the spellcasting ability (spell save DC 14):

At Will: Detect Evil and Good, Druidcraft\
1/Day Each: Calm Emotions, Dispel Evil and Good, Entangle, Pass without Trace, Word of Recall

### Bonus Actions

Unicorn’s Blessing (3/Day). The unicorn touches another creature with its horn and casts Cure Wounds or Lesser Restoration on that creature, using the same spellcasting ability as Spellcasting.

### Legendary Actions

Legendary Action Uses: 3. Immediately after another creature’s turn, the unicorn can expend a use to take one of the following actions. The unicorn regains all expended uses at the start of each of its turns.

Charging Horn. The unicorn moves up to half its Speed without provoking Opportunity Attacks, and it makes one Radiant Horn attack.

Shimmering Shield. The unicorn targets itself or one creature it can see within 60 feet of itself. The target gains 10 (3d6) Temporary Hit Points, and its AC increases by 2 until the end of the unicorn’s next turn. The unicorn can’t take this action again until the start of its next turn.

Original layout: [[../../srd-5.2.1/pages/334|page 334]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Lawful Good ` |
| ` movement_text ` | ` 50 ft. ` |
| ` immunities_text ` | ` Poison; Charmed, Paralyzed, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 13 ` |
| ` languages_text ` | ` Celestial, Elvish, Sylvan; telepathy 120 ft. ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 97 (13d10 + 26) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 11 ` | ` 0 ` |
| ` Wisdom ` | ` 17 ` | ` 3 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 97 ` |  |
| ` initiative ` | ` 8 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/unicorn ` |
| ` implementation ` | ` dnd2024:creature_unicorn ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["charmed", "paralyzed", "poisoned"] ` |

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: trait
resource:
  kind: daily
  uses: 3
  recovery: long_rest
operation: replace_save_result
trigger: self.save_failed
optional: true
result: success
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: hooves
  count: 1
- operation: attack_sequence
  feature: radiant_horn
  count: 1
```

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 7
  reach_ft: 5
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 6
      modifier: 4
    type: bludgeoning
```

##### Radiant horn

<!-- splat:yaml path=splat_rule/parameters/features/radiant_horn -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 7
  reach_ft: 5
hit:
  damage:
  - average: 9
    roll:
      count: 1
      sides: 10
      modifier: 4
    type: radiant
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["verbal", "somatic", "material"] ` |
| ` save_dc ` | ` 14 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_evil_and_good
- druidcraft
- calm_emotions
- dispel_evil_and_good
- entangle
- pass_without_trace
- word_of_recall
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: usage/kind
    type: symbol
  Uses:
    path: usage/uses
    type: integer
    optional: true
  Use pool:
    path: usage/pool
    type: symbol
    optional: true
  Recovery:
    path: usage/recovery
    type: symbol
    optional: true
-->

| Spell | Frequency | Uses | Use pool | Recovery |
| --- | --- | --- | --- | --- |
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | ` At will ` |  |  |  |
| [Druidcraft](<../spells/druidcraft.md> "dnd2024:spells/druidcraft") | ` At will ` |  |  |  |
| [Calm Emotions](<../spells/calm_emotions.md> "dnd2024:spells/calm_emotions") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Dispel Evil and Good](<../spells/dispel_evil_and_good.md> "dnd2024:spells/dispel_evil_and_good") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Entangle](<../spells/entangle.md> "dnd2024:spells/entangle") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Pass without Trace](<../spells/pass_without_trace.md> "dnd2024:spells/pass_without_trace") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Word of Recall](<../spells/word_of_recall.md> "dnd2024:spells/word_of_recall") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Unicorn s blessing

<!-- splat:yaml path=splat_rule/parameters/features/unicorn_s_blessing -->

```yaml
economy: bonus_action
resource:
  kind: daily
  uses: 3
  recovery: long_rest
operation: spell_choice
options:
- operation: cast_spell
  spell:
    key: dnd2024:spells/cure_wounds
- operation: cast_spell
  spell:
    key: dnd2024:spells/lesser_restoration
ability: charisma
targets:
  another_creature: true
  touch_with_horn: true
```

##### Charging horn

<!-- splat:yaml path=splat_rule/parameters/features/charging_horn -->

```yaml
economy: legendary_action
legendary_cost: 1
operation: sequence
effects:
- operation: move
  maximum_ft:
    multiply:
    - stat: speed
    - 0.5
  provoke_opportunity_attacks: false
- operation: attack_sequence
  feature: radiant_horn
  count: 1
```

##### Shimmering shield

<!-- splat:values path=splat_rule/parameters/features/shimmering_shield -->

| Field | Value |
| --- | --- |
| ` economy ` | ` legendary_action ` |
| ` legendary_cost ` | ` 1 ` |
| ` cooldown_until ` | ` self.next_turn_start ` |
| ` operation ` | ` grant_defense ` |
| ` ac_bonus ` | ` 2 ` |
| ` until ` | ` self.next_turn_end ` |

###### Targets

###### Any of

<!-- splat:table
path: splat_rule/parameters/features/shimmering_shield/targets/any_of
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Self:
    path: self
    type: boolean
    optional: true
  Kind:
    path: kind
    type: symbol
    optional: true
  Count:
    path: count
    type: integer
    optional: true
  Range ft:
    path: range_ft
    type: integer
    optional: true
  Visible:
    path: visible
    type: boolean
    optional: true
-->

| Key | Self | Kind | Count | Range ft | Visible |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | true |  |  |  |  |
| ` row_2 ` |  | ` Creature ` | ` 1 ` | ` 60 ` | true |

###### Temporary hp

<!-- splat:yaml path=splat_rule/parameters/features/shimmering_shield/temporary_hp -->

```yaml
roll:
  count: 3
  sides: 6
  modifier: 0
average: 10
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 13 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- celestial
- elvish
- sylvan
understands: []
telepathy_range_ft: 120
```

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
recovery: self.turn_start
trigger: other_creature.turn_end
cost_per_action: 1
requires:
  not_incapacitated: true
maximum_actions_per_trigger: 1
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
