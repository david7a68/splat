---
splat_schema: 1
splat_id: 34922b69-57c3-5971-a0d5-45a3c6005c88
splat_kind: creature
splat_key: dnd2024:creatures/giant_ape
splat_name: Giant Ape
splat_origin:
  key: dnd2024:reference/creature/giant_ape
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 40 ft.
  skills_text: Athletics +9, Perception +4, Survival +4
  senses_text: Passive Perception 14
  languages_text: None
  cr_text: 7 (XP 2,900; PB +3)
  hit_points_text: 168 (16d12 + 64)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 168
  initiative:
    value: 5
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_ape
  implementation: dnd2024:creature_giant_ape
  parameters:
    features:
      fist:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 9
        targets:
          count: 1
        hit:
        - op: damage
          average: 22
          amount:
            dice:
              count: 3
              sides: 10
            add: 6
          damage_type: bludgeoning
        reach_feet: 10
      leap:
        economy: bonus_action
        op: jump
        distance_max_feet: 30
        movement_cost_feet: 10
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: fist
          count: 2
      boulder_toss:
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
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: dexterity
        dc: 17
        targets:
          kind: creature
          count: all
          area:
            shape: sphere
            size_feet: 5
            origin: chosen_point
            origin_visible_to: self
            origin_range_feet: 90
        failure:
        - op: damage
          average: 24
          amount:
            dice:
              count: 7
              sides: 6
            add: 0
          damage_type: bludgeoning
        - op: if
          test:
            target_size_max: large
          then:
          - op: condition
            condition: prone
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
    skill_totals:
      athletics: 9
      perception: 4
      survival: 4
    defenses: {}
    senses:
      passive_perception: 14
      special: []
    communication:
      speaks: []
      understands: []
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
- id: dnd2024:creature_giant_ape
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
  source_sha256: 724a7dd850b825f5cf12e657c21f86513f6b28114d5e2cc16a58447e0a7f2663
  source_path: builtin/srd-5.2.1/creature/giant_ape.md
  pages:
  - 349
  engine_gaps: &id001
  - dnd2024:creature_giant_ape
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 3f91490c7c140125044fe33eea816d13bef00672efba8bc453b30b2fd2754d4a
splat_clauses:
- key: source_rules
  text: '# Giant Ape Huge Beast, Unaligned AC 12 Initiative +5 (15) HP 168 (16d12 + 64) Speed 40 ft.,
    Climb 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 14 +2 +2 Con 18 +4 +4 Int 5 -3 -3 Wis 12
    +1 +1 Cha 7 -2 -2 Skills Athletics +9, Perception +4, Survival +4 Senses Passive Perception 14 Languages
    None CR 7 (XP 2,900; PB +3) ### Actions Multiattack. The ape makes two Fist attacks. Fist. Melee Attack
    Roll: +9, reach 10 ft. Hit: 22 (3d10 + 6) Bludgeoning damage. Boulder Toss (Recharge 6). The ape hurls
    a boulder at a point it can see within 90 feet. Dexterity Saving Throw: DC 17, each creature in a
    5-foot-radius Sphere centered on that point. Failure: 24 (7d6) Bludgeoning damage. If the target is
    a Large or smaller creature, it has the Prone condition. Success: Half damage only. ### Bonus Actions
    Leap. The ape jumps up to 30 feet by spending 10 feet of movement.'
  sha256: 3c98555d01ff5efd51ff4e6433af7ff735a2089d8f2453f8a9126ae41e79ba9d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Ape

<!-- SRD page 349 -->

Huge Beast, Unaligned\
AC 12 Initiative +5 (15)\
HP 168 (16d12 + 64)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 14 +2 +2 Con 18 +4 +4\
Int 5 −3 −3 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Athletics +9, Perception +4, Survival +4\
Senses Passive Perception 14\
Languages None\
CR 7 (XP 2,900; PB +3)\

### Actions

Multiattack. The ape makes two Fist attacks.\
Fist. Melee Attack Roll: +9, reach 10 ft. Hit: 22 (3d10\
+ 6) Bludgeoning damage.\
Boulder Toss (Recharge 6). The ape hurls a boulder\
at a point it can see within 90 feet. Dexterity Saving\
Throw: DC 17, each creature in a 5-foot-radius Sphere\
centered on that point. Failure: 24 (7d6) Bludgeoning\
damage. If the target is a Large or smaller creature, it\
has the Prone condition. Success: Half damage only.\

### Bonus Actions

Leap. The ape jumps up to 30 feet by spending 10 feet\
of movement.\

Original layout: [[../../srd-5.2.1/pages/349|page 349]].
