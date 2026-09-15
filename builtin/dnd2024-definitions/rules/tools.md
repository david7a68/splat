---
splat_schema: 1
splat_id: cb18be95-6f20-5388-bc75-893a8f969930
splat_kind: rule
splat_key: dnd2024:rules/tools
splat_name: Tools
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/tools
  parameters:
    tool_check_ability: tool_definition
    utilize:
      options: tool_utilize_entries
      options_per_action: 1
      dc: selected_entry_dc
    craftable_items: tool_craft_entries
    each_variant_requires_separate_proficiency: true
  implementation: dnd2024:rule_tools
splat_contracts:
- id: dnd2024:rule_tools
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - tools
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 93
  engine_gaps: &id001
  - dnd2024:rule_tools
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: afe1fcb0ba6480f61bb22c456674e3cf6541cbfa06fed8fb7d8217734f07458d
  parent_name: Tools
  line_span:
    first: 7977
    count: 19
splat_clauses:
- key: source_rules
  text: '# Tools A tool helps you make specialized ability checks, craft certain items, or both. A tool’s
    description includes the tool’s cost and weight, as well as the following entries: Ability. This entry
    lists the ability to use when making an ability check with the tool. Utilize. This entry lists things
    you can do with the tool when you take the Utilize action. You can do one of those things each time
    you take the action. This entry also provides the DC for the action. Craft. This entry lists what,
    if anything, you can craft with the tool. For crafting rules, see “Crafting Nonmagical Items,” “Brewing
    Potions of Healing,” and “Scribing Spell Scrolls” later in “Equipment.” Variants. This entry appears
    if the tool has variants, which are listed. Each requires a separate proficiency.'
  sha256: 574b571caf3f502e69e663268bbadb37c35f0ecb3c32b4d1b7189c249e76d255
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tools

<!-- SRD page 93 -->

A tool helps you make specialized ability checks,\
craft certain items, or both. A tool’s description\
includes the tool’s cost and weight, as well as the\
following entries:\
Ability. This entry lists the ability to use when\
making an ability check with the tool.\
Utilize. This entry lists things you can do with the\
tool when you take the Utilize action. You can do\
one of those things each time you take the action.\
This entry also provides the DC for the action.\
Craft. This entry lists what, if anything, you\
can craft with the tool. For crafting rules, see\
“Crafting Nonmagical Items,” “Brewing Potions\
of Healing,” and “Scribing Spell Scrolls” later in\
“Equipment.”\
Variants. This entry appears if the tool has vari-\
ants, which are listed. Each requires a separate\
proficiency.\
