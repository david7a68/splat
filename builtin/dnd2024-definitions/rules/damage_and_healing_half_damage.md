---
splat_schema: 1
splat_id: 06788234-addb-5feb-a49c-755ef501e2d4
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_half_damage
splat_name: Half Damage
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_half_damage
  parameters:
    saving_throw_success_damage:
      op: floor
      args:
      - op: divide
        args:
        - var: failed_save_damage
        - 2
  implementation: dnd2024:rules_damage_and_healing_half_damage
splat_contracts:
- id: dnd2024:rules_damage_and_healing_half_damage
  inputs:
    failed_save_damage:
      type: integer
      required: true
  outputs:
  - damage
  events: []
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_half_damage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 7641e82e8e7789d917b16d3701631d0a9c439187cb1c1028a032b27e8e803baf
  parent_name: Damage and Healing
  line_span:
    first: 1160
    count: 5
splat_clauses:
- key: source_rules
  text: '# Half Damage Many saving throw effects deal half damage (round down) to a target when the target
    succeeds on the saving throw. The halved damage is equal to half the damage that would be dealt on
    a failed save.'
  sha256: 70228121782008902c6d9fc28fa7daf966c9e58f7e7bdf6825ea2b4bdbfe2f35
  status: contract_specified
splat_requires_mechanics: *id001
---

# Half Damage

<!-- SRD page 16 -->

Many saving throw effects deal half damage (round\
down) to a target when the target succeeds on the\
saving throw. The halved damage is equal to half\
the damage that would be dealt on a failed save.\
