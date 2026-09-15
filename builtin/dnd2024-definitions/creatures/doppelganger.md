---
splat_schema: 1
splat_id: 7a623dd0-f983-5f68-adfa-2b9262f43ec8
splat_kind: creature
splat_key: dnd2024:creatures/doppelganger
splat_name: Doppelganger
splat_origin:
  key: dnd2024:reference/creature/doppelganger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 280
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Neutral
  movement_text: 30 ft.
  skills_text: Deception +6, Insight +3
  immunities_text: Charmed
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: Common plus three other languages
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 52 (8d8 + 16)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 18
  save_dexterity:
    value: 4
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 14
  max_hp:
    value: 52
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/doppelganger
  implementation: dnd2024:creature_doppelganger
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: slam
          count: 2
        - operation: use_feature
          feature: unsettling_visage
          if_available: true
      slam:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
          advantage_if:
            combat_round: 1
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: bludgeoning
      read_thoughts:
        economy: action
        operation: cast_spell
        spell:
          key: dnd2024:spells/detect_thoughts
        ability: charisma
        save_dc: 12
        components_waived:
        - verbal
        - somatic
        - material
      unsettling_visage:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: wisdom
        dc: 12
        targets:
          kind: creature
          selection: each
          area:
            shape: emanation
            size_ft: 15
            origin: self
          can_see_self: true
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
        economy: bonus_action
        operation: shape_shift
        forms:
        - type: humanoid
          size:
          - medium
          - small
        - form: true_form
        statistics_preserved_except:
        - size
        equipment_transformed: false
    skill_totals:
      deception: 6
      insight: 3
    damage_immunities: []
    condition_immunities:
    - charmed
    senses:
      darkvision: 60
      passive_perception: 11
    languages:
      speaks:
      - common
      understands: []
      additional_languages:
        count: 3
        chooser: creator
        can_speak: true
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
- id: dnd2024:creature_doppelganger
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
  source_sha256: 6900469bc7a767c3fee52d9faf8ed9c91963bc953f5eade369ff4476a7e65083
  source_path: builtin/srd-5.2.1/creature/doppelganger.md
  pages:
  - 280
  engine_gaps: &id001
  - dnd2024:creature_doppelganger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: cf0bb6820c984d810a55e5694e0b15c3382e54da23bbb0a4352bd831ba9f3361
splat_clauses:
- key: source_rules
  text: '# Doppelganger Medium Monstrosity, Neutral AC 14 Initiative +4 (14) HP 52 (8d8 + 16) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 18 +4 +4 Con 14 +2 +2 Int 11 +0 +0 Wis 12 +1 +1 Cha
    14 +2 +2 Skills Deception +6, Insight +3 Immunities Charmed Senses Darkvision 60 ft.; Passive Perception
    11 Languages Common plus three other languages CR 3 (XP 700; PB +2) ### Actions Multiattack. The doppelganger
    makes two Slam attacks and uses Unsettling Visage if available. Slam. Melee Attack Roll: +6 (with
    Advantage during the first round of each combat), reach 5 ft. Hit: 11 (2d6 + 4) Bludgeoning damage.
    Read Thoughts. The doppelganger casts Detect Thoughts, requiring no spell components and using Charisma
    as the spellcasting ability (spell save DC 12). Unsettling Visage (Recharge 6). Wisdom Saving Throw:
    DC 12, each creature in a 15-foot Emanation originating from the doppelganger that can see the doppelganger.
    Failure: The target has the Frightened condition and repeats the save at the end of each of its turns,
    ending the effect on itself on a success. After 1 minute, it succeeds automatically. ### Bonus Actions
    Shape-Shift. The doppelganger shape-shifts into a Medium or Small Humanoid, or it returns to its true
    form. Its game statistics, other than its size, are the same in each form. Any equipment it is wearing
    or carrying isn’t transformed.'
  sha256: f77762b311e161587b48a7a2ffe310fe91cb87572062078343c0e9f20f259c67
  status: contract_specified
splat_requires_mechanics: *id001
---

# Doppelganger

<!-- SRD page 280 -->

Medium Monstrosity, Neutral\
AC 14 Initiative +4 (14)\
HP 52 (8d8 + 16)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 18 +4 +4 Con 14 +2 +2\
Int 11 +0 +0 Wis 12 +1 +1 Cha 14 +2 +2\
Skills Deception +6, Insight +3\
Immunities Charmed\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common plus three other languages\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The doppelganger makes two Slam attacks\
and uses Unsettling Visage if available.\
Slam. Melee Attack Roll: +6 (with Advantage during the\
first round of each combat), reach 5 ft. Hit: 11 (2d6 + 4)\
Bludgeoning damage.\
Read Thoughts. The doppelganger casts Detect\
Thoughts, requiring no spell components and using\
Charisma as the spellcasting ability (spell save DC 12).\
Unsettling Visage (Recharge 6). Wisdom Saving Throw:\
DC 12, each creature in a 15-foot Emanation originating\
from the doppelganger that can see the doppelganger.\
Failure: The target has the Frightened condition and re-\
peats the save at the end of each of its turns, ending the\
effect on itself on a success. After 1 minute, it succeeds\
automatically.\

### Bonus Actions

Shape-Shift. The doppelganger shape-shifts into a Me-\
dium or Small Humanoid, or it returns to its true form.\
Its game statistics, other than its size, are the same in\
each form. Any equipment it is wearing or carrying isn’t\
transformed.\

Original layout: [[../../srd-5.2.1/pages/280|page 280]].
