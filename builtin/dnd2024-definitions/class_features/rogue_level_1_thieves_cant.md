---
splat_schema: 1
splat_id: 68325514-4658-50b9-8cec-6c38bf89de8b
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_1_thieves_cant
splat_name: 'Level 1: Thieves’ Cant'
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_1_thieves_cant
  parameters:
    language_grants:
    - thieves_cant
    language_choice:
      count: 1
      source: character_creation_language_tables
  implementation: dnd2024:class_features_rogue_level_1_thieves_cant
splat_contracts:
- id: dnd2024:class_features_rogue_level_1_thieves_cant
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  events:
  - character.feature_gained
  capabilities:
  - languages
  - character_choices
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 62
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_1_thieves_cant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: a53e1942964168dac055b17294175d1642f2c2b069eb153fc42dfc988fc10019
  parent_name: Rogue
  line_span:
    first: 5208
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 1: Thieves’ Cant You picked up various languages in the communities where you plied your
    roguish talents. You know Thieves’ Cant and one other language of your choice, which you choose from
    the language tables in “Character Creation.”'
  sha256: 4dcf1602d4e8be43e4a028eae4fa23a8ebcc16c46655e8fec47f5612f83f81e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Thieves’ Cant

<!-- SRD page 62 -->

You picked up various languages in the commu-\
nities where you plied your roguish talents. You\
know Thieves’ Cant and one other language of your\
choice, which you choose from the language tables\
in “Character Creation.”\
