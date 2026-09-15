---
splat_schema: 1
splat_id: b490eeaa-912b-5065-bd7b-111cab3d29cf
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_metamagic_options
splat_name: Metamagic Options
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_metamagic_options
  parameters:
    options:
    - key: dnd2024:rules/sorcerer_careful_spell
    - key: dnd2024:rules/sorcerer_distant_spell
    - key: dnd2024:rules/sorcerer_empowered_spell
    - key: dnd2024:rules/sorcerer_extended_spell
    - key: dnd2024:rules/sorcerer_heightened_spell
    - key: dnd2024:rules/sorcerer_quickened_spell
    - key: dnd2024:rules/sorcerer_seeking_spell
    - key: dnd2024:rules/sorcerer_subtle_spell
    - key: dnd2024:rules/sorcerer_transmuted_spell
    - key: dnd2024:rules/sorcerer_twinned_spell
  implementation: dnd2024:rules_sorcerer_metamagic_options
splat_contracts:
- id: dnd2024:rules_sorcerer_metamagic_options
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - metamagic_options
  events: []
  capabilities:
  - character_options
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 66
  engine_gaps: &id001
  - dnd2024:rules_sorcerer_metamagic_options
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8c71bb056b24183136975d07148cc96cdef0486175c1b5550187911bf8f9ee2b
  parent_name: Sorcerer
  line_span:
    first: 5628
    count: 4
splat_clauses:
- key: source_rules
  text: '# Metamagic Options The following options are available to your Metamagic feature. The options
    are presented in alphabetical order.'
  sha256: dda768bd0c2e80e3cf35e79939d23344450c4857c01d54eac6f7d65cbb55065f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Metamagic Options

<!-- SRD page 66 -->

The following options are available to your Meta-\
magic feature. The options are presented in alpha-\
betical order.\
