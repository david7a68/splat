---
splat_schema: 1
splat_id: 94b1c667-ab0d-515c-9f10-d7e2c9df23b2
splat_kind: spell
splat_key: dnd2024:spells/foresight
splat_name: Foresight
splat_origin:
  key: dnd2024:reference/spell/foresight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 134
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: divination
  classes:
  - bard
  - druid
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: 1 minute
  ritual: false
  range:
    mode: touch
  material:
    description: a hummingbird feather
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/foresight
  implementation: dnd2024:spell_foresight
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 28800
    target:
      kind: creature
      willing: true
      touch: true
    modifiers:
    - selector: target_d20_tests
      advantage: true
    - selector: other_creatures_attack_rolls_against_target
      disadvantage: true
    end_early:
      event: spell.cast_completed
      subject: caster
      spell:
        key: dnd2024:spells/foresight
splat_contracts:
- id: dnd2024:spell_foresight
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: f37094d03db5ef7eb1209b623bf82b952c51daadbf7cb7d6291bb3917e9055ec
  source_path: builtin/srd-5.2.1/spell/foresight.md
  pages:
  - 134
  engine_gaps: &id001
  - dnd2024:spell_foresight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 561b5625d673aa2045f56fd02dea43d52871a2da2302ddeaaa340a985d4f8707
splat_clauses:
- key: source_rules
  text: '# Foresight Level 9 Divination (Bard, Druid, Warlock, Wizard) Casting Time: 1 minute Range: Touch
    Components: V, S, M (a hummingbird feather) Duration: 8 hours You touch a willing creature and bestow
    a limited ability to see into the immediate future. For the duration, the target has Advantage on
    D20 Tests, and other creatures have Disadvantage on attack rolls against it. The spell ends early
    if you cast it again.'
  sha256: 77ffc24215f6958b908552958bc2359f9102d00ea86a2ef08b44f828c4c68de6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Foresight

<!-- SRD page 134 -->

Level 9 Divination (Bard, Druid, Warlock, Wizard)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (a hummingbird feather)\
Duration: 8 hours\
You touch a willing creature and bestow a limited\
ability to see into the immediate future. For the du-\
ration, the target has Advantage on D20 Tests, and\
other creatures have Disadvantage on attack rolls\
against it. The spell ends early if you cast it again.\

Original layout: [[../../srd-5.2.1/pages/134|page 134]].
