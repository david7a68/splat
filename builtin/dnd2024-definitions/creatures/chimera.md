---
splat_schema: 1
splat_id: 95b8c704-07ab-5065-a7da-d47075365b7e
splat_kind: creature
splat_key: dnd2024:creatures/chimera
splat_name: Chimera
splat_origin:
  key: dnd2024:reference/creature/chimera
splat_sources:
- title: SRD 5.2.1
  locator: pp. 273
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Chaotic Evil
  movement_text: 30 ft., Fly 60 ft.
  skills_text: Perception +8
  senses_text: Darkvision 60 ft.; Passive Perception 18
  languages_text: Understands Draconic but can’t speak
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 114 (12d10 + 48)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 19
  save_constitution:
    value: 4
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 14
  max_hp:
    value: 114
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/chimera
  implementation: dnd2024:creature_chimera
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: choose_by_predicate
          test:
            attack_had_advantage: true
          then:
          - op: damage
            average: 18
            amount:
              dice:
                count: 4
                sides: 6
              add: 4
            damage_type: piercing
          else:
          - op: damage
            average: 11
            amount:
              dice:
                count: 2
                sides: 6
              add: 4
            damage_type: piercing
        reach_feet: 5
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 6
            add: 4
          damage_type: slashing
        reach_feet: 5
      ram:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 10
          amount:
            dice:
              count: 1
              sides: 12
            add: 4
          damage_type: bludgeoning
        - op: if
          test:
            target_kind: creature
            target_size_max: medium
          then:
          - op: condition
            condition: prone
        reach_feet: 5
      fire_breath:
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
        ability: dexterity
        dc: 15
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 15
            origin: self
        failure:
        - op: damage
          average: 31
          amount:
            dice:
              count: 7
              sides: 8
            add: 0
          damage_type: fire
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: ram
          count: 1
        - op: use_feature
          feature: bite
          count: 1
        - op: choose
          choices:
          - op: use_feature
            feature: claw
            count: 1
          - op: use_feature
            feature: fire_breath
            count: 1
    skill_totals:
      perception: 8
    defenses: {}
    senses:
      passive_perception: 18
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks: []
      understands:
      - draconic
      can_speak: false
    stat_block_conventions:
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
splat_contracts:
- id: dnd2024:creature_chimera
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events:
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: a2a199eed3324a7fb37096d7661d3a65de8991771cacf4749a3ca62d68ab2592
  source_path: builtin/srd-5.2.1/creature/chimera.md
  pages:
  - 273
  engine_gaps: &id001
  - dnd2024:creature_chimera
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 958764f5b11b3e13717cac8936f71eaa7e544d5bb5aa951a1a98b0d8790867f4
splat_clauses:
- key: source_rules
  text: '# Chimera Large Monstrosity, Chaotic Evil AC 14 Initiative +0 (10) HP 114 (12d10 + 48) Speed
    30 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 11 +0 +0 Con 19 +4 +4 Int 3 -4 -4 Wis
    14 +2 +2 Cha 10 +0 +0 Skills Perception +8 Senses Darkvision 60 ft.; Passive Perception 18 Languages
    Understands Draconic but can’t speak CR 6 (XP 2,300; PB +3) ### Actions Multiattack. The chimera makes
    one Ram attack, one Bite attack, and one Claw attack. It can replace the Claw attack with a use of
    Fire Breath if available. Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (2d6 + 4) Piercing damage,
    or 18 (4d6 + 4) Piercing damage if the chimera had Advantage on the attack roll. Claw. Melee Attack
    Roll: +7, reach 5 ft. Hit: 7 (1d6 + 4) Slashing damage. Ram. Melee Attack Roll: +7, reach 5 ft. Hit:
    10 (1d12 + 4) Bludgeoning damage. If the target is a Medium or smaller creature, it has the Prone
    condition. Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 15, each creature in a 15-foot Cone.
    Failure: 31 (7d8) Fire damage. Success: Half damage.'
  sha256: 857c8b04121dd483df5ba0e3ab78ab033c088d9616e8771d89e38846d48f339e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chimera

<!-- SRD page 273 -->

Large Monstrosity, Chaotic Evil\
AC 14 Initiative +0 (10)\
HP 114 (12d10 + 48)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 11 +0 +0 Con 19 +4 +4\
Int 3 −4 −4 Wis 14 +2 +2 Cha 10 +0 +0\
Skills Perception +8\
Senses Darkvision 60 ft.; Passive Perception 18\
Languages Understands Draconic but can’t speak\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The chimera makes one Ram attack, one\
Bite attack, and one Claw attack. It can replace the\
Claw attack with a use of Fire Breath if available.\
Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (2d6 + 4)\
Piercing damage, or 18 (4d6 + 4) Piercing damage if the\
chimera had Advantage on the attack roll.\
Claw. Melee Attack Roll: +7, reach 5 ft. Hit: 7 (1d6 + 4)\
Slashing damage.\
Ram. Melee Attack Roll: +7, reach 5 ft. Hit: 10 (1d12\
+ 4) Bludgeoning damage. If the target is a Medium or\
smaller creature, it has the Prone condition.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 15, each creature in a 15-foot Cone. Failure: 31\
(7d8) Fire damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/273|page 273]].
