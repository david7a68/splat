---
splat_schema: 1
splat_id: 7881a225-a35d-5332-b737-862468563258
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion
splat_name: 'Level 7: Aura of Devotion'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion
  parameters:
    aura: own_aura_of_protection
    targets:
    - self
    - allies
    condition_immunity: charmed
    existing_condition: suppressed_while_in_aura
  implementation: dnd2024:class_features_oath_of_devotion_level_7_aura_of_devotion
splat_contracts:
- id: dnd2024:class_features_oath_of_devotion_level_7_aura_of_devotion
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
  - conditions
  events:
  - aura.membership_changed
  - condition.pending
  capabilities:
  - auras
  - conditions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 56
  engine_gaps: &id001
  - dnd2024:class_features_oath_of_devotion_level_7_aura_of_devotion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: ab0ff34a57f9cceb5286f759f19b42963a41aad9366d79344f1bc2298ac460e0
  parent_name: Oath of Devotion
  line_span:
    first: 4763
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 7: Aura of Devotion You and your allies have Immunity to the Charmed condition while
    in your Aura of Protection. If a Charmed ally enters the aura, that condition has no effect on that
    ally while there.'
  sha256: 59abc7a752d6818b5114149524466e9ae22f69491025716fdc225e612cc9ff0e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Aura of Devotion

<!-- SRD page 56 -->

You and your allies have Immunity to the Charmed\
condition while in your Aura of Protection. If a\
Charmed ally enters the aura, that condition has no\
effect on that ally while there.\
