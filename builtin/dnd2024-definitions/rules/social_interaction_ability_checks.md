---
splat_schema: 1
splat_id: 6f298547-ba5a-5ff4-ab6f-7ddf3ce6e04a
splat_kind: rule
splat_key: dnd2024:rules/social_interaction_ability_checks
splat_name: Ability Checks
splat_origin:
  key: dnd2024:reference/pages/011
splat_sources:
- title: SRD 5.2.1
  locator: pp. 11
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/social_interaction_ability_checks
  parameters:
    roleplaying_attitude_change:
      authority: gm
      attitude: choice
    uncertain_response:
      authority: gm
      roll_optional: true
      action:
        key: dnd2024:glossary/influence_action
  implementation: dnd2024:rules_social_interaction_ability_checks
splat_contracts:
- id: dnd2024:rules_social_interaction_ability_checks
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - attitude
  - check_request
  events: []
  capabilities:
  - social_interaction
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 11
  engine_gaps: &id001
  - dnd2024:rules_social_interaction_ability_checks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 492e0a169dbc90ee8fa26731fe68496e1ad0f84c0e42a9ce012705499b8748b5
  parent_name: Social Interaction
  line_span:
    first: 579
    count: 15
splat_clauses:
- key: source_rules
  text: '# Ability Checks Ability checks can be key in determining the outcome of a social interaction.
    Your roleplaying efforts can alter an NPC’s attitude, but there might still be an element of chance
    if the GM wants dice to play a role in determining an NPC’s response to you. In such situations, the
    GM will typically ask you to take the Influence action. Pay attention to your skill proficiencies
    when thinking of how you will interact with an NPC; use an approach that relies on your group’s skill
    proficiencies. For example, if the group needs to trick a guard into letting them into a castle, the
    Rogue who is proficient in Deception should lead the discussion.'
  sha256: c9a9d4a5955ca7e3486f3e165aa0750217398027dd014b9f4d9608fcb7261c96
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Checks

<!-- SRD page 11 -->

Ability checks can be key in determining the out-\
come of a social interaction. Your roleplaying efforts\
can alter an NPC’s attitude, but there might still be\
an element of chance if the GM wants dice to play\
a role in determining an NPC’s response to you. In\
such situations, the GM will typically ask you to\
take the Influence action.\
Pay attention to your skill proficiencies when\
thinking of how you will interact with an NPC; use\
an approach that relies on your group’s skill pro-\
ficiencies. For example, if the group needs to trick\
a guard into letting them into a castle, the Rogue\
who is proficient in Deception should lead the\
discussion.\
