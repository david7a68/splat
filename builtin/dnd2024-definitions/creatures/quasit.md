---
splat_schema: 1
splat_id: 85d3ac67-eb3a-59c1-9d26-3976f026a7be
splat_kind: creature
splat_key: dnd2024:creatures/quasit
splat_name: Quasit
splat_origin:
  key: dnd2024:reference/creature/quasit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 317
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 40 ft.
  skills_text: Stealth +5
  resistances_text: Cold, Fire, Lightning
  immunities_text: Poison; Poisoned
  senses_text: Darkvision 120 ft.; Passive Perception 10
  languages_text: Abyssal, Common
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 25 (10d4)
splat_stats:
  strength:
    value: 5
  save_strength:
    value: -3
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 25
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/quasit
  implementation: dnd2024:creature_quasit
  parameters:
    features:
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 4
              modifier: 3
            type: slashing
          conditions:
          - condition: poisoned
            until: self.next_turn_start
      invisibility:
        economy: action
        operation: cast_spell
        spell:
          key: dnd2024:spells/invisibility
        ability: charisma
        targets:
        - self
        components_waived:
        - verbal
        - somatic
        - material
      scare:
        economy: action
        resource:
          kind: daily
          uses: 1
          recovery: long_rest
        operation: saving_throw
        ability: wisdom
        dc: 10
        targets:
          kind: creature
          count: 1
          range_ft: 20
          visible: false
        failure:
          conditions:
          - condition: frightened
          repeat_save:
            event: target.turn_end
            success: end_effect
          automatic_success_after_seconds: 60
        success:
          effects: []
      shape_shift:
        economy: action
        operation: shape_shift
        forms:
        - form: bat
          speed_ft: 10
          fly_speed_ft: 40
        - form: centipede
          speed_ft: 40
          climb_speed_ft: 40
        - form: toad
          speed_ft: 40
          swim_speed_ft: 40
        - form: true_form
        statistics_preserved_except:
        - speed
        equipment_transformed: false
    skill_totals:
      stealth: 5
    damage_resistances:
    - cold
    - fire
    - lightning
    damage_immunities:
    - poison
    condition_immunities:
    - poisoned
    senses:
      darkvision: 120
      passive_perception: 10
    languages:
      speaks:
      - abyssal
      - common
      understands: []
    resolution_conventions:
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
splat_contracts:
- id: dnd2024:creature_quasit
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - turn.ended
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spellcasting
  - shape_shifting
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: d3f0e8633ee0152cd1d40f3f30b48377ba4c4c10acf95f9d940263dd4c81ce4a
  source_path: builtin/srd-5.2.1/creature/quasit.md
  pages:
  - 317
  engine_gaps: &id001
  - dnd2024:creature_quasit
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: e642c4766c49c98490bad82f47eea69f0f0d31505e81af3d75abb9739b579058
splat_clauses:
- key: source_rules
  text: '# Quasit Tiny Fiend (Demon), Chaotic Evil AC 13 Initiative +3 (13) HP 25 (10d4) Speed 40 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 5 -3 -3 Dex 17 +3 +3 Con 10 +0 +0 Int 7 -2 -2 Wis 10 +0 +0 Cha 10 +0
    +0 Skills Stealth +5 Resistances Cold, Fire, Lightning Immunities Poison; Poisoned Senses Darkvision
    120 ft.; Passive Perception 10 Languages Abyssal, Common CR 1 (XP 200; PB +2) ### Traits Magic Resistance.
    The quasit has Advantage on saving throws against spells and other magical effects. ### Actions Rend.
    Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Slashing damage, and the target has the Poisoned
    condition until the start of the quasit’s next turn. Invisibility. The quasit casts Invisibility on
    itself, requiring no spell components and using Charisma as the spellcasting ability. Scare (1/Day).
    Wisdom Saving Throw: DC 10, one creature within 20 feet. Failure: The target has the Frightened condition.
    At the end of each of its turns, the target repeats the save, ending the effect on itself on a success.
    After 1 minute, it succeeds automatically. Shape-Shift. The quasit shape-shifts to resemble a bat
    (Speed 10 ft., Fly 40 ft.), a centipede (40 ft., Climb 40 ft.), or a toad (40 ft., Swim 40 ft.), or
    it returns to its true form. Its game statistics are the same in each form, except for its Speed.
    Any equipment it is wearing or carrying isn’t transformed.'
  sha256: ec14f6280b686be4b336aa52d2d1474f4eaa1a47384e056f5fc1dc1176e6f889
  status: contract_specified
splat_requires_mechanics: *id001
---

# Quasit

<!-- SRD page 317 -->

Tiny Fiend (Demon), Chaotic Evil\
AC 13  Initiative +3 (13)\
HP 25 (10d4)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex 17 +3 +3 Con 10 +0 +0\
Int 7 −2 −2 Wis 10 +0 +0 Cha 10 +0 +0\
Skills Stealth +5\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Darkvision 120 ft.; Passive Perception 10\
Languages Abyssal, Common\
CR 1 (XP 200; PB +2)\

### Traits

Magic Resistance. The quasit has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Slashing damage, and the target has the Poisoned con-\
dition until the start of the quasit’s next turn.\
Invisibility. The quasit casts Invisibility on itself, requir-\
ing no spell components and using Charisma as the\
spellcasting ability.\
Scare (1/Day). Wisdom Saving Throw: DC 10, one\
creature within 20 feet. Failure: The target has the\
Frightened condition. At the end of each of its turns, the\
target repeats the save, ending the effect on itself on a\
success. After 1 minute, it succeeds automatically.\
Shape-Shift. The quasit shape-shifts to resemble a bat\
(Speed 10 ft., Fly 40 ft.), a centipede (40 ft., Climb 40\
ft.), or a toad (40 ft., Swim 40 ft.), or it returns to its\
true form. Its game statistics are the same in each form,\
except for its Speed. Any equipment it is wearing or\
carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/317|page 317]].
