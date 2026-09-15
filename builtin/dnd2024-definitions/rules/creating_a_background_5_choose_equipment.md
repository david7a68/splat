---
splat_schema: 1
splat_id: 26ea07ed-dd70-53d2-909f-323829f0c8fb
splat_kind: rule
splat_key: dnd2024:rules/creating_a_background_5_choose_equipment
splat_name: '5: Choose Equipment'
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/creating_a_background_5_choose_equipment
  parameters:
    equipment_package:
      total_value_gp: 50
      unspent_gold_included: true
      excluded_categories:
      - martial_weapons
      - armor
  implementation: dnd2024:rules_creating_a_background_5_choose_equipment
splat_contracts:
- id: dnd2024:rules_creating_a_background_5_choose_equipment
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
  - 193
  engine_gaps: &id001
  - dnd2024:rules_creating_a_background_5_choose_equipment
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 35cad847a7c59a8bd58e410797ecaa95026d151cb6595db3712bb19ef83d9854
  parent_name: Creating a Background
  line_span:
    first: 9427
    count: 5
splat_clauses:
- key: source_rules
  text: '# 5: Choose Equipment Assemble a package of equipment worth 50 GP (including unspent gold). Don’t
    include Martial weapons or armor, as characters get them from their class choices.'
  sha256: b645fad983b43da1bd70a4dc5228cc802eb2a833019373081e6bdf08bb44ccbf
  status: contract_specified
splat_requires_mechanics: *id001
---

# 5: Choose Equipment

<!-- SRD page 193 -->

Assemble a package of equipment worth 50 GP (in-\
cluding unspent gold). Don’t include Martial weap-\
ons or armor, as characters get them from their\
class choices.\
