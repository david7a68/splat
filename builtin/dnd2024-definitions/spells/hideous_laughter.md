---
splat_schema: 1
splat_id: 0b5651a1-6a64-5e9f-89f9-f603fd79b44a
splat_kind: spell
splat_key: dnd2024:spells/hideous_laughter
splat_name: Hideous Laughter
splat_origin:
  key: dnd2024:reference/spell/hideous_laughter
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a tart and a feather
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: cfbeb7f64f69ee5d32911a12feba3f27ef1c213f0436eb10492f7b9fa66ccbdc
  source_path: builtin/srd-5.2.1/spell/hideous_laughter.md
  pages:
  - 140
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Hideous Laughter Level 1 Enchantment (Bard, Warlock, Wizard) Casting Time: Action Range: 30
    feet Components: V, S, M (a tart and a feather) Duration: Concentration, up to 1 minute One creature
    of your choice that you can see within range makes a Wisdom saving throw. On a failed save, it has
    the Prone and Incapacitated conditions for the duration. During that time, it laughs uncontrollably
    if it’s capable of laughter, and it can’t end the Prone condition on itself. At the end of each of
    its turns and each time it takes damage, it makes another Wisdom saving throw. The target has Advantage
    on the save if the save is triggered by damage. On a successful save, the spell ends. Using a Higher-Level
    Spell Slot. You can target one additional creature for each spell slot level above 1.'
  sha256: 266f101ed63874359a6e48a04a88ff1275292d3047edcafa86965089531f0248
  status: needs_semantic_conversion
---

# Hideous Laughter

<!-- SRD page 140 -->

Level 1 Enchantment (Bard, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a tart and a feather)\
Duration: Concentration, up to 1 minute\
One creature of your choice that you can see within\
range makes a Wisdom saving throw. On a failed\
save, it has the Prone and Incapacitated conditions\
for the duration. During that time, it laughs uncon-\
trollably if it’s capable of laughter, and it can’t end\
the Prone condition on itself.\
At the end of each of its turns and each time it\
takes damage, it makes another Wisdom saving\
throw. The target has Advantage on the save if the\
save is triggered by damage. On a successful save,\
the spell ends.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].
