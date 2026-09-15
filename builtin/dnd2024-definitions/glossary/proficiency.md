---
splat_schema: 1
splat_id: 4018583f-4bb2-56f8-89f2-41176e0b6184
splat_kind: rule
splat_key: dnd2024:glossary/proficiency
splat_name: Proficiency
splat_origin:
  key: dnd2024:reference/glossary/proficiency
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/proficiency
  parameters:
    categories:
    - skill
    - saving_throw
    - weapon
    - tool
    d20_test_bonus:
      requires: test_uses_proficiency
      add:
        var: proficiency_bonus
  implementation: dnd2024:glossary_proficiency
splat_contracts:
- id: dnd2024:glossary_proficiency
  inputs:
    subject:
      type: participant
      required: true
    proficiency:
      type: definition
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - check_bonus
  events:
  - d20_test.pending
  capabilities:
  - proficiency
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a1a728b0bf3d73440efecfbc2161a3dca16629fd206863bdf080c24ff832f7a2
  source_path: builtin/srd-5.2.1/glossary/proficiency.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_proficiency
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 69d635094434a40fdbc0304bb7f89b5038670fa47fb56dd0cefb4cd6ae761e5b
splat_clauses:
- key: source_rules
  text: '# Proficiency If you have proficiency with something, you can add your Proficiency Bonus to any
    D20 Test you make using that thing. A creature might have proficiency in a skill or saving throw or
    with a weapon or tool. See also “Playing the Game” (“Proficiency”).'
  sha256: f6a0a932cff7f10aa4483cf883fa6918d655c99c966bf5d6cd44d2ec4fa8125d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Proficiency

<!-- SRD page 186 -->

If you have proficiency with something, you can add\
your Proficiency Bonus to any D20 Test you make\
using that thing. A creature might have proficiency\
in a skill or saving throw or with a weapon or tool.\
See also “Playing the Game” (“Proficiency”).\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
