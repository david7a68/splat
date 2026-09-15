---
splat_schema: 1
splat_id: 4ff38385-cf51-5684-abd1-6ed1274c5916
splat_kind: spell
splat_key: dnd2024:spells/cure_wounds
splat_name: Cure Wounds
splat_origin:
  key: dnd2024:reference/spell/cure_wounds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 121
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/cure_wounds
  implementation: dnd2024:spell_cure_wounds
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      touch: true
    healing:
      dice:
        count: 2
        sides: 8
        add:
          binding: spellcasting_modifier
    scaling:
      per_slot_above: 1
      healing_dice: 2
      sides: 8
splat_contracts:
- id: dnd2024:spell_cure_wounds
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
  - healing
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - healing
splat_conversion:
  status: specified
  category: spell
  source_sha256: fa510b85cff9347ef844da3f8aa575308bd750368b6ce9a501185da8fd5e29b0
  source_path: builtin/srd-5.2.1/spell/cure_wounds.md
  pages:
  - 121
  engine_gaps: &id001
  - dnd2024:spell_cure_wounds
  conversion_gaps: []
  execution_profiles:
  - dnd2024:spells/cure_wounds
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 5c071ab9ef50faef6c107fdef36f8276813b602ea67864f9dc2d764646bf6b0f
splat_clauses:
- key: source_rules
  text: '# Cure Wounds Level 1 Abjuration (Bard, Cleric, Druid, Paladin, Ranger) Casting Time: Action
    Range: Touch Components: V, S Duration: Instantaneous A creature you touch regains a number of Hit
    Points equal to 2d8 plus your spellcasting ability modifier. Using a Higher-Level Spell Slot. The
    healing increases by 2d8 for each spell slot level above 1.'
  sha256: a05543213c0a487fe621054bfe72470aba9cdda6d2fe2bbcbf8556631baac73e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cure Wounds

<!-- SRD page 121 -->

Level 1 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
A creature you touch regains a number of Hit Points\
equal to 2d8 plus your spellcasting ability modifier.\
Using a Higher-Level Spell Slot. The healing in-\
creases by 2d8 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/121|page 121]].
