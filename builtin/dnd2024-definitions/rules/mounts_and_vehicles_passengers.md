---
splat_schema: 1
splat_id: 174efe28-e206-514c-b0c9-ead3d93990bc
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_passengers
splat_name: Passengers
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_passengers
  parameters:
    passenger_sizes:
    - small
    - medium
    hammock_capacity: vehicle_table_passengers
    private_capacity:
      op: floor
      args:
      - op: divide
        args:
        - variable: vehicle_table_passengers
        - 5
    typical_daily_prices:
      hammock:
        currency: sp
        amount: 5
      small_private_cabin:
        currency: gp
        amount: 2
    price_override:
      actor: gm
      scope: ship
  implementation: dnd2024:rule_mounts_and_vehicles_passengers
splat_contracts:
- id: dnd2024:rule_mounts_and_vehicles_passengers
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    vehicle_table_passengers:
      type: number
      required: true
  outputs:
  - vehicle_capacity
  events: []
  capabilities:
  - vehicles
  - economy
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  engine_gaps: &id001
  - dnd2024:rule_mounts_and_vehicles_passengers
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: a01e46f2cbd8567142e14d0307940f12af380d80437c59c201a9061a4142a57b
  parent_name: Mounts and Vehicles
  line_span:
    first: 8760
    count: 8
splat_clauses:
- key: source_rules
  text: '# Passengers The table lists the number of Small and Medium passengers the ship can accommodate
    using hammocks. A ship outfitted with private accommodations can carry one-fifth as many passengers.
    A passenger usually pays 5 SP per day for a hammock, but prices can vary from ship to ship. A small
    private cabin usually costs 2 GP per day.'
  sha256: ab8959a7b7e70c70c243c28b37f0c41c85839c7614ad0f56c619db8793093911
  status: contract_specified
splat_requires_mechanics: *id001
---

# Passengers

<!-- SRD page 101 -->

The table lists the number of Small and Medium\
passengers the ship can accommodate using ham-\
mocks. A ship outfitted with private accommoda-\
tions can carry one-fifth as many passengers. A pas-\
senger usually pays 5 SP per day for a hammock, but\
prices can vary from ship to ship. A small private\
cabin usually costs 2 GP per day.\
