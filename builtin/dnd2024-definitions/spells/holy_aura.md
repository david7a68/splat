---
splat_schema: 1
splat_id: 94b66205-d1c9-5a86-8d01-2c6c7c49c9e9
splat_kind: spell
splat_key: dnd2024:spells/holy_aura
splat_name: Holy Aura
splat_origin:
  key: dnd2024:reference/spell/holy_aura
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: abjuration
  classes:
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a reliquary worth 1,000+ GP
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: b89c7efd695e92a26f128c4284929fd81a6f705993bd2fe51fbadcb50796cbd4
  source_path: builtin/srd-5.2.1/spell/holy_aura.md
  pages:
  - 141
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Holy Aura Level 8 Abjuration (Cleric) Casting Time: Action Range: Self Components: V, S, M
    (a reliquary worth 1,000+ GP) Duration: Concentration, up to 1 minute For the duration, you emit an
    aura in a 30-foot Emanation. While in the aura, creatures of your choice have Advantage on all saving
    throws, and other creatures have Disadvantage on attack rolls against them. In addition, when a Fiend
    or an Undead hits an affected creature with a melee attack roll, the attacker must succeed on a Constitution
    saving throw or have the Blinded condition until the end of its next turn.'
  sha256: c0abe93127a87d2067cb97b48cd97b715416a9defbf895fc60b1ee89e2416117
  status: needs_semantic_conversion
---

# Holy Aura

<!-- SRD page 141 -->

Level 8 Abjuration (Cleric)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a reliquary worth 1,000+ GP)\
Duration: Concentration, up to 1 minute\
For the duration, you emit an aura in a 30-foot Em-\
anation. While in the aura, creatures of your choice\
have Advantage on all saving throws, and other\
creatures have Disadvantage on attack rolls against\
them. In addition, when a Fiend or an Undead hits\
an affected creature with a melee attack roll, the at-\
tacker must succeed on a Constitution saving throw\
or have the Blinded condition until the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].
