---
splat_schema: 1
splat_id: b3f3b4e2-e4a0-5b9b-a8f7-909da1308af3
splat_kind: spell
splat_key: dnd2024:spells/mind_blank
splat_name: Mind Blank
splat_origin:
  key: dnd2024:reference/spell/mind_blank
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: abjuration
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 24 hours
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/mind_blank
  implementation: dnd2024:spell_mind_blank
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      willing: true
      touch: true
    immunities:
      damage:
      - psychic
      conditions:
      - charmed
      effects:
      - sense_emotions
      - sense_alignment
      - read_thoughts
      - magically_detect_location
      spells:
      - gather_information_about_target
      - remote_observation
      - mind_control
      includes_wish: true
splat_contracts:
- id: dnd2024:spell_mind_blank
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
  - immunity_effect
  - information_protection
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - perception
  - forms
  - knowledge
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: be81dc3e21ad9eaa8d2036ca4e920e3e0ba20223fa8354181a2413e579d1e18b
  source_path: builtin/srd-5.2.1/spell/mind_blank.md
  pages:
  - 149
  engine_gaps: &id001
  - dnd2024:spell_mind_blank
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: e318517a47f98589e321b16bdaaf646c853563661ef7e8f5a849add745fe7517
splat_clauses:
- key: source_rules
  text: '# Mind Blank Level 8 Abjuration (Bard, Wizard) Casting Time: Action Range: Touch Components:
    V, S Duration: 24 hours Until the spell ends, one willing creature you touch has Immunity to Psychic
    damage and the Charmed condition. The target is also unaffected by anything that would sense its emotions
    or alignment, read its thoughts, or magically detect its location, and no spell—not even Wish—can
    gather information about the target, observe it remotely, or control its mind.'
  sha256: c77d1b5095d15f7608c853532fbca0cc3223a764ca511924987586c1de5a71ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mind Blank

<!-- SRD page 149 -->

Level 8 Abjuration (Bard, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: 24 hours\
Until the spell ends, one willing creature you touch\
has Immunity to Psychic damage and the Charmed\
condition. The target is also unaffected by anything\
that would sense its emotions or alignment, read\
its thoughts, or magically detect its location, and\
no spell—not even Wish—can gather information\
about the target, observe it remotely, or control its\
mind.\

Original layout: [[../../srd-5.2.1/pages/149|page 149]].
