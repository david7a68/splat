---
splat_schema: 1
splat_id: 9b36e628-5e18-5aa9-a75c-932262a364be
splat_kind: rule
splat_key: dnd2024:rules/character_backgrounds_feat
splat_name: Feat
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_backgrounds_feat
  parameters:
    grant: background_specified_origin_feat
  implementation: dnd2024:rule_character_backgrounds_feat
splat_contracts:
- id: dnd2024:rule_character_backgrounds_feat
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
  - feats
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  engine_gaps: &id001
  - dnd2024:rule_character_backgrounds_feat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 930041aaf457b89de0c2e4a22cd7bb33222ba93dd112f1568f3058ea9e0de68f
  parent_name: Character Backgrounds
  line_span:
    first: 7072
    count: 3
splat_clauses:
- key: source_rules
  text: '# Feat A background gives your character a specified Origin feat (described in “Feats”).'
  sha256: 05bf2a5c29b067f7bfa5a33961b4f1e615c81bedbaea216d2aeda5836cf04c10
  status: contract_specified
splat_requires_mechanics: *id001
---

# Feat

<!-- SRD page 83 -->

A background gives your character a specified Ori-\
gin feat (described in “Feats”).\
