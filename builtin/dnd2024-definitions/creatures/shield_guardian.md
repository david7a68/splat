---
splat_schema: 1
splat_id: f2cd7371-10dd-5eda-86b7-c87817a8a6df
splat_kind: creature
splat_key: dnd2024:creatures/shield_guardian
splat_name: Shield Guardian
splat_origin:
  key: dnd2024:reference/creature/shield_guardian
splat_sources:
- title: SRD 5.2.1
  locator: pp. 323
  url: https://www.dndbeyond.com/srd
---

# Shield Guardian

## Source text

<!-- SRD page 323 -->

Large Construct, Unaligned\
AC 17  Initiative −1 (9)\
HP 142 (15d10 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 8 −1 −1 Con 18 +4 +4\
Int 7 −2 −2 Wis 10 +0 +0 Cha 3 −4 −4\
Immunities Poison; Charmed, Exhaustion, Frightened,\
Paralyzed, Petrified, Poisoned\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 10\
Languages Understands commands given in any language\
but can’t speak\
CR 7 (XP 2,900; PB +3)\

### Traits

Bound. The guardian is magically bound to an amulet.\
While the guardian and its amulet are on the same\
plane of existence, the amulet’s wearer can telepathi-\
cally call the guardian to travel to it, and the guardian\
knows the distance and direction to the amulet. If the\
guardian is within 60 feet of the amulet’s wearer, half of\
any damage the wearer takes (round up) is transferred\
to the guardian.\
Regeneration. The guardian regains 10 Hit Points at the\
start of each of its turns if it has at least 1 Hit Point.\
Spell Storing. A spellcaster who wears the guardian’s\
amulet can cause the guardian to store one spell of\
level 4 or lower. To do so, the wearer must cast the\
spell on the guardian while within 5 feet of it. The\
spell has no effect but is stored within the guardian.\
Any previously stored spell is lost when a new spell is\
stored. The guardian can cast the spell stored with any\
parameters set by the original caster, requiring no spell\
components and using the caster’s spellcasting ability.\
The stored spell is then lost.\

### Actions

Multiattack. The guardian makes two Fist attacks.\
Fist. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6\
+ 4) Bludgeoning damage plus 7 (2d6) Force damage.\

### Reactions

Protection. Trigger: An attack roll hits the wearer of the\
guardian’s amulet while the wearer is within 5 feet of\
the guardian. Response: The wearer gains a +5 bonus\
to AC, including against the triggering attack and pos-\
sibly causing it to miss, until the start of the guardian’s\
next turn.\

Original layout: [[../../srd-5.2.1/pages/323|page 323]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` construct ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned ` |
| ` senses_text ` | ` Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Understands commands given in any language but can’t speak ` |
| ` cr_text ` | ` 7 (XP 2,900; PB +3) ` |
| ` hit_points_text ` | ` 142 (15d10 + 60) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 8 ` | ` -1 ` |
| ` Constitution ` | ` 18 ` | ` 4 ` |
| ` Intelligence ` | ` 7 ` | ` -2 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 142 ` |  |
| ` initiative ` | ` -1 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/shield_guardian ` |
| ` implementation ` | ` dnd2024:creature_shield_guardian ` |

### Parameters

#### Features

##### Fist

<!-- splat:yaml path=splat_rule/parameters/features/fist -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 6
    add: 4
  damage_type: bludgeoning
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: force
reach_feet: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: fist
  count: 2
```

##### Bound

<!-- splat:yaml path=splat_rule/parameters/features/bound -->

```yaml
economy: passive
bond:
  item: amulet
  magical: true
same_plane:
  wearer_can_telepathically_call_guardian: true
  guardian_knows_amulet:
  - distance
  - direction
damage_transfer:
  requires_wearer_within_feet: 60
  from: amulet_wearer
  to: self
  fraction:
    numerator: 1
    denominator: 2
    round: up
telepathic_call_effect:
  op: travel_to
  destination: amulet_wearer
```

##### Regeneration

<!-- splat:yaml path=splat_rule/parameters/features/regeneration -->

```yaml
economy: passive
trigger: self.turn.start
requires:
  hit_points_min: 1
op: heal
amount:
  constant: 10
```

##### Spell storing

<!-- splat:yaml path=splat_rule/parameters/features/spell_storing -->

```yaml
economy: passive
capacity: 1
store:
  actor: amulet_wearer
  actor_kind: spellcaster
  cast_target: self
  caster_distance_max_feet: 5
  spell_level_max: 4
  spell_effect: suppressed
  previous_spell: lost
release:
  op: cast_stored_spell
  parameters: set_by_original_caster
  ability: original_caster_spellcasting_ability
  components_waived:
  - verbal
  - somatic
  - material
  stored_spell_after_cast: lost
  economy: stored_spell_casting_time
```

##### Protection

<!-- splat:yaml path=splat_rule/parameters/features/protection -->

```yaml
economy: reaction
trigger: attack_roll.hits_amulet_wearer
requires:
  amulet_wearer_distance_max_feet: 5
targets: amulet_wearer
effects:
- op: modify_ac
  add: 5
  includes_triggering_attack: true
  reevaluate_hit: true
  ends:
    event: turn.start
    participant: self
    occurrence: next
```

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 10 ` |
| ` 1 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` understands ` | ` all ` |
| ` commands_only ` | ` true ` |
| ` can_speak ` | ` false ` |

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
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
