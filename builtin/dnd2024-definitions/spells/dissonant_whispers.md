---
splat_schema: 1
splat_id: 92291c5c-5ac9-5558-b0d0-d4b9c99f383c
splat_kind: spell
splat_key: dnd2024:spells/dissonant_whispers
splat_name: Dissonant Whispers
splat_origin:
  key: dnd2024:reference/spell/dissonant_whispers
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/dissonant_whispers
  implementation: dnd2024:spell_dissonant_whispers
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
          dice: 3d6
          type: psychic
        movement:
          economy: reaction_if_available
          timing: immediate
          direction: away_from_caster
          distance: maximum_possible
          route: safest
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
        movement: false
    slot_scaling:
      above: 1
      damage_per_level: 1d6
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_dissonant_whispers
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
splat_conversion:
  status: specified
  category: spell
  source_sha256: 78d7c1103c52de2368817f3393e62a68eccfc4e743f80bfffd6044e3ec20027f
  source_path: builtin/srd-5.2.1/spell/dissonant_whispers.md
  pages:
  - 124
  engine_gaps: &id001
  - dnd2024:spell_dissonant_whispers
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: a528e54518aa2570548fedc6cc85ee2416394304b72c369c11e689efa63d5936
splat_clauses:
- key: source_rules
  text: '# Dissonant Whispers Level 1 Enchantment (Bard) Casting Time: Action Range: 60 feet Components:
    V Duration: Instantaneous One creature of your choice that you can see within range hears a discordant
    melody in its mind. The target makes a Wisdom saving throw. On a failed save, it takes 3d6 Psychic
    damage and must immediately use its Reaction, if available, to move as far away from you as it can,
    using the safest route. On a successful save, the target takes half as much damage only. Using a Higher-Level
    Spell Slot. The damage increases by 1d6 for each spell slot level above 1.'
  sha256: fc60a76c5cc95cf729b82095f0b475b3a90fa97a403e041d5bc00f61d08bd6ad
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dissonant Whispers

<!-- SRD page 124 -->

Level 1 Enchantment (Bard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous\
One creature of your choice that you can see within\
range hears a discordant melody in its mind. The\
target makes a Wisdom saving throw. On a failed\
save, it takes 3d6 Psychic damage and must imme-\
diately use its Reaction, if available, to move as far\
away from you as it can, using the safest route. On a\
successful save, the target takes half as much dam-\
age only.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/124|page 124]].
