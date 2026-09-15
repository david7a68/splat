---
splat_schema: 1
splat_id: 9744acff-d657-5010-9742-e6daa4d540d9
splat_kind: spell
splat_key: dnd2024:spells/cone_of_cold
splat_name: Cone of Cold
splat_origin:
  key: dnd2024:reference/spell/cone_of_cold
splat_sources:
- title: SRD 5.2.1
  locator: pp. 117
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: evocation
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a small crystal or glass cone
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 4591ff1820efbc1fffd4166180cf138d6e297b34b02e64cb146728d22ab2eec1
  source_path: builtin/srd-5.2.1/spell/cone_of_cold.md
  pages:
  - 117
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Cone of Cold Level 5 Evocation (Druid, Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a small crystal or glass cone) Duration: Instantaneous You unleash a blast of cold air. Each
    creature in a 60-foot Cone originating from you makes a Constitution saving throw, taking 8d8 Cold
    damage on a failed save or half as much damage on a successful one. A creature killed by this spell
    becomes a frozen statue until it thaws. Using a Higher-Level Spell Slot. The damage increases by 1d8
    for each spell slot level above 5.'
  sha256: bb49504e1e7a793858600dd41cfe9c37c7b45269c0b5015800ff0866ace41353
  status: needs_semantic_conversion
---

# Cone of Cold

<!-- SRD page 117 -->

Level 5 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a small crystal or glass cone)\
Duration: Instantaneous\
You unleash a blast of cold air. Each creature in\
a 60-foot Cone originating from you makes a\
Constitution saving throw, taking 8d8 Cold damage\
on a failed save or half as much damage on a suc-\
cessful one. A creature killed by this spell becomes\
a frozen statue until it thaws.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/117|page 117]].
