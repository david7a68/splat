---
splat_schema: 1
splat_id: 8897075f-89e4-5187-bf75-0a6426ea2771
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_mounts_and_cargo
splat_name: Mounts and Cargo
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_mounts_and_cargo
  parameters:
    vehicle_kinds:
    - carriage
    - cart
    - chariot
    - sled
    - wagon
    maximum_pulled_weight:
      op: multiply
      args:
      - 5
      - op: sum
        args:
        - variable: pulling_animals_base_carrying_capacities
    vehicle_weight_counts: true
  implementation: dnd2024:rule_mounts_and_vehicles_mounts_and_cargo
splat_contracts:
- id: dnd2024:rule_mounts_and_vehicles_mounts_and_cargo
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    pulling_animals_base_carrying_capacities:
      type: resolution
      required: true
  outputs:
  - vehicle_capacity
  events: []
  capabilities:
  - vehicles
  - encumbrance
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 100
  engine_gaps: &id001
  - dnd2024:rule_mounts_and_vehicles_mounts_and_cargo
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 0a104e11f35c4ede3ca35d2aa0947a43209f4102706b51d298a517d85944fc3e
  parent_name: Mounts and Vehicles
  line_span:
    first: 8690
    count: 6
splat_clauses:
- key: source_rules
  text: '# Mounts and Cargo An animal pulling a carriage, cart, chariot, sled, or wagon can move weight
    up to five times its base carrying capacity, including the weight of the vehicle. If multiple animals
    pull the same vehicle, add their carrying capacities together.'
  sha256: 2cfac55522c405460fc8c1a8265497cd2a659425a20c0626feda1b5879111f43
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mounts and Cargo

<!-- SRD page 100 -->

An animal pulling a carriage, cart, chariot, sled, or\
wagon can move weight up to five times its base\
carrying capacity, including the weight of the vehi-\
cle. If multiple animals pull the same vehicle, add\
their carrying capacities together.\
