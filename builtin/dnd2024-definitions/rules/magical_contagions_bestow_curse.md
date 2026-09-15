---
splat_schema: 1
splat_id: 50772e5a-4ec9-514c-aafe-0302af9a3ed4
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_bestow_curse
splat_name: Bestow Curse
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magical_contagions_bestow_curse
  parameters:
    curse:
      key: dnd2024:spells/bestow_curse
    ended_by:
      key: dnd2024:spells/remove_curse
    potency_benchmarks:
    - duration:
        value: 1
        unit: minute
      spell_level: 3
    - duration: until_dispelled
      spell_level: 9
  implementation: dnd2024:rules_magical_contagions_bestow_curse
splat_contracts:
- id: dnd2024:rules_magical_contagions_bestow_curse
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - curse_effects
  - potency_benchmarks
  events: []
  capabilities:
  - curses
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 193
  engine_gaps: &id001
  - dnd2024:rules_magical_contagions_bestow_curse
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: c226187a003aa434481e4df5a37dd5518b1f10d20cffde28a955144632a334aa
  parent_name: Magical Contagions
  line_span:
    first: 9443
    count: 9
splat_clauses:
- key: source_rules
  text: '# Bestow Curse The simplest curses are created by the Bestow Curse spell. The effects of such
    curses are limited and can be ended by the Remove Curse spell. Bestow Curse provides useful benchmarks
    for gauging the potency of other curses. A curse that lasts for 1 minute equates to a level 3 spell,
    while one that lasts until dispelled equates to a level 9 spell.'
  sha256: 91004dc55c321b42fb6b10f4cfe31f0f660afb1ca3b7754662ca32bf5267e8bb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bestow Curse

<!-- SRD page 193 -->

The simplest curses are created by the Bestow Curse\
spell. The effects of such curses are limited and can\
be ended by the Remove Curse spell.\
Bestow Curse provides useful benchmarks for\
gauging the potency of other curses. A curse that\
lasts for 1 minute equates to a level 3 spell, while\
one that lasts until dispelled equates to a level 9\
spell.\
