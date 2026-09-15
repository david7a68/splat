---
splat_schema: 1
splat_id: c2287311-87b0-59ab-a189-62d6d78e671a
splat_kind: spell
splat_key: dnd2024:spells/enthrall
splat_name: Enthrall
splat_origin:
  key: dnd2024:reference/spell/enthrall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 128
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: enchantment
  classes:
  - bard
  - warlock
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/enthrall
  implementation: dnd2024:spell_enthrall
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    targets:
      kind: creature
      visible: true
      range_feet: 60
      chooser: caster
      count: any
    save:
      ability: wisdom
      dc: spell_save_dc
      automatic_success_if: caster_or_companions_fighting_target
    failure:
      modifiers:
      - selector: wisdom_perception_checks
        add: -10
      - selector: passive_perception
        add: -10
splat_contracts:
- id: dnd2024:spell_enthrall
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5972ed01d62e5eb95cd48cb3b8b8d6eb8bf1cab050d738b383284d23673a60e3
  source_path: builtin/srd-5.2.1/spell/enthrall.md
  pages:
  - 128
  engine_gaps: &id001
  - dnd2024:spell_enthrall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 8981960906b9d6431324f8fe506e478ad101804cdd5e579ede64bc45817f87b1
splat_clauses:
- key: source_rules
  text: '# Enthrall Level 2 Enchantment (Bard, Warlock) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Concentration, up to 1 minute You weave a distracting string of words, causing creatures
    of your choice that you can see within range to make a Wisdom saving throw. Any creature you or your
    companions are fighting automatically succeeds on this save. On a failed save, a target has a -10
    penalty to Wisdom (Perception) checks and Passive Perception until the spell ends.'
  sha256: 95f7f23b25e29f7dd7114734d366b48d9980a916ac21138e0ac1bcd7e416117b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Enthrall

<!-- SRD page 128 -->

Level 2 Enchantment (Bard, Warlock)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
You weave a distracting string of words, causing\
creatures of your choice that you can see within\
range to make a Wisdom saving throw. Any crea-\
ture you or your companions are fighting automati-\
cally succeeds on this save. On a failed save, a target\
has a −10 penalty to Wisdom (Perception) checks\
and Passive Perception until the spell ends.\

Original layout: [[../../srd-5.2.1/pages/128|page 128]].
