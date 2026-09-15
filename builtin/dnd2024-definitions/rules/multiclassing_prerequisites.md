---
splat_schema: 1
splat_id: 752ca5af-c9f9-50d5-827f-6c64627e1d8f
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_prerequisites
splat_name: Prerequisites
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24, 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_prerequisites
  parameters:
    minimum_primary_ability_score: 13
    classes_to_check: new_and_all_current
    primary_abilities:
      source:
        key: dnd2024:rules/create_your_character_step_1_choose_class
    interleaved_starting_equipment_table:
      source:
        key: dnd2024:rules/starting_at_higher_levels_creating_your_character
      field: starting_equipment_table
  implementation: dnd2024:rule_multiclassing_prerequisites
splat_contracts:
- id: dnd2024:rule_multiclassing_prerequisites
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - multiclassing
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 24
  - 25
  engine_gaps: &id001
  - dnd2024:rule_multiclassing_prerequisites
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: b86a2ac60d8abda35fcb8bbdf9260747ddb96160591444c45f4f3126b8155e12
  parent_name: Multiclassing
  line_span:
    first: 1902
    count: 13
splat_clauses:
- key: source_rules
  text: '# Prerequisites To qualify for a new class, you must have a score of at least 13 in the primary
    ability of the new class and your current classes. For example, a Barbarian who decides to multiclass
    into the Druid class must have Strength and Wisdom scores of 13 or higher, Magic Items 1 Common 1
    Common, 1 Uncommon 2 Common, 3 Uncommon, 1 Rare 2 Common, 4 Uncommon, 3 Rare, 1 Very Rare since Strength
    is the primary ability for Barbarians and Wisdom is the primary ability for Druids.'
  sha256: d825591c79773b3fc97ad19d729e26117a8efb3126710b7c2d3a1ce51421bf20
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prerequisites

<!-- SRD page 24 -->

To qualify for a new class, you must have a score of\
at least 13 in the primary ability of the new class\
and your current classes. For example, a Barbarian\
who decides to multiclass into the Druid class must\
have Strength and Wisdom scores of 13 or higher,\
Magic Items\
1 Common\
1 Common, 1 Uncommon\
2 Common, 3 Uncommon, 1 Rare\
2 Common, 4 Uncommon, 3 Rare, 1 Very Rare\

<!-- SRD page 25 -->

since Strength is the primary ability for Barbarians\
and Wisdom is the primary ability for Druids.\
