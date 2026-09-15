---
splat_schema: 1
splat_id: 78179ce1-92b8-56eb-a470-54ca77355e39
splat_kind: spell
splat_key: dnd2024:spells/dancing_lights
splat_name: Dancing Lights
splat_origin:
  key: dnd2024:reference/spell/dancing_lights
splat_sources:
- title: SRD 5.2.1
  locator: pp. 121, 122
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: illusion
  classes:
  - bard
  - sorcerer
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
    feet: 120
  material:
    description: a bit of phosphorus
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 93d71aed860a5d80e428a9cab2f3c2fd374a796255b5f55e4b74251df53e2235
  source_path: builtin/srd-5.2.1/spell/dancing_lights.md
  pages:
  - 121
  - 122
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Dancing Lights Illusion Cantrip (Bard, Sorcerer, Wizard) Casting Time: Action Range: 120 feet
    Components: V, S, M (a bit of phosphorus) Duration: Concentration, up to 1 minute You create up to
    four torch-size lights within range, making them appear as torches, lanterns, or glowing orbs that
    hover for the duration. Alternatively, you combine the four lights into one glowing Medium form that
    is vaguely humanlike. Whichever form you choose, each light sheds Dim Light in a 10- foot radius.
    As a Bonus Action, you can move the lights up to 60 feet to a space within range. A light must be
    within 20 feet of another light created by this spell, and a light vanishes if it exceeds the spell’s
    range.'
  sha256: f157341665845876445268412b2dd858a26bb65a18e25c3c9fc630b6b576f657
  status: needs_semantic_conversion
---

# Dancing Lights

<!-- SRD page 121 -->

Illusion Cantrip (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a bit of phosphorus)\
Duration: Concentration, up to 1 minute\

<!-- SRD page 122 -->

You create up to four torch-size lights within range,\
making them appear as torches, lanterns, or glow-\
ing orbs that hover for the duration. Alternatively,\
you combine the four lights into one glowing Me-\
dium form that is vaguely humanlike. Whichever\
form you choose, each light sheds Dim Light in a 10-\
foot radius.\
As a Bonus Action, you can move the lights up\
to 60 feet to a space within range. A light must be\
within 20 feet of another light created by this spell,\
and a light vanishes if it exceeds the spell’s range.\

Original layout: [[../../srd-5.2.1/pages/121|page 121]], [[../../srd-5.2.1/pages/122|page 122]].
