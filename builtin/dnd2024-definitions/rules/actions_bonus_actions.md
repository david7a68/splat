---
splat_schema: 1
splat_id: ba5b915c-9d78-54d3-9457-816e3aee8ffb
splat_kind: rule
splat_key: dnd2024:rules/actions_bonus_actions
splat_name: Bonus Actions
splat_origin:
  key: dnd2024:reference/pages/010
splat_sources:
- title: SRD 5.2.1
  locator: pp. 10
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/actions_bonus_actions
  parameters:
    per_turn_limit: 1
    requires_grant: true
    turn_scope: own
    timing: actor_choice_unless_specified
    blocked_by: cannot_take_actions
  implementation: dnd2024:rules_actions_bonus_actions
splat_contracts:
- id: dnd2024:rules_actions_bonus_actions
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events: []
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 10
  engine_gaps: &id001
  - dnd2024:rules_actions_bonus_actions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 34cdda7c6e86cdf27b0a86e17b882b4968c6658d8711a24a7493b7d386f15477
  parent_name: Actions
  line_span:
    first: 508
    count: 17
splat_clauses:
- key: source_rules
  text: '# Bonus Actions Various class features, spells, and other abilities let you take an additional
    action on your turn called a Bonus Action. The Cunning Action feature, for example, allows a Rogue
    to take a Bonus Action. You can take a Bonus Action only when a special ability, a spell, or another
    feature of the game states that you can do something as a Bonus Action. You otherwise don’t have a
    Bonus Action to take. You can take only one Bonus Action on your turn, so you must choose which Bonus
    Action to use if you have more than one available. You choose when to take a Bonus Action during your
    turn unless the Bonus Action’s timing is specified. Anything that deprives you of your ability to
    take actions also prevents you from taking a Bonus Action.'
  sha256: 469c4b419774cbf357bcb612c2b156f12d739114773b503fb3fa692ed1c1f39c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bonus Actions

<!-- SRD page 10 -->

Various class features, spells, and other abilities let\
you take an additional action on your turn called\
a Bonus Action. The Cunning Action feature, for\
example, allows a Rogue to take a Bonus Action. You\
can take a Bonus Action only when a special ability,\
a spell, or another feature of the game states that\
you can do something as a Bonus Action. You other-\
wise don’t have a Bonus Action to take.\
You can take only one Bonus Action on your turn,\
so you must choose which Bonus Action to use if you\
have more than one available.\
You choose when to take a Bonus Action during\
your turn unless the Bonus Action’s timing is spec-\
ified. Anything that deprives you of your ability to\
take actions also prevents you from taking a Bonus\
Action.\
