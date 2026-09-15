---
splat_schema: 1
splat_id: 039ce569-5324-5bfe-8b21-b1ae5f50ca85
splat_kind: spell
splat_key: dnd2024:spells/false_life
splat_name: False Life
splat_origin:
  key: dnd2024:reference/spell/false_life
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: necromancy
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a drop of alcohol
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/false_life
  implementation: dnd2024:spell_false_life
  parameters:
    targets:
      kind: self
    temporary_hp:
      dice:
        count: 2
        sides: 4
        add:
          op: add
          args:
          - 4
          - op: multiply
            args:
            - 5
            - op: subtract
              args:
              - binding: cast_level
              - 1
splat_contracts:
- id: dnd2024:spell_false_life
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - health
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - dice
  - health
splat_conversion:
  status: specified
  category: spell
  source_sha256: e0270e5c417a51bf8ba67fac6a80d6b07255169acb99434dfa4ac9c3ae853a89
  source_path: builtin/srd-5.2.1/spell/false_life.md
  pages:
  - 129
  engine_gaps: &id001
  - dnd2024:spell_false_life
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 89bf496c9f6f12c9c5fe4860fa7264c3b61d7a5c6e3b23d3cb7b4360b9702625
splat_clauses:
- key: source_rules
  text: '# False Life Level 1 Necromancy (Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a drop of alcohol) Duration: Instantaneous You gain 2d4 + 4 Temporary Hit Points. Using a
    Higher-Level Spell Slot. You gain 5 additional Temporary Hit Points for each spell slot level above
    1.'
  sha256: f72a06d6e72affa5d281e48b1336e1f68112eba143e1effabdd66f511518ea9a
  status: contract_specified
splat_requires_mechanics: *id001
---

# False Life

<!-- SRD page 129 -->

Level 1 Necromancy (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a drop of alcohol)\
Duration: Instantaneous\
You gain 2d4 + 4 Temporary Hit Points.\
Using a Higher-Level Spell Slot. You gain 5 addi-\
tional Temporary Hit Points for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].
