---
splat_schema: 1
splat_id: 148f9e56-912a-5c76-a42e-de36e6d7c9eb
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_5_stunning_strike
splat_name: 'Level 5: Stunning Strike'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_5_stunning_strike
  parameters:
    trigger:
      hit_creature: true
      attack_types:
      - monk_weapon
      - unarmed_strike
    limit:
      count: 1
      period: turn
    focus_cost: 1
    save:
      ability: constitution
      dc:
        op: add
        args:
        - 8
        - stat: abilities.wisdom.modifier
        - stat: proficiency_bonus
      failure:
        condition: stunned
        until: source.next_turn_start
      success:
        speed_multiplier: 0.5
        next_attack_against_target_advantage: true
        until: source.next_turn_start
  implementation: dnd2024:class_features_monk_level_5_stunning_strike
splat_contracts:
- id: dnd2024:class_features_monk_level_5_stunning_strike
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - resource_changes
  - conditions
  events:
  - attack.hit
  capabilities:
  - saving_throws
  - conditions
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_5_stunning_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 1bc687ba230e1a76b5180b09afb8386ae90a7565eebc1fe7db58483b92f2789a
  parent_name: Monk
  line_span:
    first: 4273
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 5: Stunning Strike Once per turn when you hit a creature with a Monk weapon or an Unarmed
    Strike, you can expend 1 Focus Point to attempt a stunning strike. The target must make a Constitution
    saving throw. On a failed save, the target has the Stunned condition until the start of your next
    turn. On a successful save, the target’s Speed is halved until the start of your next turn, and the
    next attack roll made against the target before then has Advantage.'
  sha256: 11ac12a97d8c7cd50ea1633f782d204b9741b971139ee3f8b6361aa61d622a4d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Stunning Strike

<!-- SRD page 51 -->

Once per turn when you hit a creature with a Monk\
weapon or an Unarmed Strike, you can expend 1\
Focus Point to attempt a stunning strike. The target\
must make a Constitution saving throw. On a failed\
save, the target has the Stunned condition until the\
start of your next turn. On a successful save, the\
target’s Speed is halved until the start of your next\
turn, and the next attack roll made against the tar-\
get before then has Advantage.\
