---
splat_schema: 1
splat_id: 7365eef7-5968-589a-9784-55a7acda7ef8
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_tinker_s_tools_50_gp
splat_name: Tinker’s Tools (50 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 50
    unit: gp
  mass:
    value: 10
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_tinker_s_tools_50_gp
  implementation: dnd2024:equipment_artisan_s_tools_tinker_s_tools_50_gp
  parameters:
    tool:
      ability: dexterity
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: assemble_scrap_item
        check:
          ability: dexterity
          dc: 20
        size: tiny
        expires:
          value: 1
          unit: minute
        expiry: falls_apart
      craftable:
      - key: dnd2024:equipment/musket
      - key: dnd2024:equipment/pistol
      - key: dnd2024:equipment/bell_1_gp
      - key: dnd2024:equipment/lantern_bullseye_10_gp
      - key: dnd2024:equipment/flask_2_cp
      - key: dnd2024:equipment/lantern_hooded_5_gp
      - key: dnd2024:equipment/hunting_trap_5_gp
      - key: dnd2024:equipment/lock_10_gp
      - key: dnd2024:equipment/manacles_2_gp
      - key: dnd2024:equipment/mirror_5_gp
      - key: dnd2024:equipment/shovel_2_gp
      - key: dnd2024:equipment/signal_whistle_5_cp
      - key: dnd2024:equipment/tinderbox_5_sp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 10
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_tinker_s_tools_50_gp
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
  - 94
  engine_gaps: &id001
  - dnd2024:equipment_artisan_s_tools_tinker_s_tools_50_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 7c2c57ba0798358604d28391d58eda4cdac4f656836552e6dda444482ffffc3b
  parent_name: Artisan’s Tools
  line_span:
    first: 8081
    count: 7
splat_clauses:
- key: source_rules
  text: '# Tinker’s Tools (50 GP) Ability: Dexterity Weight: 10 lb. Utilize: Assemble a Tiny item composed
    of scrap, which falls apart in 1 minute (DC 20) Craft: Musket, Pistol, Bell, Bullseye Lantern, Flask,
    Hooded Lantern, Hunting Trap, Lock, Manacles, Mirror, Shovel, Signal Whistle, Tinderbox'
  sha256: 910287838996753f0d9503803b84c168dd473a9f781b1c883b4d84febc5dde6a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tinker’s Tools (50 GP)

<!-- SRD page 94 -->

Ability:  Dexterity  Weight: 10 lb.\
Utilize: Assemble a Tiny item composed of scrap, which\
falls apart in 1 minute (DC 20)\
Craft: Musket, Pistol, Bell, Bullseye Lantern, Flask,\
Hooded Lantern, Hunting Trap, Lock, Manacles, Mir-\
ror, Shovel, Signal Whistle, Tinderbox\
