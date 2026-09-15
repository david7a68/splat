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
splat_conversion:
  status: partial
  category: spell
  source_sha256: fa510b85cff9347ef844da3f8aa575308bd750368b6ce9a501185da8fd5e29b0
  source_path: builtin/srd-5.2.1/spell/cure_wounds.md
  pages:
  - 121
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles:
  - dnd2024:spells/cure_wounds
splat_clauses:
- key: source_rules
  text: '# Cure Wounds Level 1 Abjuration (Bard, Cleric, Druid, Paladin, Ranger) Casting Time: Action
    Range: Touch Components: V, S Duration: Instantaneous A creature you touch regains a number of Hit
    Points equal to 2d8 plus your spellcasting ability modifier. Using a Higher-Level Spell Slot. The
    healing increases by 2d8 for each spell slot level above 1.'
  sha256: a05543213c0a487fe621054bfe72470aba9cdda6d2fe2bbcbf8556631baac73e
  status: needs_semantic_conversion
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
