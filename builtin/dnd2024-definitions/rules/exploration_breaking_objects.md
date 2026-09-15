---
splat_schema: 1
splat_id: 46e1c93e-f0f1-5a9e-b49e-72bd046929ef
splat_kind: rule
splat_key: dnd2024:rules/exploration_breaking_objects
splat_name: Breaking Objects
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_breaking_objects
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/breaking_objects
    fragile_nonmagical:
      action_cost: 1
      automatically_destroy: true
    marching_order:
      ordered_participants: true
      change_allowed: outside_combat
      determines:
      - trap_targets
      - hidden_enemy_detection
      - starting_distances
  implementation: dnd2024:rule_exploration_breaking_objects
splat_contracts:
- id: dnd2024:rule_exploration_breaking_objects
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - objects
  - travel
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 12
  engine_gaps: &id001
  - dnd2024:rule_exploration_breaking_objects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 308249732b12866c082d803fbcc1abc3098f2e953088ff66088d4707d086646f
  parent_name: Exploration
  line_span:
    first: 696
    count: 16
splat_clauses:
- key: source_rules
  text: '# Breaking Objects As an action, you can automatically break or otherwise destroy a fragile,
    nonmagical object, such as a glass container or a piece of paper. If you try to damage something more
    resilient, the GM might use the rules on breaking objects in “Rules Glossary.” MarcHinG ordEr The
    adventurers should establish a marching order while they travel, whether indoors or outdoors. A marching
    order makes it easier to determine which characters are affected by traps, which ones can spot hidden
    enemies, and which ones are the closest to those enemies if a fight breaks out. You can change your
    marching order outside combat and record the order any way you like: write it down, for example, or
    arrange miniatures to show it.'
  sha256: dc0333f08b147b44c8666dd377387311a444b5da874778abe9e3cf1772dd0729
  status: contract_specified
splat_requires_mechanics: *id001
---

# Breaking Objects

<!-- SRD page 12 -->

As an action, you can automatically break or oth-\
erwise destroy a fragile, nonmagical object, such\
as a glass container or a piece of paper. If you try to\
damage something more resilient, the GM might use\
the rules on breaking objects in “Rules Glossary.”\
MarcHinG ordEr\
The adventurers should establish a marching order\
while they travel, whether indoors or outdoors. A\
marching order makes it easier to determine which\
characters are affected by traps, which ones can spot\
hidden enemies, and which ones are the closest to\
those enemies if a fight breaks out. You can change\
your marching order outside combat and record the\
order any way you like: write it down, for example,\
or arrange miniatures to show it.\
