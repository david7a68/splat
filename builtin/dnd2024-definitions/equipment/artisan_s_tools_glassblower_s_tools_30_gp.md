---
splat_schema: 1
splat_id: eca12d19-b2e3-5925-b1d6-0dda3bb92d61
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_glassblower_s_tools_30_gp
splat_name: Glassblower’s Tools (30 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 30
    unit: gp
  mass:
    value: 5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_glassblower_s_tools_30_gp
  implementation: dnd2024:equipment_artisan_s_tools_glassblower_s_tools_30_gp
  parameters:
    tool:
      ability: intelligence
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: identify_previous_contents
        check:
          ability: intelligence
          dc: 15
        material: glass
        lookback:
          value: 24
          unit: hour
      craftable:
      - key: dnd2024:equipment/bottle_glass_2_gp
      - key: dnd2024:equipment/magnifying_glass_100_gp
      - key: dnd2024:equipment/spyglass_1_000_gp
      - key: dnd2024:equipment/vial_1_gp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_glassblower_s_tools_30_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    operation:
      type: choice
      required: true
    target:
      required: false
      type: object
    choices:
      required: false
      type: choice
  outputs:
  - check_requests
  - tool_effects
  - crafting_options
  events:
  - action.utilize
  capabilities:
  - tool_proficiency
  - ability_checks
  - crafting
  - character_choices
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 93
  engine_gaps: &id001
  - dnd2024:equipment_artisan_s_tools_glassblower_s_tools_30_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 971130060a0aab59983df4cf3adcfbae1f8686c60f70608473c86af3eb9b6fc9
  parent_name: Artisan’s Tools
  line_span:
    first: 8044
    count: 5
splat_clauses:
- key: source_rules
  text: '# Glassblower’s Tools (30 GP) Ability: Intelligence Weight: 5 lb. Utilize: Discern what a glass
    object held in the past 24 hours (DC 15) Craft: Glass Bottle, Magnifying Glass, Spyglass, Vial'
  sha256: 63224f9800a19c81de6479db79cd2005e114c568479d38708f94928d24a3aafd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Glassblower’s Tools (30 GP)

<!-- SRD page 93 -->

Ability: Intelligence Weight: 5 lb.\
Utilize: Discern what a glass object held in the past 24\
hours (DC 15)\
Craft: Glass Bottle, Magnifying Glass, Spyglass, Vial\
