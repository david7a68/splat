---
splat_schema: 1
splat_id: 55b31e22-129b-58eb-815a-62c84014c39c
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_proficiency_bonus
splat_name: Proficiency Bonus
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_proficiency_bonus
  parameters:
    proficiency_bonus_level_source: sum_all_class_levels
    proficiency_bonus:
      op: add
      args:
      - 2
      - op: floor
        args:
        - op: divide
          args:
          - op: subtract
            args:
            - variable: total_level
            - 1
          - 4
  implementation: dnd2024:rule_multiclassing_proficiency_bonus
splat_contracts:
- id: dnd2024:rule_multiclassing_proficiency_bonus
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    total_level:
      type: number
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - multiclassing
  - statistics
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 25
  engine_gaps: &id001
  - dnd2024:rule_multiclassing_proficiency_bonus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: e6a26add7e8d3df13d0d47b85fab0af19e05c0632ba6b2d0e3f92e0ceaba48d1
  parent_name: Multiclassing
  line_span:
    first: 1939
    count: 7
splat_clauses:
- key: source_rules
  text: '# Proficiency Bonus Your Proficiency Bonus is based on your total character level, not your level
    in a particular class, as shown in the Character Advancement table. For example, if you are a level
    3 Fighter / level 2 Rogue, you have the Proficiency Bonus of a level 5 character, which is +3.'
  sha256: 118200944337c15e2e8bb3a47d712a0706d6ef2ac5b26402a70e5b85632a79bf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Proficiency Bonus

<!-- SRD page 25 -->

Your Proficiency Bonus is based on your total char-\
acter level, not your level in a particular class, as\
shown in the Character Advancement table. For\
example, if you are a level 3 Fighter / level 2 Rogue,\
you have the Proficiency Bonus of a level 5 charac-\
ter, which is +3.\
