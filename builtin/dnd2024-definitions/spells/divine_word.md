---
splat_schema: 1
splat_id: 59d55d3b-0a95-5744-9560-181a239903a3
splat_kind: spell
splat_key: dnd2024:spells/divine_word
splat_name: Divine Word
splat_origin:
  key: dnd2024:reference/spell/divine_word
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: evocation
  classes:
  - cleric
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/divine_word
  implementation: dnd2024:spell_divine_word
  parameters:
    targeting:
      kind: creatures
      chosen: true
      within_range: true
    save:
      ability: charisma
      success: no_effect
    failure_hp_branches:
    - hp_min: 0
      hp_max: 20
      effect: death
    - hp_min: 21
      hp_max: 30
      conditions:
      - blinded
      - deafened
      - stunned
      duration_minutes: 60
    - hp_min: 31
      hp_max: 40
      conditions:
      - blinded
      - deafened
      duration_minutes: 10
    - hp_min: 41
      hp_max: 50
      conditions:
      - deafened
      duration_minutes: 1
    additional_failure:
      types:
      - celestial
      - elemental
      - fey
      - fiend
      hp_independent: true
      destination: origin_plane
      only_if_elsewhere: true
      return_to_departed_plane_forbidden_hours: 24
      bypass: wish
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_divine_word
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
    choices:
      type: choice
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - conditions
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: 769041189cca7e3fb80e21ae97da9edc0d0bbc6df5ec6b44cb8049e0d06623fb
  source_path: builtin/srd-5.2.1/spell/divine_word.md
  pages:
  - 125
  engine_gaps: &id001
  - dnd2024:spell_divine_word
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: edbddf13cf4cc87835a6da408d6cd2eb7711f02811e1e69729d20fe1496cee9f
splat_clauses:
- key: source_rules
  text: '# Divine Word Level 7 Evocation (Cleric) Casting Time: Bonus Action Range: 30 feet Components:
    V Duration: Instantaneous You utter a word imbued with power from the Upper Planes. Each creature
    of your choice in range makes a Charisma saving throw. On a failed save, a target that has 50 Hit
    Points or fewer suffers an effect based on its current Hit Points, as shown in the Divine Word Effects
    table. Regardless of its Hit Points, a Celestial, an Elemental, a Fey, or a Fiend target that fails
    its save is forced back to its plane of origin (if it isn’t there already) and can’t return to the
    current plane for 24 hours by any means short of a Wish spell. Divine Word Effects Hit Points Effect
    0–20 The target dies. 21–30 The target has the Blinded, Deafened, and Stunned conditions for 1 hour.
    31–40 The target has the Blinded and Deafened conditions for 10 minutes. 41–50 The target has the
    Deafened condition for 1 minute.'
  sha256: 2b33a32f62298affbf554d2f1c551993eb0df5ed6e624b5a1dcaa60a722b70f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Divine Word

<!-- SRD page 125 -->

Level 7 Evocation (Cleric)\
Casting Time: Bonus Action\
Range: 30 feet\
Components: V\
Duration: Instantaneous\
You utter a word imbued with power from the Up-\
per Planes. Each creature of your choice in range\
makes a Charisma saving throw. On a failed save,\
a target that has 50 Hit Points or fewer suffers an\
effect based on its current Hit Points, as shown in\
the Divine Word Effects table. Regardless of its Hit\
Points, a Celestial, an Elemental, a Fey, or a Fiend\
target that fails its save is forced back to its plane of\
origin (if it isn’t there already) and can’t return to\
the current plane for 24 hours by any means short\
of a Wish spell.\
Divine Word Effects\
Hit Points Effect\
0–20 The target dies.\
21–30 The target has the Blinded, Deafened, and\
Stunned conditions for 1 hour.\
31–40 The target has the Blinded and Deafened\
conditions for 10 minutes.\
41–50 The target has the Deafened condition for\
1 minute.\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].
