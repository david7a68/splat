---
splat_schema: 1
splat_id: dd1a9d3d-72b4-56d0-8aa3-de6cee106087
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_10_aura_of_courage
splat_name: 'Level 10: Aura of Courage'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_10_aura_of_courage
  parameters:
    aura: own_aura_of_protection
    targets:
    - self
    - allies
    condition_immunity: frightened
    existing_condition: suppressed_while_in_aura
  implementation: dnd2024:class_features_paladin_level_10_aura_of_courage
splat_contracts:
- id: dnd2024:class_features_paladin_level_10_aura_of_courage
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
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_10_aura_of_courage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f6b4ce06f51c5509e3113c76fc998505b2911967602eab5d3dc7bf960fdb2d00
  parent_name: Paladin
  line_span:
    first: 4637
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 10: Aura of Courage You and your allies have Immunity to the Frightened condition while
    in your Aura of Protection. If a Frightened ally enters the aura, that condition has no effect on
    that ally while there.'
  sha256: 1dbd77a2655c9692220358a32170e8d490024618009818629777c6ede740237f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Aura of Courage

<!-- SRD page 55 -->

You and your allies have Immunity to the Fright-\
ened condition while in your Aura of Protection. If\
a Frightened ally enters the aura, that condition has\
no effect on that ally while there.\
