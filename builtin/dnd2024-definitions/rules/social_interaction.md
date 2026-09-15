---
splat_schema: 1
splat_id: 0ceb2480-62d9-5c92-819c-c28f6ff8a286
splat_kind: rule
splat_key: dnd2024:rules/social_interaction
splat_name: Social Interaction
splat_origin:
  key: dnd2024:reference/pages/010
splat_sources:
- title: SRD 5.2.1
  locator: pp. 10
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/social_interaction
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/friendly_attitude
    - key: dnd2024:glossary/indifferent_attitude
    - key: dnd2024:glossary/hostile_attitude
    npc_controller: gm
    resolution_methods:
    - roleplay
    - ability_check
  implementation: dnd2024:rule_social_interaction
splat_contracts:
- id: dnd2024:rule_social_interaction
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
  - social_interaction
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 10
  engine_gaps: &id001
  - dnd2024:rule_social_interaction
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: d79dfcbdb5845e39e10ce0e886a6a72ec2966f3c2699fd6f03fb672492229ed2
  parent_name: Social Interaction
  line_span:
    first: 540
    count: 15
splat_clauses:
- key: source_rules
  text: '# Social Interaction During their adventures, player characters meet many different people and
    face some monsters that would rather talk than fight. In those situations, it’s time for social interaction,
    which takes many forms. For example, you might try to convince a burglar to confess to wrongdoing
    or try to flatter a guard. The Game Master assumes the roles of any nonplayer characters who are participating.
    An NPC’s attitude toward your character is Friendly, Indifferent, or Hostile, as defined in “Rules
    Glossary.” Friendly NPCs are predisposed to help, and Hostile ones are inclined to hinder. Social
    interactions progress in two ways: through roleplaying and ability checks.'
  sha256: 6daa9a52ce65079c32316647791f375ee27a03e2cf3be978d1b028ed91473dcf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Social Interaction

<!-- SRD page 10 -->

During their adventures, player characters meet\
many different people and face some monsters that\
would rather talk than fight. In those situations, it’s\
time for social interaction, which takes many forms.\
For example, you might try to convince a burglar to\
confess to wrongdoing or try to flatter a guard. The\
Game Master assumes the roles of any nonplayer\
characters who are participating.\
An NPC’s attitude toward your character is\
Friendly, Indifferent, or Hostile, as defined in “Rules\
Glossary.” Friendly NPCs are predisposed to help,\
and Hostile ones are inclined to hinder.\
Social interactions progress in two ways: through\
roleplaying and ability checks.\
