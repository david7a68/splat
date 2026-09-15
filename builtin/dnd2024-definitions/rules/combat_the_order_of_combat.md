---
splat_schema: 1
splat_id: b54910bb-1adf-5923-8e2e-949f0a5e6630
splat_kind: rule
splat_key: dnd2024:rules/combat_the_order_of_combat
splat_name: The Order of Combat
splat_origin:
  key: dnd2024:reference/pages/013
splat_sources:
- title: SRD 5.2.1
  locator: pp. 13
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_the_order_of_combat
  parameters:
    round_seconds: 6
    turns_per_round_per_participant: 1
    initial_order: initiative
    repeat_rounds_until: side_defeated
  implementation: dnd2024:rule_combat_the_order_of_combat
splat_contracts:
- id: dnd2024:rule_combat_the_order_of_combat
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - turn_state
  events: []
  capabilities:
  - turns
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 13
  engine_gaps: &id001
  - dnd2024:rule_combat_the_order_of_combat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: f5e82b825a6eabb30874548308634d5ba3c60a58ed9750c8ed84dff486ad95c7
  parent_name: Combat
  line_span:
    first: 765
    count: 11
splat_clauses:
- key: source_rules
  text: '# The Order of Combat A typical combat encounter is a clash between two sides: a flurry of weapon
    swings, feints, parries, footwork, and spellcasting. The game organizes combat into a cycle of rounds
    and turns. A round represents about 6 seconds in the game world. During a round, each participant
    in a battle takes a turn. The order of turns is determined at the beginning of combat when everyone
    rolls Initiative. Once everyone has taken a turn, the fight continues to the next round if neither
    side is defeated.'
  sha256: 8288e3d668737e4e294051430e03f64108a78dd09968b800c27388529467cdc2
  status: contract_specified
splat_requires_mechanics: *id001
---

# The Order of Combat

<!-- SRD page 13 -->

A typical combat encounter is a clash between two\
sides: a flurry of weapon swings, feints, parries,\
footwork, and spellcasting. The game organizes\
combat into a cycle of rounds and turns. A round\
represents about 6 seconds in the game world.\
During a round, each participant in a battle takes a\
turn. The order of turns is determined at the begin-\
ning of combat when everyone rolls Initiative. Once\
everyone has taken a turn, the fight continues to the\
next round if neither side is defeated.\
