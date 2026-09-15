---
splat_schema: 1
splat_id: cb4d373c-9890-5a96-ab17-86bf65a3f03b
splat_kind: rule
splat_key: dnd2024:rules/actions_reactions
splat_name: Reactions
splat_origin:
  key: dnd2024:reference/pages/010
splat_sources:
- title: SRD 5.2.1
  locator: pp. 10
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/actions_reactions
  parameters:
    requires_grant_and_trigger: true
    allowed_turn: any
    uses:
      max: 1
      reset: subject_turn_start
    default_timing: immediately_after_trigger
    specific_timing_overrides_default: true
    interrupted_turn: resume_after_reaction
  implementation: dnd2024:rules_actions_reactions
splat_contracts:
- id: dnd2024:rules_actions_reactions
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - reaction.triggered
  - turn.started
  capabilities:
  - action_economy
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 10
  engine_gaps: &id001
  - dnd2024:rules_actions_reactions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d5bb158210e47547cbcc784a6f42c2da378a80c89784b4d8404f0bf16072d32b
  parent_name: Actions
  line_span:
    first: 525
    count: 15
splat_clauses:
- key: source_rules
  text: '# Reactions Certain special abilities, spells, and situations allow you to take a special action
    called a Reaction. A Reaction is an instant response to a trigger of some kind, which can occur on
    your turn or on someone else’s. The Opportunity Attack, described later in “Playing the Game,” is
    the most common type of Reaction. When you take a Reaction, you can’t take another one until the start
    of your next turn. If the reaction interrupts another creature’s turn, that creature can continue
    its turn right after the Reaction. In terms of timing, a Reaction takes place immediately after its
    trigger unless the Reaction’s description says otherwise.'
  sha256: 18efa2a25ac5e0447e91cb7a2bd9ae213f04b376c79cfb1ff4bb80a40201f704
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reactions

<!-- SRD page 10 -->

Certain special abilities, spells, and situations al-\
low you to take a special action called a Reaction. A\
Reaction is an instant response to a trigger of some\
kind, which can occur on your turn or on someone\
else’s. The Opportunity Attack, described later in\
“Playing the Game,” is the most common type of\
Reaction.\
When you take a Reaction, you can’t take another\
one until the start of your next turn. If the reaction\
interrupts another creature’s turn, that creature\
can continue its turn right after the Reaction.\
In terms of timing, a Reaction takes place immedi-\
ately after its trigger unless the Reaction’s descrip-\
tion says otherwise.\
