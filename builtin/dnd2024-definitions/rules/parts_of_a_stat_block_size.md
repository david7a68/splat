---
splat_schema: 1
splat_id: 0ecbedb2-9cb8-518b-927c-f760d9df468f
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_size
splat_name: Size
splat_origin:
  key: dnd2024:reference/pages/254
splat_sources:
- title: SRD 5.2.1
  locator: pp. 254
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_size
  parameters:
    size_options:
    - tiny
    - small
    - medium
    - large
    - huge
    - gargantuan
    multiple_stat_block_sizes:
      selector: gm
      constraint: listed_options
    size_mechanics:
      key: dnd2024:glossary/size
  implementation: dnd2024:rule_parts_of_a_stat_block_size
splat_contracts:
- id: dnd2024:rule_parts_of_a_stat_block_size
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - monster_parameters
  events: []
  capabilities:
  - monsters
  - statistics
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 254
  engine_gaps: &id001
  - dnd2024:rule_parts_of_a_stat_block_size
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: a94dc0f6b06f71ef78d11c25554b184f8bb376179b27d290354fad0dc2380943
  parent_name: Parts of a Stat Block
  line_span:
    first: 10889
    count: 5
splat_clauses:
- key: source_rules
  text: '# Size A monster is Tiny, Small, Medium, Large, Huge, or Gargantuan. If size options are presented,
    you choose the creature’s size from those options. See “Playing the Game” for information on size.'
  sha256: baa99c46a5241baf1eadcc371325dd5dc3320f1b4cc9f014aa4f9134b7e40da1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Size

<!-- SRD page 254 -->

A monster is Tiny, Small, Medium, Large, Huge,\
or Gargantuan. If size options are presented, you\
choose the creature’s size from those options. See\
“Playing the Game” for information on size.\
