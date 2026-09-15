---
splat_schema: 1
splat_id: 632b3c3b-7ae6-5085-885d-26575d315b13
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_crew
splat_name: Crew
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_crew
  parameters:
    minimum_crew:
      airship: 10
      galley: 80
      keelboat: 1
      longship: 40
      rowboat: 1
      sailing_ship: 20
      warship: 60
    skilled_hirelings_required_except:
    - keelboat
    - rowboat
  implementation: dnd2024:rules_mounts_and_vehicles_crew
splat_contracts:
- id: dnd2024:rules_mounts_and_vehicles_crew
  inputs:
    vehicle:
      type: object
      required: true
    crew:
      type: participant_list
      required: true
  outputs:
  - operational
  events: []
  capabilities:
  - vehicles
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  engine_gaps: &id001
  - dnd2024:rules_mounts_and_vehicles_crew
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 9172399da00d714e809f4d1921306783df99e4b02dd97a1b4634a6f662256f87
  parent_name: Mounts and Vehicles
  line_span:
    first: 8754
    count: 6
splat_clauses:
- key: source_rules
  text: '# Crew A ship larger than a Keelboat or Rowboat needs a crew of skilled hirelings (see “Hirelings”
    later in “Equipment”) to function. The minimum number of skilled hirelings needed to crew a ship depends
    on the type of ship, as shown in the table.'
  sha256: 77ff514d90e5ff02e5974e4d7f063997d0b97f96a9d046730f2f76c13b3b14d3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crew

<!-- SRD page 101 -->

A ship larger than a Keelboat or Rowboat needs a\
crew of skilled hirelings (see “Hirelings” later in\
“Equipment”) to function. The minimum number of\
skilled hirelings needed to crew a ship depends on\
the type of ship, as shown in the table.\
