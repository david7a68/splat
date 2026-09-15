---
splat_schema: 1
splat_id: dd0edd67-e5d3-5013-955d-b1161908efdf
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_2_paladin_s_smite
splat_name: 'Level 2: Paladin’s Smite'
splat_origin:
  key: dnd2024:reference/pages/054
splat_sources:
- title: SRD 5.2.1
  locator: pp. 54
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_2_paladin_s_smite
  parameters:
    always_prepared:
    - key: dnd2024:spells/divine_smite
    free_cast:
      spell:
        key: dnd2024:spells/divine_smite
      spell_slot_cost: 0
      uses:
        maximum: 1
        recover:
          event: rest.long_rest_completed
          amount: all
  implementation: dnd2024:class_features_paladin_level_2_paladin_s_smite
splat_contracts:
- id: dnd2024:class_features_paladin_level_2_paladin_s_smite
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  - spell_grants
  events:
  - spell.cast_pending
  - rest.long_rest_completed
  capabilities:
  - spellcasting
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 54
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_2_paladin_s_smite
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: bd466c2f21b415bf3d0ab64d57b1e3ef159b08543d0e2e2be8a7dc094cf70632
  parent_name: Paladin
  line_span:
    first: 4552
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 2: Paladin’s Smite You always have the Divine Smite spell prepared. In addition, you
    can cast it without expending a spell slot, but you must finish a Long Rest before you can cast it
    in this way again.'
  sha256: 3fa2d60fc3f2f0b0c3e6f4fffe132d421825c18a58339c4188c1677975736887
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Paladin’s Smite

<!-- SRD page 54 -->

You always have the Divine Smite spell prepared. In\
addition, you can cast it without expending a spell\
slot, but you must finish a Long Rest before you can\
cast it in this way again.\
