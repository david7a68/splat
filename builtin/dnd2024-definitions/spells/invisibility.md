---
splat_schema: 1
splat_id: 1d9f348b-1d10-5840-b28b-6160733877bb
splat_kind: spell
splat_key: dnd2024:spells/invisibility
splat_name: Invisibility
splat_origin:
  key: dnd2024:reference/spell/invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: an eyelash in gum arabic
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/invisibility
  implementation: dnd2024:spell_invisibility
  parameters:
    targeting:
      kind: creatures
      touch: true
      count:
        base: 1
        per_slot_above: 1
        slot_base: 2
    condition:
      name: invisible
      duration: spell_duration
    early_end:
      timing: immediately_after
      target_events:
      - attack_roll_made
      - damage_dealt
      - spell_cast
      scope: spell
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_invisibility
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
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - damage.applied
  - attack.hit
  - attack.roll_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: a32a1119f9484ccd8b0fb00a7b5925c4cc433855b6fab74bd573afe6068f20e4
  source_path: builtin/srd-5.2.1/spell/invisibility.md
  pages:
  - 143
  engine_gaps: &id001
  - dnd2024:spell_invisibility
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 33cb3e6dccf31450ec3c74038646fe353a4608df6dc2dd6272b42975c31375b5
splat_clauses:
- key: source_rules
  text: '# Invisibility Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (an eyelash in gum arabic) Duration: Concentration, up to 1 hour A creature
    you touch has the Invisible condition until the spell ends. The spell ends early immediately after
    the target makes an attack roll, deals damage, or casts a spell. Using a Higher-Level Spell Slot.
    You can target one additional creature for each spell slot level above 2.'
  sha256: c3dc98f60f2e2e1fa127fd861f17f0af6bb99b08a538cbc853312b8eeef295ed
  status: contract_specified
splat_requires_mechanics: *id001
---

# Invisibility

<!-- SRD page 143 -->

Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (an eyelash in gum arabic)\
Duration: Concentration, up to 1 hour\
A creature you touch has the Invisible condition un-\
til the spell ends. The spell ends early immediately\
after the target makes an attack roll, deals damage,\
or casts a spell.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 2.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
