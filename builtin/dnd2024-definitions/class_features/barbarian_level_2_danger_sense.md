---
splat_schema: 1
splat_id: 6e678613-a17d-5a78-bd4a-867bb558c13f
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_2_danger_sense
splat_name: 'Level 2: Danger Sense'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_2_danger_sense
  parameters:
    roll: saving_throw
    ability: dexterity
    advantage: true
    unless_condition: incapacitated
  implementation: dnd2024:class_features_barbarian_level_2_danger_sense
splat_contracts:
- id: dnd2024:class_features_barbarian_level_2_danger_sense
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - saving_throw.pending
  capabilities:
  - character_features
  - effects
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_2_danger_sense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: b51df56fba7229ae3745695f249a50cb98d61727e920dc48fb02aae3cb95f55b
  parent_name: Barbarian
  line_span:
    first: 2336
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 2: Danger Sense You gain an uncanny sense of when things aren’t as they should be, giving
    you an edge when you dodge perils. You have Advantage on Dexterity saving throws unless you have the
    Incapacitated condition.'
  sha256: 4f13b8e7f8cb14852cf1389b8a3d065d18779e628e07f9f3fb58f9d185dfd3d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Danger Sense

<!-- SRD page 29 -->

You gain an uncanny sense of when things aren’t as\
they should be, giving you an edge when you dodge\
perils. You have Advantage on Dexterity saving\
throws unless you have the Incapacitated condition.\
