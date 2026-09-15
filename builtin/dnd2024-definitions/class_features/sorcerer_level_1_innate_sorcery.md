---
splat_schema: 1
splat_id: 82874b69-d75c-52b7-8ba9-ee18236be272
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
splat_name: 'Level 1: Innate Sorcery'
splat_origin:
  key: dnd2024:reference/pages/065
splat_sources:
- title: SRD 5.2.1
  locator: pp. 65, 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_1_innate_sorcery
  parameters:
    action: bonus_action
    duration:
      minutes: 1
    uses:
      maximum: 2
      recover:
        event: rest.long_rest_completed
        amount: all
    benefits:
      sorcerer_spell_save_dc_bonus: 1
      sorcerer_spell_attack_advantage: true
  implementation: dnd2024:class_features_sorcerer_level_1_innate_sorcery
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_1_innate_sorcery
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  - spell_grants
  events:
  - action.bonus
  - spell.cast_pending
  capabilities:
  - spellcasting
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 65
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_1_innate_sorcery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 3b83cd2d86f30144234266049043ffbc3f838e9232849b247e3b09ee6705020b
  parent_name: Sorcerer
  line_span:
    first: 5534
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 1: Innate Sorcery An event in your past left an indelible mark on you, infusing you with
    simmering magic. As a Bonus Action, you can unleash that magic for 1 minute, during which you gain
    the following benefits: • The spell save DC of your Sorcerer spells increases by 1. • You have Advantage
    on the attack rolls of Sorcerer spells you cast. You can use this feature twice, and you regain all
    expended uses of it when you finish a Long Rest.'
  sha256: c654104b37b982c409229e8503606d7463b489bb9c1ce648266f3ff58c951d58
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Innate Sorcery

<!-- SRD page 65 -->

An event in your past left an indelible mark on you,\
infusing you with simmering magic. As a Bonus\
Action, you can unleash that magic for 1 minute,\
during which you gain the following benefits:\

<!-- SRD page 66 -->

•  The spell save DC of your Sorcerer spells\
increases by 1.\
•  You have Advantage on the attack rolls of\
Sorcerer spells you cast.\
You can use this feature twice, and you regain all\
expended uses of it when you finish a Long Rest.\
