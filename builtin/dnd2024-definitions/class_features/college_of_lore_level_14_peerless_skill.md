---
splat_schema: 1
splat_id: fea3cc65-4c97-55ba-aafc-13fe7e008ff5
splat_kind: rule
splat_key: dnd2024:class_features/college_of_lore_level_14_peerless_skill
splat_name: 'Level 14: Peerless Skill'
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/college_of_lore_level_14_peerless_skill
  parameters:
    trigger:
    - actor.ability_check_failed
    - actor.attack_roll_failed
    resource_cost:
      id: bard.bardic_inspiration
      amount: 1
    roll:
      var: actor.bardic_inspiration_die
    modify: add_to_d20
    recompute_success: true
    refund_on_failure: true
    class_scope:
      class: bard
      minimum_level: 14
  implementation: dnd2024:class_features_college_of_lore_level_14_peerless_skill
splat_contracts:
- id: dnd2024:class_features_college_of_lore_level_14_peerless_skill
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - ability.check_failed
  - attack.roll_failed
  capabilities:
  - character_features
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 35
  engine_gaps: &id001
  - dnd2024:class_features_college_of_lore_level_14_peerless_skill
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: df3a5e20c3797ed0b8eeb3f092c3f8940942f9fbf4f6314f7124c36642a25d6e
  parent_name: College of Lore
  line_span:
    first: 2906
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 14: Peerless Skill When you make an ability check or attack roll and fail, you can expend
    one use of Bardic Inspiration; roll the Bardic Inspiration die, and add the number rolled to the d20,
    potentially turning a failure into a success. On a failure, the Bardic Inspiration isn’t expended.'
  sha256: 73429a695f5ee9f6f82b5297e62072016ff0abcbd815d1d4b2cb87c129602ff9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Peerless Skill

<!-- SRD page 35 -->

When you make an ability check or attack roll and\
fail, you can expend one use of Bardic Inspiration;\
roll the Bardic Inspiration die, and add the number\
rolled to the d20, potentially turning a failure into\
a success. On a failure, the Bardic Inspiration isn’t\
expended.\
