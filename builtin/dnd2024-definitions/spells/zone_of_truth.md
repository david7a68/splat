---
splat_schema: 1
splat_id: 93abf320-7330-5202-b16c-a1d5dfee1f4c
splat_kind: spell
splat_key: dnd2024:spells/zone_of_truth
splat_name: Zone of Truth
splat_origin:
  key: dnd2024:reference/spell/zone_of_truth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 175
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: enchantment
  classes:
  - bard
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  concentration: false
  duration: 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/zone_of_truth
  implementation: dnd2024:spell_zone_of_truth
  parameters:
    area:
      shape: sphere
      radius_feet: 15
      origin: chosen_point_within_range
      duration: spell_duration
    save_triggers:
    - creature_enters_area_first_time_on_turn
    - creature_starts_turn_in_area
    save:
      ability: charisma
      dc:
        binding: save_dc
      failure:
        cannot_speak_deliberate_lie: true
        restriction_only_in_radius: true
        aware_of_spell: true
        may_refuse_answer: true
        may_evade_truthfully: true
      success:
        effect: none
    caster_knows_each_save_outcome: true
splat_contracts:
- id: dnd2024:spell_zone_of_truth
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
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - knowledge
  events:
  - spell.cast_completed
  - turn.start
  - movement.completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - spatial_relationships
  - saving_throws
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: d3ce95cdeb67f455db6528efd98013c00d04a8660dd940de03aa5a9a5fa9c3c9
  source_path: builtin/srd-5.2.1/spell/zone_of_truth.md
  pages:
  - 175
  engine_gaps: &id001
  - dnd2024:spell_zone_of_truth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 49d741b4401d57230a61cbf9223355891508c4966a8e928fece3153c65071cdf
splat_clauses:
- key: source_rules
  text: '# Zone of Truth Level 2 Enchantment (Bard, Cleric, Paladin) Casting Time: Action Range: 60 feet
    Components: V, S Duration: 10 minutes You create a magical zone that guards against deception in a
    15-foot-radius Sphere centered on a point within range. Until the spell ends, a creature that enters
    the spell’s area for the first time on a turn or starts its turn there makes a Charisma saving throw.
    On a failed save, a creature can’t speak a deliberate lie while in the radius. You know whether a
    creature succeeds or fails on this save. An affected creature is aware of the spell and can avoid
    answering questions to which it would normally respond with a lie. Such a creature can be evasive
    yet must be truthful.'
  sha256: ce26cfa72bb6944fd540d0516eafad12d77fdcc779a898e2f8b3362a4ea3de5f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Zone of Truth

<!-- SRD page 175 -->

Level 2 Enchantment (Bard, Cleric, Paladin)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: 10 minutes\
You create a magical zone that guards against de-\
ception in a 15-foot-radius Sphere centered on a\
point within range. Until the spell ends, a creature\
that enters the spell’s area for the first time on a\
turn or starts its turn there makes a Charisma sav-\
ing throw. On a failed save, a creature can’t speak a\
deliberate lie while in the radius. You know whether\
a creature succeeds or fails on this save.\
An affected creature is aware of the spell and\
can avoid answering questions to which it would\
normally respond with a lie. Such a creature can be\
evasive yet must be truthful.\

Original layout: [[../../srd-5.2.1/pages/175|page 175]].
