---
splat_schema: 1
splat_id: 38054dda-f99e-5a3d-9ae7-627cfeba4b88
splat_kind: spell
splat_key: dnd2024:spells/prayer_of_healing
splat_name: Prayer of Healing
splat_origin:
  key: dnd2024:reference/spell/prayer_of_healing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: 10 minutes
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/prayer_of_healing
  implementation: dnd2024:spell_prayer_of_healing
  parameters:
    targeting:
      kind: creatures
      max_count: 5
      chosen: true
      within_range_for_entire_cast: true
      eligibility: no_benefit_since_last_long_rest
    benefits:
      short_rest: true
      healing:
        dice: 2d8
    repeat_limit:
      scope: each_target
      reset: target_long_rest_finished
    slot_scaling:
      above: 2
      healing_per_level: 1d8
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_prayer_of_healing
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
  - healing_results
  events:
  - spell.cast_completed
  - rest.completed
  capabilities:
  - spellcasting
  - effects
  - rest_lifecycle
  - healing
splat_conversion:
  status: specified
  category: spell
  source_sha256: df26ba38ed855dea4528e5dc7212528e00c0023371e6aa922881a106b20b9fa1
  source_path: builtin/srd-5.2.1/spell/prayer_of_healing.md
  pages:
  - 154
  engine_gaps: &id001
  - dnd2024:spell_prayer_of_healing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 391d30a411fa54423377c91ad9c66db75416ac527c2991d7f924a53ad9f01e1a
splat_clauses:
- key: source_rules
  text: '# Prayer of Healing Level 2 Abjuration (Cleric, Paladin) Casting Time: 10 minutes Range: 30 feet
    Components: V Duration: Instantaneous Up to five creatures of your choice who remain within range
    for the spell’s entire casting gain the benefits of a Short Rest and also regain 2d8 Hit Points. A
    creature can’t be affected by this spell again until that creature finishes a Long Rest. Using a Higher-Level
    Spell Slot. The healing increases by 1d8 for each spell slot level above 2.'
  sha256: 19beaeb9bad958b19808bce88dc68e1684a114b91bfe7594ad4152c6f9c19f2e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prayer of Healing

<!-- SRD page 154 -->

Level 2 Abjuration (Cleric, Paladin)\
Casting Time: 10 minutes\
Range: 30 feet\
Components: V\
Duration: Instantaneous\
Up to five creatures of your choice who remain\
within range for the spell’s entire casting gain the\
benefits of a Short Rest and also regain 2d8 Hit\
Points. A creature can’t be affected by this spell\
again until that creature finishes a Long Rest.\
Using a Higher-Level Spell Slot. The healing in-\
creases by 1d8 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]].
