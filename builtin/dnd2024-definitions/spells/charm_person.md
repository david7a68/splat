---
splat_schema: 1
splat_id: 7e0b22bc-9ed8-50fa-b68d-9d1f0718f6bb
splat_kind: spell
splat_key: dnd2024:spells/charm_person
splat_name: Charm Person
splat_origin:
  key: dnd2024:reference/spell/charm_person
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - druid
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/charm_person
  implementation: dnd2024:spell_charm_person
  parameters:
    target:
      kind: humanoid
      count: 1
      visible_to_caster: true
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: charmed
        attitude_to_caster: friendly
      success:
        effect: none
    save_advantage_if: caster_or_allies_fighting_target
    end_on:
      event: damage.applied
      source:
      - caster
      - caster_allies
      subject: target
    on_end:
      target_knows_charmed_by_caster: true
    scaling:
      per_slot_above: 1
      additional_targets: 1
splat_contracts:
- id: dnd2024:spell_charm_person
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
  - saves
  - charmed
  - attitude
  - knowledge
  events:
  - spell.cast_completed
  - damage.applied
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - conditions
  - knowledge
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2f2fa564d6f32c6e721258183cc27a4ec6d4599cc854bb67d8ec5f010af1ec0a
  source_path: builtin/srd-5.2.1/spell/charm_person.md
  pages:
  - 115
  engine_gaps: &id001
  - dnd2024:spell_charm_person
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: b7e36baf8f93a78eea8b10c83806ddfef0a583e71afed8074d87f4a8c31c9759
splat_clauses:
- key: source_rules
  text: '# Charm Person Level 1 Enchantment (Bard, Druid, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 30 feet Components: V, S Duration: 1 hour One Humanoid you can see within range makes a Wisdom
    saving throw. It does so with Advantage if you or your allies are fighting it. On a failed save, the
    target has the Charmed condition until the spell ends or until you or your allies damage it. The Charmed
    creature is Friendly to you. When the spell ends, the target knows it was Charmed by you. Using a
    Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 1.'
  sha256: 11aa62e8659f64de553c84bf9fb1ca89d59a5a09133ca7b859d33852d566d0ec
  status: contract_specified
splat_requires_mechanics: *id001
---

# Charm Person

<!-- SRD page 115 -->

Level 1 Enchantment (Bard, Druid, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 1 hour\
One Humanoid you can see within range makes a\
Wisdom saving throw. It does so with Advantage\
if you or your allies are fighting it. On a failed save,\
the target has the Charmed condition until the\
spell ends or until you or your allies damage it. The\
Charmed creature is Friendly to you. When the spell\
ends, the target knows it was Charmed by you.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].
