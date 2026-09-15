---
splat_schema: 1
splat_id: f7dc75fc-f4ea-50ec-9325-11612921416a
splat_kind: rule
splat_key: dnd2024:rules/warlock_devouring_blade
splat_name: Devouring Blade
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_devouring_blade
  parameters:
    prerequisites:
      warlock_level_min: 12
      invocation:
        key: dnd2024:rules/warlock_thirsting_blade
    thirsting_blade_extra_attacks: 2
  implementation: dnd2024:rules_warlock_devouring_blade
splat_contracts:
- id: dnd2024:rules_warlock_devouring_blade
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - extra_attacks
  events: []
  capabilities:
  - character_features
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  engine_gaps: &id001
  - dnd2024:rules_warlock_devouring_blade
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 16f84341940285c521457e8f28921c10fdc5719a3597f3db2c327ae544431681
  parent_name: Warlock
  line_span:
    first: 6165
    count: 5
splat_clauses:
- key: source_rules
  text: '# Devouring Blade Prerequisite: Level 12+ Warlock, Thirsting Blade Invocation The Extra Attack
    of your Thirsting Blade invocation confers two extra attacks rather than one.'
  sha256: 1358db33b045e94e37eb17285688e62f4dc6e502f65f0db30891dd3e6a957e4a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Devouring Blade

<!-- SRD page 72 -->

Prerequisite: Level 12+ Warlock, Thirsting Blade\
Invocation\
The Extra Attack of your Thirsting Blade invocation\
confers two extra attacks rather than one.\
