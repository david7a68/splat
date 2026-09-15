---
splat_schema: 1
splat_id: f704be79-b52a-5cf3-af24-407a3caf1ee4
splat_kind: rule
splat_key: dnd2024:glossary/high_jump
splat_name: High Jump
splat_origin:
  key: dnd2024:reference/glossary/high_jump
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/high_jump
  parameters:
    direction: vertical
    running_start:
      minimum_feet: 10
      on_foot: true
      immediately_before: true
    running_height_feet:
      op: max
      args:
      - 0
      - op: add
        args:
        - 3
        - var: strength_modifier
    standing_height_multiplier: 0.5
    movement_cost_per_foot: 1
    arms_extension_height_multiplier: 0.5
    maximum_reach:
      op: add
      args:
      - var: jump_height_feet
      - op: multiply
        args:
        - 1.5
        - var: creature_height_feet
  implementation: dnd2024:glossary_high_jump
splat_contracts:
- id: dnd2024:glossary_high_jump
  inputs:
    subject:
      type: participant
      required: true
    run_up_feet:
      type: number
      required: true
    on_foot:
      type: boolean
      required: true
    creature_height_feet:
      type: number
      required: true
  outputs:
  - jump_height
  - movement_cost
  - maximum_reach
  events: []
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 1369e4ceffecebc43bfc22c94dc260198cf4ecb6372427c44025db9e72e5bf8e
  source_path: builtin/srd-5.2.1/glossary/high_jump.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_high_jump
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 35d29a8bac2906bafe998bbe9cc5b0b77f2ad704722342ebe7e33b783a5fc002
splat_clauses:
- key: source_rules
  text: '# High Jump When you make a High Jump, you leap into the air a number of feet equal to 3 plus
    your Strength modifier (minimum of 0 feet) if you move at least 10 feet on foot immediately before
    the jump. When you make a standing High Jump, you can jump only half that distance. Either way, each
    foot of the jump costs a foot of movement. You can extend your arms half your height above yourself
    during the jump. Thus, you can reach a distance equal to the height of the jump plus 11/2 times your
    height.'
  sha256: 95b3556274fa5eb0ca63bc596f73537f4cadd34beb358909ac356877b51cb820
  status: contract_specified
splat_requires_mechanics: *id001
---

# High Jump

<!-- SRD page 183 -->

When you make a High Jump, you leap into the\
air a number of feet equal to 3 plus your Strength\
modifier (minimum of 0 feet) if you move at least\
10 feet on foot immediately before the jump. When\
you make a standing High Jump, you can jump only\
half that distance. Either way, each foot of the jump\
costs a foot of movement.\
You can extend your arms half your height above\
yourself during the jump. Thus, you can reach a dis-\
tance equal to the height of the jump plus 11/2 times\
your height.\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
