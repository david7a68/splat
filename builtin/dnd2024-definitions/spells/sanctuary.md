---
splat_schema: 1
splat_id: 3214d5d0-ab3f-5d2f-a4dc-27dc268a48b2
splat_kind: spell
splat_key: dnd2024:spells/sanctuary
splat_name: Sanctuary
splat_origin:
  key: dnd2024:reference/spell/sanctuary
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 minute
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 30
  material:
    description: a shard of glass from a mirror
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/sanctuary
  implementation: dnd2024:spell_sanctuary
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 60
    target:
      kind: creature
      range_feet: 30
    protective_trigger:
      event: target.selected
      warded_creature_targeted_by:
      - attack_roll
      - damaging_spell
      attacker_kind: creature
      exclude: area_of_effect
    attacker_save:
      ability: wisdom
      dc: spell_save_dc
      success: original_target_allowed
      failure:
        chooser: attacker
        options:
        - choose_new_legal_target
        - lose_attack_or_spell
    end_early_events:
    - warded_creature.made_attack_roll
    - warded_creature.cast_spell
    - warded_creature.dealt_damage
splat_contracts:
- id: dnd2024:spell_sanctuary
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: participant
      required: true
    retarget_choice:
      required: false
      type: choice
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - target.selected
  - warded_creature.cast_spell
  - warded_creature.dealt_damage
  - warded_creature.made_attack_roll
  capabilities:
  - spellcasting
  - effects
  - attacks
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 4bbd7faba48ae89b8bd4f88381442f0e76546239a8b9e8ae598b3035e736f12a
  source_path: builtin/srd-5.2.1/spell/sanctuary.md
  pages:
  - 159
  engine_gaps: &id001
  - dnd2024:spell_sanctuary
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: d7eb7d8d255a4141f0232ef0b92154841d3224b03978c8d1e7334ad77dafdafb
splat_clauses:
- key: source_rules
  text: '# Sanctuary Level 1 Abjuration (Cleric) Casting Time: Bonus Action Range: 30 feet Components:
    V, S, M (a shard of glass from a mirror) Duration: 1 minute You ward a creature within range. Until
    the spell ends, any creature who targets the warded creature with an attack roll or a damaging spell
    must succeed on a Wisdom saving throw or either choose a new target or lose the attack or spell. This
    spell doesn’t protect the warded creature from areas of effect. The spell ends if the warded creature
    makes an attack roll, casts a spell, or deals damage.'
  sha256: 1c5395b237e6d218737432ddc6c213e18db2746c6b95db1502601b9f31776318
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sanctuary

<!-- SRD page 159 -->

Level 1 Abjuration (Cleric)\
Casting Time: Bonus Action\
Range: 30 feet\
Components: V, S, M (a shard of glass from a mirror)\
Duration: 1 minute\
You ward a creature within range. Until the spell\
ends, any creature who targets the warded crea-\
ture with an attack roll or a damaging spell must\
succeed on a Wisdom saving throw or either choose\
a new target or lose the attack or spell. This spell\
doesn’t protect the warded creature from areas of\
effect. The spell ends if the warded creature makes\
an attack roll, casts a spell, or deals damage.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
