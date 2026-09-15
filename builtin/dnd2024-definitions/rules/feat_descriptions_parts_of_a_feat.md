---
splat_schema: 1
splat_id: 8872fe18-55a2-56be-9027-aa4ba24669e8
splat_kind: rule
splat_key: dnd2024:rules/feat_descriptions_parts_of_a_feat
splat_name: Parts of a Feat
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/feat_descriptions_parts_of_a_feat
  parameters:
    category_constraint:
      if_specified: must_match
      otherwise: any_category
    prerequisites:
      must_satisfy_all: true
      feature_can_waive: true
      class_requirement_min_level: 1
    owned_feat_grants_benefits: true
    maximum_instances_default: 1
    repeatable_override: feat_repeatable_section
  implementation: dnd2024:rules_feat_descriptions_parts_of_a_feat
splat_contracts:
- id: dnd2024:rules_feat_descriptions_parts_of_a_feat
  inputs:
    subject:
      type: participant
      required: true
    feat:
      type: definition
      required: true
    grant:
      type: resolution
      required: true
  outputs:
  - feat_eligibility
  - feat_grants
  events: []
  capabilities:
  - character_options
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:rules_feat_descriptions_parts_of_a_feat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 7d65c1a3528f023ccd2f9355a3a922c7e58dd772ff12c5f08a8315e0991b26dd
  parent_name: Feat Descriptions
  line_span:
    first: 7455
    count: 21
splat_clauses:
- key: source_rules
  text: '# Parts of a Feat The description of a feat contains the following parts, which are presented
    after the feat’s name. Category. A feat is a member of a category, which is noted in the feat. If
    you’re instructed to choose a feat from a specific category, such as the Origin category, that category
    must appear under the feat’s name. If you’re instructed to choose a feat and no category is specified,
    you can choose from any category. Prerequisite. To take a feat, you must meet any prerequisite in
    its description unless a feature allows you to take the feat without the prerequisite. If a prerequisite
    includes a class, you must have at least 1 level in that class to take the feat. Benefit. The benefits
    of a feat are detailed after any prerequisites are listed. If you have a feat, you gain its benefits.
    Repeatable. A feat can be taken only once unless its description states otherwise in a “Repeatable”
    subsection.'
  sha256: 2cb8fdd58eeb69a53d4254fb03d3249d286caed84f0d7927596d7e879625c7e7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Parts of a Feat

<!-- SRD page 87 -->

The description of a feat contains the following\
parts, which are presented after the feat’s name.\
Category. A feat is a member of a category, which\
is noted in the feat. If you’re instructed to choose\
a feat from a specific category, such as the Origin\
category, that category must appear under the\
feat’s name. If you’re instructed to choose a feat and\
no category is specified, you can choose from any\
category.\
Prerequisite. To take a feat, you must meet any\
prerequisite in its description unless a feature al-\
lows you to take the feat without the prerequisite.\
If a prerequisite includes a class, you must have at\
least 1 level in that class to take the feat.\
Benefit. The benefits of a feat are detailed after\
any prerequisites are listed. If you have a feat, you\
gain its benefits.\
Repeatable. A feat can be taken only once unless\
its description states otherwise in a “Repeatable”\
subsection.\
