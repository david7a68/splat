---
splat_schema: 1
splat_id: 8454f260-b423-5188-80dc-5bf56ebb1537
splat_kind: rule
splat_key: dnd2024:glossary/reaction
splat_name: Reaction
splat_origin:
  key: dnd2024:reference/glossary/reaction
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/reaction
  parameters:
    trigger: reaction_description_defined
    own_or_other_turn: true
    own_turn_compatible_with:
    - action
    - bonus_action
    - both
    budget:
      maximum: 1
      reset: own_next_turn_start
    available_to_all:
      key: dnd2024:glossary/opportunity_attacks
  implementation: dnd2024:glossary_reaction
splat_contracts:
- id: dnd2024:glossary_reaction
  inputs:
    subject:
      type: participant
      required: true
    reaction:
      type: definition
      required: true
    trigger:
      type: event
      required: true
  outputs:
  - reaction_budget
  - eligibility
  events:
  - reaction.requested
  - turn.started
  capabilities:
  - action_economy
splat_conversion:
  status: specified
  category: glossary
  source_sha256: c96b0bdd490923d351c59483616674b8679e2c85d7772fa90f919651a339c806
  source_path: builtin/srd-5.2.1/glossary/reaction.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_reaction
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 1ed8c61fb18f83cf2c144125894691e9ce606f60891ab24035f8379bb74c4467
splat_clauses:
- key: source_rules
  text: '# Reaction A Reaction is a special action taken in response to a trigger defined in the Reaction’s
    description. You can take a Reaction on another creature’s turn, and if you take it on your turn,
    you can do so even if you also take an action, a Bonus Action, or both. Once you take a Reaction,
    you can’t take another one until the start of your next turn. The Opportunity Attack is a Reaction
    available to all creatures. See also “Opportunity Attacks” and “Playing the Game” (“Actions”).'
  sha256: 5f56852ce0e2d979eede243fcb2a59617244fa20feec44fbeaf33c4e6df239b1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reaction

<!-- SRD page 186 -->

A Reaction is a special action taken in response to\
a trigger defined in the Reaction’s description. You\
can take a Reaction on another creature’s turn, and\
if you take it on your turn, you can do so even if you\
also take an action, a Bonus Action, or both. Once\
you take a Reaction, you can’t take another one\
until the start of your next turn. The Opportunity\
Attack is a Reaction available to all creatures. See\
also “Opportunity Attacks” and “Playing the Game”\
(“Actions”).\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
