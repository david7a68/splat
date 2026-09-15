---
splat_schema: 1
splat_id: 59ec6b04-1e2e-5008-b0d9-b9324dd93e0e
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_ship_repair
splat_name: Ship Repair
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_ship_repair
  parameters:
    requires_berthed: true
    per_hp:
      days: 1
      cost_gp: 20
    abundant_supplies_and_skilled_labor:
      time_multiplier: 0.5
      cost_multiplier: 0.5
  implementation: dnd2024:rules_mounts_and_vehicles_ship_repair
splat_contracts:
- id: dnd2024:rules_mounts_and_vehicles_ship_repair
  inputs:
    ship:
      type: object
      required: true
    hp_to_repair:
      type: integer
      required: true
    abundant_resources:
      type: boolean
      required: true
  outputs:
  - repair_time
  - repair_cost
  - hit_points
  events: []
  capabilities:
  - crafting
  - vehicles
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  engine_gaps: &id001
  - dnd2024:rules_mounts_and_vehicles_ship_repair
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 21df8ecca116025b3d0abda244fc473b2d44811fb70a2cb9670cdab6d16e5570
  parent_name: Mounts and Vehicles
  line_span:
    first: 8771
    count: 7
splat_clauses:
- key: source_rules
  text: '# Ship Repair Repairs to a damaged ship can be made while the vessel is berthed. Repairing 1
    Hit Point of damage requires 1 day and costs 20 GP for materials and labor. If the repairs are made
    in a location where supplies and skilled labor are abundant, such as a city shipyard, the repair time
    and cost are halved.'
  sha256: f310dd7a9b151ad7007b354399a0c543c0332ddda2215480b7792d094169ba90
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ship Repair

<!-- SRD page 101 -->

Repairs to a damaged ship can be made while the\
vessel is berthed. Repairing 1 Hit Point of damage\
requires 1 day and costs 20 GP for materials and la-\
bor. If the repairs are made in a location where sup-\
plies and skilled labor are abundant, such as a city\
shipyard, the repair time and cost are halved.\
