---
splat_schema: 1
splat_id: e085ecb5-245e-57d4-a4bb-67796e1be499
splat_kind: rule
splat_key: dnd2024:glossary/search_action
splat_name: Search [Action]
splat_origin:
  key: dnd2024:reference/glossary/search_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/search_action
  parameters:
    cost: action
    ability: wisdom
    purpose: discern_nonobvious_information
    suggested_skill_by_target:
      creature_state_of_mind: insight
      creature_ailment: medicine
      cause_of_death: medicine
      concealed_creature: perception
      concealed_object: perception
      tracks: survival
      food: survival
  implementation: dnd2024:glossary_search_action
splat_contracts:
- id: dnd2024:glossary_search_action
  inputs:
    subject:
      type: participant
      required: true
    search_topic:
      type: choice
      required: true
    dc:
      type: integer
      required: true
  outputs:
  - check
  - discovered_information
  events:
  - action.requested
  capabilities:
  - action_economy
  - checks
  - perception
splat_conversion:
  status: specified
  category: glossary
  source_sha256: c693cb0a57f60e83cf46e8d341e0bc143513002e96f577ce6dc830ae2646b5c8
  source_path: builtin/srd-5.2.1/glossary/search_action.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_search_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d88657a5674bba1655943c617c2a0b51432a33bf9b780f8fe391794c2e0bb464
splat_clauses:
- key: source_rules
  text: '# Search [Action] When you take the Search action, you make a Wisdom check to discern something
    that isn’t obvious. The Search table suggests which skills are applicable when you take this action,
    depending on what you’re trying to detect. Search Skill Thing to Detect Insight Creature’s state of
    mind Medicine Creature’s ailment or cause of death Perception Concealed creature or object Survival
    Tracks or food'
  sha256: 5e3900c77c74605c0dd276d835187e3e3af5ff3c1399d979feae9786067a2eaa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Search [Action]

<!-- SRD page 187 -->

When you take the Search action, you make a Wis-\
dom check to discern something that isn’t obvious.\
The Search table suggests which skills are applica-\
ble when you take this action, depending on what\
you’re trying to detect.\
Search\
Skill Thing to Detect\
Insight Creature’s state of mind\
Medicine Creature’s ailment or cause of death\
Perception Concealed creature or object\
Survival Tracks or food\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
