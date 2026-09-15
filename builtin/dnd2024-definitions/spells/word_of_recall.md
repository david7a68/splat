---
splat_schema: 1
splat_id: f01125ab-d5e2-5211-9b98-faa790036b8c
splat_kind: spell
splat_key: dnd2024:spells/word_of_recall
splat_name: Word of Recall
splat_origin:
  key: dnd2024:reference/spell/word_of_recall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 175
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - cleric
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 5
splat_rule:
  mechanic: dnd2024:spells/word_of_recall
  implementation: dnd2024:spell_word_of_recall
  parameters:
    sanctuary_preparation:
      method: cast_this_spell_at_chosen_location
      persists_as: designated_sanctuary
    targeting:
      caster: included
      other_creatures:
        max_count: 5
        willing: true
        within_caster_feet: 5
    teleport:
      timing: instantaneous
      requires: previously_prepared_sanctuary
      destination: nearest_unoccupied_spaces_to_designated_sanctuary_spot
      no_prepared_sanctuary: no_teleport_effect
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_word_of_recall
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
    locations:
      type: location_list
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - campaign_state
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6b580d1ab82fb1a1cc1196e36f63b730c7e04403101914a46dfd39f28183e317
  source_path: builtin/srd-5.2.1/spell/word_of_recall.md
  pages:
  - 175
  engine_gaps: &id001
  - dnd2024:spell_word_of_recall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: ff0c45cd8d4faec5a7e2edce406ab20485c79863b8a90e204f271da827be9b79
splat_clauses:
- key: source_rules
  text: '# Word of Recall Level 6 Conjuration (Cleric) Casting Time: Action Range: 5 feet Components:
    V Duration: Instantaneous You and up to five willing creatures within 5 feet of you instantly teleport
    to a previously designated sanctuary. You and any creatures that teleport with you appear in the nearest
    unoccupied space to the spot you designated when you prepared your sanctuary (see below). If you cast
    this spell without first preparing a sanctuary, the spell has no effect. You must designate a location,
    such as a temple, as a sanctuary by casting this spell there.'
  sha256: cefdf8177a197a5b67daea34929221ed795f0f33c56b49d5ac39b5823b990c46
  status: contract_specified
splat_requires_mechanics: *id001
---

# Word of Recall

<!-- SRD page 175 -->

Level 6 Conjuration (Cleric)\
Casting Time: Action\
Range: 5 feet\
Components: V\
Duration: Instantaneous\
You and up to five willing creatures within 5 feet\
of you instantly teleport to a previously designated\
sanctuary. You and any creatures that teleport with\
you appear in the nearest unoccupied space to the\
spot you designated when you prepared your sanc-\
tuary (see below). If you cast this spell without first\
preparing a sanctuary, the spell has no effect.\
You must designate a location, such as a temple, as\
a sanctuary by casting this spell there.\

Original layout: [[../../srd-5.2.1/pages/175|page 175]].
