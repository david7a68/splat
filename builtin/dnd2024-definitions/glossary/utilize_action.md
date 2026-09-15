---
splat_schema: 1
splat_id: 63b98aa3-9c18-575b-a043-aa333584fc2b
splat_kind: rule
splat_key: dnd2024:glossary/utilize_action
splat_name: Utilize [Action]
splat_origin:
  key: dnd2024:reference/glossary/utilize_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 191
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/utilize_action
  parameters:
    normal_object_interaction: alongside_other_activity
    object_use_requiring_action: utilize_action
    cost: action
  implementation: dnd2024:glossary_utilize_action
splat_contracts:
- id: dnd2024:glossary_utilize_action
  inputs:
    subject:
      type: participant
      required: true
    object:
      type: object
      required: true
    use:
      type: choice
      required: true
  outputs:
  - object_use
  events:
  - action.requested
  capabilities:
  - action_economy
  - objects
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 57d3c7bb8c9b75524ef297b66dfb63afdbfb12159bc31306cb126246e4e15cb8
  source_path: builtin/srd-5.2.1/glossary/utilize_action.md
  pages:
  - 191
  engine_gaps: &id001
  - dnd2024:glossary_utilize_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 60974b030539f7f7df30aa50b230f691988fc973a99bf5292620a623176cc240
splat_clauses:
- key: source_rules
  text: '# Utilize [Action] You normally interact with an object while doing something else, such as when
    you draw a sword as part of the Attack action. When an object requires an action for its use, you
    take the Utilize action.'
  sha256: 4e598490e1df2bc1829e97d6313797bd8d0af186ef7857242818f8358e20eec6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Utilize [Action]

<!-- SRD page 191 -->

You normally interact with an object while doing\
something else, such as when you draw a sword as\
part of the Attack action. When an object requires\
an action for its use, you take the Utilize action.\

Original layout: [[../../srd-5.2.1/pages/191|page 191]].
