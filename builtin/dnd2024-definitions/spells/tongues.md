---
splat_schema: 1
splat_id: 1c5762b0-876d-57b7-bb92-c91e8614a4e7
splat_kind: spell
splat_key: dnd2024:spells/tongues
splat_name: Tongues
splat_origin:
  key: dnd2024:reference/spell/tongues
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169, 170
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: divination
  classes:
  - bard
  - cleric
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a miniature ziggurat
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/tongues
  implementation: dnd2024:spell_tongues
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      touch: true
    understand:
      languages: any
      spoken_if: heard
      signed_if: seen
    be_understood:
      listener_knows_minimum_languages: 1
      speech_if: heard
      signing_if: seen
splat_contracts:
- id: dnd2024:spell_tongues
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - language_comprehension_effect
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: 4c2cbca5e3b4543704d8a89b038d917e51e2f365e69d215a65a910c31441f8f4
  source_path: builtin/srd-5.2.1/spell/tongues.md
  pages:
  - 169
  - 170
  engine_gaps: &id001
  - dnd2024:spell_tongues
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 79d9d63cbe38ef513b6f067fe5340e8ac9a81a46af999049177cb0ac2c204311
splat_clauses:
- key: source_rules
  text: '# Tongues Level 3 Divination (Bard, Cleric, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Touch Components: V, M (a miniature ziggurat) Duration: 1 hour This spell grants the creature you
    touch the ability to understand any spoken or signed language that it hears or sees. Moreover, when
    the target communicates by speaking or signing, any creature that knows at least one language can
    understand it if that creature can hear the speech or see the signing.'
  sha256: e64703482d6e1d967b77a8c6e8e2711d9473de740ae8f8086fc17e78d2056651
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tongues

<!-- SRD page 169 -->

Level 3 Divination (Bard, Cleric, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, M (a miniature ziggurat)\
Duration: 1 hour\
This spell grants the creature you touch the ability\
to understand any spoken or signed language that\
it hears or sees. Moreover, when the target commu-\
nicates by speaking or signing, any creature that\

<!-- SRD page 170 -->

knows at least one language can understand it if\
that creature can hear the speech or see the signing.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]], [[../../srd-5.2.1/pages/170|page 170]].
