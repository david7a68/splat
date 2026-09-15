---
splat_schema: 1
splat_id: 261ca738-5b92-5fb3-9b25-4b24aa4bdbdf
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_proficiencies
splat_name: Proficiencies
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_proficiencies
  parameters:
    new_class_first_level:
      grant: class_multiclass_proficiencies_only
      exclude: other_starting_proficiencies
      source: chosen_class_definition
  implementation: dnd2024:rules_multiclassing_proficiencies
splat_contracts:
- id: dnd2024:rules_multiclassing_proficiencies
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 25
  engine_gaps: &id001
  - dnd2024:rules_multiclassing_proficiencies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 05655cceaf7cb58371b1f8c084737250231a63c97c673ff6fcf6103880708995
  parent_name: Multiclassing
  line_span:
    first: 1946
    count: 5
splat_clauses:
- key: source_rules
  text: '# Proficiencies When you gain your first level in a class other than your initial class, you
    gain only some of the new class’s starting proficiencies, as detailed in each class’s description
    in “Classes.”'
  sha256: fc444c8f6a1a3c4c76657f6ddd76c2a593c94fed847302e892a4b1d038c5259d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Proficiencies

<!-- SRD page 25 -->

When you gain your first level in a class other than\
your initial class, you gain only some of the new\
class’s starting proficiencies, as detailed in each\
class’s description in “Classes.”\
