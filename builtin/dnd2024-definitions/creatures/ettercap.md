---
splat_schema: 1
splat_id: 18a69e7f-9d87-5f7d-aacb-add6f23f142d
splat_kind: creature
splat_key: dnd2024:creatures/ettercap
splat_name: Ettercap
splat_origin:
  key: dnd2024:reference/creature/ettercap
splat_sources:
- title: SRD 5.2.1
  locator: pp. 284
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Neutral Evil
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Perception +3, Stealth +4, Survival +3
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 44 (8d8 + 8)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 44
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ettercap
  implementation: dnd2024:creature_ettercap
  parameters:
    features:
      spider_climb:
        economy: trait
        operation: passive_rule
        climb_difficult_surfaces: true
        climb_ceilings: true
        ability_check_required: false
      web_walker:
        economy: trait
        operation: passive_rule
        ignore_web_movement_restrictions: true
        locate_creatures_on_same_web: true
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: bite
          count: 1
        - operation: attack_sequence
          feature: claw
          count: 1
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 6
              modifier: 2
            type: piercing
          - average: 2
            roll:
              count: 1
              sides: 4
              modifier: 0
            type: poison
          conditions:
          - condition: poisoned
            until: self.next_turn_start
      claw:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 7
            roll:
              count: 2
              sides: 4
              modifier: 2
            type: slashing
      web_strand:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: dexterity
        dc: 12
        targets:
          kind: creature
          count: 1
          range_ft: 30
          visible: true
          maximum_size: large
        failure:
          conditions:
          - condition: restrained
            until: web.destroyed
          web_object:
            ac: 10
            hp: 5
            damage_vulnerabilities:
            - fire
            damage_immunities:
            - bludgeoning
            - poison
            - psychic
        success:
          effects: []
      reel:
        economy: bonus_action
        operation: pull
        targets:
          kind: creature
          count: 1
          range_ft: 30
          restrained_by: self.web_strand
        maximum_ft: 25
        direction: straight_toward_self
    skill_totals:
      perception: 3
      stealth: 4
      survival: 3
    senses:
      darkvision: 60
      passive_perception: 13
    languages:
      speaks: []
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
- id: dnd2024:creature_ettercap
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
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: b75fc0d4d41faad976e36ec8dc3ef5f13343a4cdcdfb56693c30f02bb29f1529
  source_path: builtin/srd-5.2.1/creature/ettercap.md
  pages:
  - 284
  engine_gaps: &id001
  - dnd2024:creature_ettercap
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: bd2bbe9db273c0c2702493e3dadf8c8d5e716cac732e22b8c29b72dd85407d7d
splat_clauses:
- key: source_rules
  text: '# Ettercap Medium Monstrosity, Neutral Evil AC 13 Initiative +2 (12) HP 44 (8d8 + 8) Speed 30
    ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 15 +2 +2 Con 13 +1 +1 Int 7 -2 -2 Wis
    12 +1 +1 Cha 8 -1 -1 Skills Perception +3, Stealth +4, Survival +3 Senses Darkvision 60 ft.; Passive
    Perception 13 Languages None CR 2 (XP 450; PB +2) ### Traits Spider Climb. The ettercap can climb
    difficult surfaces, including along ceilings, without needing to make an ability check. Web Walker.
    The ettercap ignores movement restrictions caused by webs, and the ettercap knows the location of
    any other creature in contact with the same web. ### Actions Multiattack. The ettercap makes one Bite
    attack and one Claw attack. Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage
    plus 2 (1d4) Poison damage, and the target has the Poisoned condition until the start of the ettercap’s
    next turn. Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Slashing damage. Web Strand (Recharge
    5–6). Dexterity Saving Throw: DC 12, one Large or smaller creature the ettercap can see within 30
    feet. Failure: The target has the Restrained condition until the web is destroyed (AC 10; HP 5; Vulnerability
    to Fire damage; Immunity to Bludgeoning, Poison, and Psychic damage). ### Bonus Actions Reel. The
    ettercap pulls one creature within 30 feet of itself that is Restrained by its Web Strand up to 25
    feet straight toward itself.'
  sha256: 340c7999c90e7c32e94bfed26647347243078da3524ff7f2c377fc18eb10906f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ettercap

<!-- SRD page 284 -->

Medium Monstrosity, Neutral Evil\
AC 13  Initiative +2 (12)\
HP 44 (8d8 + 8)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 15 +2 +2 Con 13 +1 +1\
Int 7 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +3, Stealth +4, Survival +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Spider Climb. The ettercap can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Web Walker. The ettercap ignores movement re-\
strictions caused by webs, and the ettercap knows\
the location of any other creature in contact with\
the same web.\

### Actions

Multiattack. The ettercap makes one Bite attack and\
one Claw attack.\
Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage plus 2 (1d4) Poison damage, and the\
target has the Poisoned condition until the start of the\
ettercap’s next turn.\
Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2)\
Slashing damage.\
Web Strand (Recharge 5–6). Dexterity Saving Throw:\
DC 12, one Large or smaller creature the ettercap can\
see within 30 feet. Failure: The target has the Restrained\
condition until the web is destroyed (AC 10; HP 5;\
Vulnerability to Fire damage; Immunity to Bludgeoning,\
Poison, and Psychic damage).\

### Bonus Actions

Reel. The ettercap pulls one creature within 30 feet of\
itself that is Restrained by its Web Strand up to 25 feet\
straight toward itself.\

Original layout: [[../../srd-5.2.1/pages/284|page 284]].
