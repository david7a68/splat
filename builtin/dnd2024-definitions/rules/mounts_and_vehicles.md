---
splat_schema: 1
splat_id: 017c8afb-915f-514b-a8b7-592487ac13aa
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles
splat_name: Mounts and Vehicles
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles
  parameters:
    animal_statistics_source: monster_stat_block
    carrying_capacity_source: mounts_and_other_animals_table
    capacity_table_rule:
      key: dnd2024:rules/mounts_and_vehicles_saddles
  implementation: dnd2024:rules_mounts_and_vehicles
splat_contracts:
- id: dnd2024:rules_mounts_and_vehicles
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - mount_statistics
  events: []
  capabilities:
  - mounts
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 100
  engine_gaps: &id001
  - dnd2024:rules_mounts_and_vehicles
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d050f6fa37bec8db8a313716683e6acc9ad0c9843185688f8452d234e5850956
  parent_name: Mounts and Vehicles
  line_span:
    first: 8683
    count: 7
splat_clauses:
- key: source_rules
  text: '# Mounts and Vehicles A mount can help you move more quickly through the wilderness, but its
    primary purpose is to carry gear that would otherwise slow you down. The Mounts and Other Animals
    table shows each animal’s carrying capacity. See “Monsters” for the animals’ stat blocks.'
  sha256: ca839d96c7ba4992f5f22b5b131a9efe4e5928e0d2abba7212329e7f3de1dbe8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mounts and Vehicles

<!-- SRD page 100 -->

A mount can help you move more quickly through\
the wilderness, but its primary purpose is to carry\
gear that would otherwise slow you down. The\
Mounts and Other Animals table shows each ani-\
mal’s carrying capacity. See “Monsters” for the ani-\
mals’ stat blocks.\
