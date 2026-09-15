---
splat_schema: 1
splat_id: d7877970-e1f1-5007-9751-73a71408b6b9
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_senses
splat_name: Senses
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_senses
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/passive_perception
    - key: dnd2024:glossary/blindsight
    - key: dnd2024:glossary/darkvision
    - key: dnd2024:glossary/tremorsense
    - key: dnd2024:glossary/truesight
    value_source: stat_block_senses
    grant_only_listed_senses: true
  implementation: dnd2024:rule_parts_of_a_stat_block_senses
splat_contracts:
- id: dnd2024:rule_parts_of_a_stat_block_senses
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - monsters
  - senses
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rule_parts_of_a_stat_block_senses
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 12698b4a54d4781dc13e18bb9a25f4c876605231828be5c9e1b4f497385d017d
  parent_name: Parts of a Stat Block
  line_span:
    first: 11064
    count: 5
splat_clauses:
- key: source_rules
  text: '# Senses The Senses entry specifies a monster’s Passive Perception score, as well as any special
    senses the monster possesses. Passive Perception and special senses are described in “Rules Glossary.”'
  sha256: d944dbb7d28650aae74868078aef8efb6727f34e2a0768ce0775b785662c1191
  status: contract_specified
splat_requires_mechanics: *id001
---

# Senses

<!-- SRD page 256 -->

The Senses entry specifies a monster’s Passive\
Perception score, as well as any special senses the\
monster possesses. Passive Perception and special\
senses are described in “Rules Glossary.”\
