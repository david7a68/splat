---
splat_schema: 1
splat_id: a6b4c0c6-56b5-5ae7-8916-92358e545e95
splat_kind: rule
splat_key: dnd2024:glossary/saving_throw
splat_name: Saving Throw
splat_origin:
  key: dnd2024:reference/glossary/saving_throw
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/saving_throw
  parameters:
    normally_requires_rule: true
    voluntary_failure:
      chooser: subject
      rolling_required: false
    missing_relevant_ability_score: automatic_failure
    result_effects: requiring_effect_defined
  implementation: dnd2024:glossary_saving_throw
splat_contracts:
- id: dnd2024:glossary_saving_throw
  inputs:
    subject:
      type: object
      required: true
    effect:
      type: effect
      required: true
    ability:
      type: choice
      required: true
    voluntary_failure:
      type: boolean
      required: true
  outputs:
  - save_requirement
  - save_result
  events:
  - saving_throw.pending
  capabilities:
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: c43feec4553021894a1fcd229f5515e7c533d3a7cbe3dffd7738decec555c974
  source_path: builtin/srd-5.2.1/glossary/saving_throw.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_saving_throw
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 6f9f62da69857e2d97af7ba552a0242b5f380169b9aa82a609f3bbb9b54b40b5
splat_clauses:
- key: source_rules
  text: '# Saving Throw A saving throw—also called a save—represents an attempt to avoid or resist a threat.
    You normally make a saving throw only when a rule requires you to do so, but you can decide to fail
    the save without rolling. The result of a save is detailed in the effect that allowed it. If a target
    is forced to make a save and lacks the ability score used by it, the target automatically fails. See
    also “Playing the Game” (“D20 Tests”).'
  sha256: ea4fdfcd3617a8190c833faee9f4a15eaedbef52d7b84e864d51f6da06e6b4c9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Saving Throw

<!-- SRD page 187 -->

A saving throw—also called a save—represents an\
attempt to avoid or resist a threat. You normally\
make a saving throw only when a rule requires you\
to do so, but you can decide to fail the save without\
rolling. The result of a save is detailed in the effect\
that allowed it. If a target is forced to make a save\
and lacks the ability score used by it, the target au-\
tomatically fails. See also “Playing the Game” (“D20\
Tests”).\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
