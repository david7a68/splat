---
splat_schema: 1
splat_id: 1f2b7ac1-fa40-53ca-9bae-203376d8160c
splat_kind: spell
splat_key: dnd2024:spells/telepathic_bond
splat_name: Telepathic Bond
splat_origin:
  key: dnd2024:reference/spell/telepathic_bond
splat_sources:
- title: SRD 5.2.1
  locator: pp. 168
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: divination
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 30
  material:
    description: two eggs
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/telepathic_bond
  implementation: dnd2024:spell_telepathic_bond
  parameters:
    targeting:
      kind: creatures
      max_count: 8
      willing: true
      chosen: true
      within_range: true
      must_communicate_in_at_least_one_language: true
    bond:
      topology: each_to_all
      channel: telepathic
      shared_language_required: false
      distance_limit: none
      planar_crossing: false
      duration: spell_duration
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_telepathic_bond
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
    choices:
      type: choice
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: f7258de1ef50fb135b652028007a35b349d109fd1f89713c137f561ca537fdcb
  source_path: builtin/srd-5.2.1/spell/telepathic_bond.md
  pages:
  - 168
  engine_gaps: &id001
  - dnd2024:spell_telepathic_bond
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 32540cc3faaaa6eb0abda71f532bbb39de08f556588e09478fd9445213509920
splat_clauses:
- key: source_rules
  text: '# Telepathic Bond Level 5 Divination (Bard, Wizard) Casting Time: Action or Ritual Range: 30
    feet Components: V, S, M (two eggs) Duration: 1 hour You forge a telepathic link among up to eight
    willing creatures of your choice within range, psychically linking each creature to all the others
    for the duration. Creatures that can’t communicate in any languages aren’t affected by this spell.
    Until the spell ends, the targets can communicate telepathically through the bond whether or not they
    share a language. The communication is possible over any distance, though it can’t extend to other
    planes of existence.'
  sha256: f8285576b4e56de10fb685c9992de2389539c98be4b04aaba14bb3e6543f5b94
  status: contract_specified
splat_requires_mechanics: *id001
---

# Telepathic Bond

<!-- SRD page 168 -->

Level 5 Divination (Bard, Wizard)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (two eggs)\
Duration: 1 hour\
You forge a telepathic link among up to eight willing\
creatures of your choice within range, psychically\
linking each creature to all the others for the dura-\
tion. Creatures that can’t communicate in any lan-\
guages aren’t affected by this spell.\
Until the spell ends, the targets can communicate\
telepathically through the bond whether or not they\
share a language. The communication is possible\
over any distance, though it can’t extend to other\
planes of existence.\

Original layout: [[../../srd-5.2.1/pages/168|page 168]].
