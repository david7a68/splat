---
splat_schema: 1
splat_id: 397d1e99-e0d9-5cd4-bb11-7a29716e088d
splat_kind: spell
splat_key: dnd2024:spells/mage_armor
splat_name: Mage Armor
splat_origin:
  key: dnd2024:reference/spell/mage_armor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a piece of cured leather
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: dc10563cde2d18e93af171ae8ab8f591a57e41de7fea35447f871e1593f1c9ea
  source_path: builtin/srd-5.2.1/spell/mage_armor.md
  pages:
  - 145
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Mage Armor Level 1 Abjuration (Sorcerer, Wizard) Casting Time: Action Range: Touch Components:
    V, S, M (a piece of cured leather) Duration: 8 hours You touch a willing creature who isn’t wearing
    armor. Until the spell ends, the target’s base AC becomes 13 plus its Dexterity modifier. The spell
    ends early if the target dons armor.'
  sha256: 502c918b4b29eeac992f83a1d8076ee4bb44897bedce698016a26414b81bd587
  status: needs_semantic_conversion
---

# Mage Armor

<!-- SRD page 145 -->

Level 1 Abjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a piece of cured leather)\
Duration: 8 hours\
You touch a willing creature who isn’t wearing\
armor. Until the spell ends, the target’s base AC be-\
comes 13 plus its Dexterity modifier. The spell ends\
early if the target dons armor.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]].
