---
splat_schema: 1
splat_id: 7652eed9-eb1e-5d0c-8598-a193dc31cedd
splat_kind: spell
splat_key: dnd2024:spells/mind_spike
splat_name: Mind Spike
splat_origin:
  key: dnd2024:reference/spell/mind_spike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: divination
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/mind_spike
  implementation: dnd2024:spell_mind_spike
  parameters:
    targeting:
      kind: creature
      count: 1
      visible: true
      within_range: true
    save:
      ability: wisdom
      failure:
        damage:
          dice: 3d8
          type: psychic
        tracking:
          duration: spell_duration
          requires: same_plane
          knows_location: always
          target_cannot_hide_from_caster: true
          invisible_benefits_against_caster: none
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    slot_scaling:
      above: 2
      damage_per_level: 1d8
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_mind_spike
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
  - damage_results
  events:
  - spell.cast_completed
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: b2eabc673870583714f3d5c660d42193e77fe6f1f7496305b0982ad598e4c395
  source_path: builtin/srd-5.2.1/spell/mind_spike.md
  pages:
  - 149
  engine_gaps: &id001
  - dnd2024:spell_mind_spike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: f3f718754eb9d8c2b64f21bdef4712074cb46c7f6639b96711b0449064bec639
splat_clauses:
- key: source_rules
  text: '# Mind Spike Level 2 Divination (Sorcerer, Warlock, Wizard) Casting Time: Action Range: 120 feet
    Components: S Duration: Concentration, up to 1 hour You drive a spike of psionic energy into the mind
    of one creature you can see within range. The target makes a Wisdom saving throw, taking 3d8 Psychic
    damage on a failed save or half as much damage on a successful one. On a failed save, you also always
    know the target’s location until the spell ends, but only while the two of you are on the same plane
    of existence. While you have this knowledge, the target can’t become hidden from you, and if it has
    the Invisible condition, it gains no benefit from that condition against you. Using a Higher-Level
    Spell Slot. The damage increases by 1d8 for each spell slot level above 2.'
  sha256: 58d7db9d334fa8c13e71ab6b3eb4287455149dc09f20437e293bc38ac085ce9e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mind Spike

<!-- SRD page 149 -->

Level 2 Divination (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: S\
Duration: Concentration, up to 1 hour\
You drive a spike of psionic energy into the mind of\
one creature you can see within range. The target\
makes a Wisdom saving throw, taking 3d8 Psychic\
damage on a failed save or half as much damage on\
a successful one. On a failed save, you also always\
know the target’s location until the spell ends, but\
only while the two of you are on the same plane\
of existence. While you have this knowledge, the\
target can’t become hidden from you, and if it has\
the Invisible condition, it gains no benefit from that\
condition against you.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/149|page 149]].
