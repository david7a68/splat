---
splat_schema: 1
splat_id: 85bb3099-8f3a-5296-b590-eb7ed9e2d595
splat_kind: rule
splat_key: dnd2024:glossary/expertise
splat_name: Expertise
splat_origin:
  key: dnd2024:reference/glossary/expertise
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/expertise
  parameters:
    selection:
      skill_count: 1
      requires_skill_proficiency: true
      existing_expertise_disallowed: true
    proficiency_bonus_multiplier: 2
    scope: ability_check_with_selected_skill
    stacks_with_other_doubling: false
    same_skill_multiple_expertise: false
  implementation: dnd2024:glossary_expertise
splat_contracts:
- id: dnd2024:glossary_expertise
  inputs:
    subject:
      type: participant
      required: true
    skill:
      type: definition
      required: true
  outputs:
  - expertise_selection
  - check_modifier
  events:
  - expertise.granted
  - ability_check.pending
  capabilities:
  - proficiency
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 124e0ac423b63724573fa3c2833f5822357d89f2135a961a6d86608b278453f2
  source_path: builtin/srd-5.2.1/glossary/expertise.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_expertise
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: b9eb1d526af5f75bc72bed38e34c646147fabf9cfe783bc0c459793f39b50345
splat_clauses:
- key: source_rules
  text: '# Expertise Expertise is a feature that enhances your use of a skill proficiency. When you make
    an ability check with a skill proficiency in which you have Expertise, your Proficiency Bonus is doubled
    for that check unless the bonus is doubled by another feature. If you gain Expertise, you gain it
    in one skill in which you have proficiency. You can’t have Expertise in the same skill proficiency
    more than once. See also “Playing the Game” (“Proficiency”).'
  sha256: 100f9e89589ce338a814abc778d5a8eef0949584c36cf6f4ec99fbd77b9ea7b1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Expertise

<!-- SRD page 182 -->

Expertise is a feature that enhances your use of a\
skill proficiency. When you make an ability check\
with a skill proficiency in which you have Expertise,\
your Proficiency Bonus is doubled for that check un-\
less the bonus is doubled by another feature.\
If you gain Expertise, you gain it in one skill in\
which you have proficiency. You can’t have Exper-\
tise in the same skill proficiency more than once.\
See also “Playing the Game” (“Proficiency”).\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
