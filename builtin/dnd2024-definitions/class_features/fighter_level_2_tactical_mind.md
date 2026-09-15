---
splat_schema: 1
splat_id: 43a06bd9-351e-5d13-b348-9e20aca36ec0
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_2_tactical_mind
splat_name: 'Level 2: Tactical Mind'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_2_tactical_mind
  parameters:
    trigger: actor.ability_check_failed
    resource_cost:
      id: fighter.second_wind
      amount: 1
    roll:
      count: 1
      sides: 10
    modify: add_to_d20
    recompute_success: true
    refund_on_failure: true
    healing: false
    class_scope:
      class: fighter
      minimum_level: 2
  implementation: dnd2024:class_features_fighter_level_2_tactical_mind
splat_contracts:
- id: dnd2024:class_features_fighter_level_2_tactical_mind
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
  capabilities:
  - character_features
  - resources
  - healing
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_2_tactical_mind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 0fa31777bad39f5311303bd534cf8e499c9b524adbaf9943c3091f887de98a1a
  parent_name: Fighter
  line_span:
    first: 3993
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 2: Tactical Mind You have a mind for tactics on and off the battlefield. When you fail
    an ability check, you can expend a use of your Second Wind to push yourself toward success. Rather
    than regaining Hit Points, you roll 1d10 and add the number rolled to the ability check, potentially
    turning it into a success. If the check still fails, this use of Second Wind isn’t expended.'
  sha256: b8fffb3863e7ddb6ac9f14af0b4226ded7b88314b86eea955670ba28a2a8d680
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Tactical Mind

<!-- SRD page 48 -->

You have a mind for tactics on and off the battle-\
field. When you fail an ability check, you can expend\
a use of your Second Wind to push yourself toward\
success. Rather than regaining Hit Points, you roll\
1d10 and add the number rolled to the ability check,\
potentially turning it into a success. If the check still\
fails, this use of Second Wind isn’t expended.\
