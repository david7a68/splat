---
splat_schema: 1
splat_id: f9ed276d-2674-5d36-ae3b-0e5c34192ee7
splat_kind: spell
splat_key: dnd2024:spells/lesser_restoration
splat_name: Lesser Restoration
splat_origin:
  key: dnd2024:reference/spell/lesser_restoration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Bonus Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/lesser_restoration
  implementation: dnd2024:spell_lesser_restoration
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      touched: true
    remove_condition:
      count: 1
      choices:
      - blinded
      - deafened
      - paralyzed
      - poisoned
splat_contracts:
- id: dnd2024:spell_lesser_restoration
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
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: ba2d0f806edaf4945d7928433bdd5b2cbdab16d72fcc5884417afe4423155fb1
  source_path: builtin/srd-5.2.1/spell/lesser_restoration.md
  pages:
  - 144
  engine_gaps: &id001
  - dnd2024:spell_lesser_restoration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 297cf687ee2c4c967d4f59adb779d913884130e6489907690d4bc1cc51356b34
splat_clauses:
- key: source_rules
  text: '# Lesser Restoration Level 2 Abjuration (Bard, Cleric, Druid, Paladin, Ranger) Casting Time:
    Bonus Action Range: Touch Components: V, S Duration: Instantaneous You touch a creature and end one
    condition on it: Blinded, Deafened, Paralyzed, or Poisoned.'
  sha256: 336e1e335bb29675ae42d7b5e73c00d9b377b03a7e9d449c4f68418735883811
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lesser Restoration

<!-- SRD page 144 -->

Level 2 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger)\
Casting Time: Bonus Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
You touch a creature and end one condition on it:\
Blinded, Deafened, Paralyzed, or Poisoned.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].
