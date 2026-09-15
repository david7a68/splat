---
splat_schema: 1
splat_id: 0114a2f2-f02a-5702-801b-e1aef8c5824d
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_10_empowered_evocation
splat_name: 'Level 10: Empowered Evocation'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/evoker_level_10_empowered_evocation
  parameters:
    spell_class: wizard
    spell_school: evocation
    optional_bonus:
      stat: abilities.intelligence.modifier
    maximum_damage_rolls_per_spell: 1
  implementation: dnd2024:class_features_evoker_level_10_empowered_evocation
splat_contracts:
- id: dnd2024:class_features_evoker_level_10_empowered_evocation
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
  - damage
  events:
  - spell.damage_rolled
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 82
  engine_gaps: &id001
  - dnd2024:class_features_evoker_level_10_empowered_evocation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 16e32f789e5b3b587ea91a45e0259688c42d30471cb45ec211d09caa20fa1c24
  parent_name: Evoker
  line_span:
    first: 7042
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 10: Empowered Evocation Whenever you cast a Wizard spell from the Evocation school, you
    can add your Intelligence modifier to one damage roll of that spell.'
  sha256: 50f54615e5435acb61c392d4fc224918d32e8747c3d9145f7b51fa0e7efb8175
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Empowered Evocation

<!-- SRD page 82 -->

Whenever you cast a Wizard spell from the Evoca-\
tion school, you can add your Intelligence modifier\
to one damage roll of that spell.\
