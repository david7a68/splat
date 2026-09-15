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
splat_conversion:
  status: partial
  category: spell
  source_sha256: a32a1119f9484ccd8b0fb00a7b5925c4cc433855b6fab74bd573afe6068f20e4
  source_path: builtin/srd-5.2.1/spell/invisibility.md
  pages:
  - 143
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Invisibility Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (an eyelash in gum arabic) Duration: Concentration, up to 1 hour A creature
    you touch has the Invisible condition until the spell ends. The spell ends early immediately after
    the target makes an attack roll, deals damage, or casts a spell. Using a Higher-Level Spell Slot.
    You can target one additional creature for each spell slot level above 2.'
  sha256: c3dc98f60f2e2e1fa127fd861f17f0af6bb99b08a538cbc853312b8eeef295ed
  status: needs_semantic_conversion
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
