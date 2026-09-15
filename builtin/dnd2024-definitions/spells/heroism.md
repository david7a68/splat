---
splat_schema: 1
splat_id: bbdf55ef-a5be-5853-b290-8f40317602d9
splat_kind: spell
splat_key: dnd2024:spells/heroism
splat_name: Heroism
splat_origin:
  key: dnd2024:reference/spell/heroism
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - paladin
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/heroism
  implementation: dnd2024:spell_heroism
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      willing: true
      touch: true
    condition_immunity:
    - frightened
    temporary_hp:
      event: turn.start
      subject: target
      amount:
        binding: spellcasting_modifier
    scaling:
      per_slot_above: 1
      additional_targets: 1
splat_contracts:
- id: dnd2024:spell_heroism
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spellcasting_modifier:
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
  - condition_immunity
  - temporary_hp
  events:
  - spell.cast_completed
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - healing
  - conditions
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 387b7a9380c580d74d0bf86d529201660cf6ec71781e6b61b42fa90f35fd465d
  source_path: builtin/srd-5.2.1/spell/heroism.md
  pages:
  - 140
  engine_gaps: &id001
  - dnd2024:spell_heroism
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 58ebb3e5a833177ae8bed6df585f51ea9a55a064ff7161b244a51abd8d40ef85
splat_clauses:
- key: source_rules
  text: '# Heroism Level 1 Enchantment (Bard, Paladin) Casting Time: Action Range: Touch Components: V,
    S Duration: Concentration, up to 1 minute A willing creature you touch is imbued with bravery. Until
    the spell ends, the creature is immune to the Frightened condition and gains Temporary Hit Points
    equal to your spellcasting ability modifier at the start of each of its turns. Using a Higher-Level
    Spell Slot. You can target one additional creature for each spell slot level above 1.'
  sha256: ebef90bd458aec885f9892f2f2b723a61724a615d6ea3ffaae23267ee50033e4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heroism

<!-- SRD page 140 -->

Level 1 Enchantment (Bard, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 minute\
A willing creature you touch is imbued with brav-\
ery. Until the spell ends, the creature is immune to\
the Frightened condition and gains Temporary Hit\
Points equal to your spellcasting ability modifier at\
the start of each of its turns.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].
