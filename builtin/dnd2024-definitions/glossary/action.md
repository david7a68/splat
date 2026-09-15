---
splat_schema: 1
splat_id: 05503f6b-dd1c-5f81-9dcd-1c48d8a93280
splat_kind: rule
splat_key: dnd2024:glossary/action
splat_name: Action
splat_origin:
  key: dnd2024:reference/glossary/action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/action
  parameters:
    budget:
      per_turn: 1
      timing: own_turn
    options:
    - key: dnd2024:glossary/attack_action
    - key: dnd2024:glossary/dash_action
    - key: dnd2024:glossary/disengage_action
    - key: dnd2024:glossary/dodge_action
    - key: dnd2024:glossary/help_action
    - key: dnd2024:glossary/hide_action
    - key: dnd2024:glossary/influence_action
    - key: dnd2024:glossary/magic_action
    - key: dnd2024:glossary/ready_action
    - key: dnd2024:glossary/search_action
    - key: dnd2024:glossary/study_action
    - key: dnd2024:glossary/utilize_action
    additional_options: granted_feature_actions
  implementation: dnd2024:glossary_action
splat_contracts:
- id: dnd2024:glossary_action
  inputs:
    subject:
      type: participant
      required: true
    selected_action:
      type: definition
      required: true
  outputs:
  - action_budget
  - legal_actions
  events:
  - turn.started
  - action.requested
  capabilities:
  - action_economy
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a568fc30c841ec28f75d244e0e1ded4479a7c7e4f0391afb92b271577fdff840
  source_path: builtin/srd-5.2.1/glossary/action.md
  pages:
  - 176
  engine_gaps: &id001
  - dnd2024:glossary_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 564c0c2638a79dcb5112631491ffe197e3e2217537670c539deeb18a0860f3fe
splat_clauses:
- key: source_rules
  text: '# Action On your turn, you can take one action. Choose which action to take from those below
    or from the special actions provided by your features. See also “Playing the Game” (“Actions”). These
    actions are defined elsewhere in this glossary: Attack Dodge Influence Search Dash Help Magic Study
    Disengage Hide Ready Utilize'
  sha256: 5575000d60aa293af6cfebf0e751bc7acda41d65d33c687185ed9d6466f50cd6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Action

<!-- SRD page 176 -->

On your turn, you can take one action. Choose which\
action to take from those below or from the special\
actions provided by your features. See also “Playing\
the Game” (“Actions”). These actions are defined\
elsewhere in this glossary:\
Attack Dodge Influence Search\
Dash Help Magic Study\
Disengage Hide Ready Utilize\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].
