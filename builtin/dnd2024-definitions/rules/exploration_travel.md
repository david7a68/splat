---
splat_schema: 1
splat_id: 04f1b738-4d86-580c-b194-c0193acaf6a2
splat_kind: rule
splat_key: dnd2024:rules/exploration_travel
splat_name: Travel
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_travel
  parameters:
    long_distance_resolution:
      authority: gm
      choices:
      - summarized
      - travel_pace
    time_critical_resolution: combat_movement
    travel_pace_rule:
      key: dnd2024:rules/exploration_travel_pace
  implementation: dnd2024:rules_exploration_travel
splat_contracts:
- id: dnd2024:rules_exploration_travel
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - travel_mode
  events: []
  capabilities:
  - travel
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 12
  engine_gaps: &id001
  - dnd2024:rules_exploration_travel
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 5c3f51eeedda227b3687d8491ea68e7a7fb67caff0ff3fe74c2e6dd88c12c76f
  parent_name: Exploration
  line_span:
    first: 718
    count: 9
splat_clauses:
- key: source_rules
  text: '# Travel During an adventure, the characters might travel long distances on trips that could
    take hours or days. The GM can summarize this travel without calculating exact distances or travel
    times, or the GM might have you use the travel pace rules below. If you need to know how fast you
    can move when every second matters, see the movement rules in “Combat” later in “Playing the Game.”'
  sha256: 8804241657c0efbc8bd795223382080a154c690bf9248162336e414e5e471175
  status: contract_specified
splat_requires_mechanics: *id001
---

# Travel

<!-- SRD page 12 -->

During an adventure, the characters might travel\
long distances on trips that could take hours or\
days. The GM can summarize this travel without\
calculating exact distances or travel times, or the\
GM might have you use the travel pace rules below.\
If you need to know how fast you can move when\
every second matters, see the movement rules in\
“Combat” later in “Playing the Game.”\
