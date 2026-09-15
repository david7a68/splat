---
splat_schema: 1
splat_id: 5aae911d-50b9-50d4-a03b-6957d559bc2b
splat_kind: rule
splat_key: dnd2024:glossary/ready_action
splat_name: Ready [Action]
splat_origin:
  key: dnd2024:reference/glossary/ready_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186, 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/ready_action
  parameters:
    cost: action
    timing: own_turn
    expires: own_next_turn_start
    trigger:
      chooser: subject
      must_be_perceivable: true
    response:
      chooser: subject
      options:
      - chosen_action
      - move_up_to_speed
      cost: reaction
      timing: immediately_after_trigger_finishes
      may_ignore_trigger: true
    spell:
      casting_time_required: action
      cast_when_readied: true
      casting_resources_expended_when_readied: true
      energy_held_until_release: true
      release_cost: reaction
      concentration_required: true
      maximum_hold_until: own_next_turn_start
      concentration_loss:
        dissipates: true
        effect_occurs: false
  implementation: dnd2024:glossary_ready_action
splat_contracts:
- id: dnd2024:glossary_ready_action
  inputs:
    subject:
      type: participant
      required: true
    trigger:
      type: choice
      required: true
    response:
      type: choice
      required: true
    spell:
      required: false
      type: definition
  outputs:
  - readied_effect
  - resource_cost
  - reaction_response
  events:
  - action.requested
  - trigger.completed
  - reaction.requested
  - concentration.ended
  - turn.started
  capabilities:
  - action_economy
  - events
  - spellcasting
  - concentration
splat_conversion:
  status: specified
  category: glossary
  source_sha256: b5473403e3db53477249d722d9bc090c664f5925520bd654d3cd33e6eeecd6d0
  source_path: builtin/srd-5.2.1/glossary/ready_action.md
  pages:
  - 186
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_ready_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: aeb9a53ab7e7a2c8400a8ea32dc05fdb36c6ee010f1de6cfa69b96c1fa8cfab3
splat_clauses:
- key: source_rules
  text: '# Ready [Action] You take the Ready action to wait for a particular circumstance before you act.
    To do so, you take this action on your turn, which lets you act by taking a Reaction before the start
    of your next turn. First, you decide what perceivable circumstance will trigger your Reaction. Then,
    you choose the action you will take in response to that trigger, or you choose to move up to your
    Speed in response to it. Examples include “If the cultist steps on the trapdoor, I’ll pull the lever
    that opens it,” and “If the zombie steps next to me, I move away.” When the trigger occurs, you can
    either take your Reaction right after the trigger finishes or ignore the trigger. When you Ready a
    spell, you cast it as normal (expending any resources used to cast it) but hold its energy, which
    you release with your Reaction when the trigger occurs. To be readied, a spell must have a casting
    time of an action, and holding on to the spell’s magic requires Concentration, which you can maintain
    up to the start of your next turn. If your Concentration is broken, the spell dissipates without taking
    effect.'
  sha256: bf77246f65282df2b1fdbd04ff0f32739133d78b43e19032b7659060c9edb9e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ready [Action]

<!-- SRD page 186 -->

You take the Ready action to wait for a particular\
circumstance before you act. To do so, you take this\
action on your turn, which lets you act by taking a\
Reaction before the start of your next turn.\
First, you decide what perceivable circumstance\
will trigger your Reaction. Then, you choose the\
action you will take in response to that trigger, or\
you choose to move up to your Speed in response\
to it. Examples include “If the cultist steps on the\

<!-- SRD page 187 -->

trapdoor, I’ll pull the lever that opens it,” and “If the\
zombie steps next to me, I move away.”\
When the trigger occurs, you can either take your\
Reaction right after the trigger finishes or ignore\
the trigger.\
When you Ready a spell, you cast it as normal (ex-\
pending any resources used to cast it) but hold its\
energy, which you release with your Reaction when\
the trigger occurs. To be readied, a spell must have\
a casting time of an action, and holding on to the\
spell’s magic requires Concentration, which you can\
maintain up to the start of your next turn. If your\
Concentration is broken, the spell dissipates with-\
out taking effect.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]], [[../../srd-5.2.1/pages/187|page 187]].
