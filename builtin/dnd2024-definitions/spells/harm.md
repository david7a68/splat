---
splat_schema: 1
splat_id: 87cc2289-5542-5c7f-ab89-d16e578280ee
splat_kind: spell
splat_key: dnd2024:spells/harm
splat_name: Harm
splat_origin:
  key: dnd2024:reference/spell/harm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: necromancy
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/harm
  implementation: dnd2024:spell_harm
  parameters:
    targeting:
      kind: creature
      count: 1
      visible: true
      within_range: true
    save:
      ability: constitution
      failure:
        damage:
          dice: 14d6
          type: necrotic
        hp_maximum_reduction:
          amount: necrotic_damage_taken
          minimum_resulting_hp_maximum: 1
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
        hp_maximum_reduction: false
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_harm
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
  capabilities:
  - spellcasting
  - effects
  - damage
  - health_maximum_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 600ba44f78fff99a7b26878968abe2609e9dccc9d4232517a9fbfa96daae350f
  source_path: builtin/srd-5.2.1/spell/harm.md
  pages:
  - 139
  engine_gaps: &id001
  - dnd2024:spell_harm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: e0084f6c7ad31aa64968c8fcfbf3fb4f16b0b7814d4753744f0425c0be9746cf
splat_clauses:
- key: source_rules
  text: '# Harm Level 6 Necromancy (Cleric) Casting Time: Action Range: 60 feet Components: V, S Duration:
    Instantaneous You unleash virulent magic on a creature you can see within range. The target makes
    a Constitution saving throw. On a failed save, it takes 14d6 Necrotic damage, and its Hit Point maximum
    is reduced by an amount equal to the Necrotic damage it took. On a successful save, it takes half
    as much damage only. This spell can’t reduce a target’s Hit Point maximum below 1.'
  sha256: b3a59b2da8a53bdc6af306db1cc92a0d8f5db6ec70de0a1a730e8921aec150d3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Harm

<!-- SRD page 139 -->

Level 6 Necromancy (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
You unleash virulent magic on a creature you can\
see within range. The target makes a Constitu-\
tion saving throw. On a failed save, it takes 14d6\
Necrotic damage, and its Hit Point maximum is re-\
duced by an amount equal to the Necrotic damage\
it took. On a successful save, it takes half as much\
damage only. This spell can’t reduce a target’s Hit\
Point maximum below 1.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]].
