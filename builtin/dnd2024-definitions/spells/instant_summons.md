---
splat_schema: 1
splat_id: 04c83d44-8888-569d-93c9-6d1dd55650c0
splat_kind: spell
splat_key: dnd2024:spells/instant_summons
splat_name: Instant Summons
splat_origin:
  key: dnd2024:reference/spell/instant_summons
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: touch
  material:
    description: a sapphire worth 1,000+ GP
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 06a7da6e6ad2349aa8bae24521abaac6ac10596defc772383d6f5d9d0e3a2b15
  source_path: builtin/srd-5.2.1/spell/instant_summons.md
  pages:
  - 143
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Instant Summons Level 6 Conjuration (Wizard) Casting Time: 1 minute or Ritual Range: Touch
    Components: V, S, M (a sapphire worth 1,000+ GP) Duration: Until dispelled You touch the sapphire
    used in the casting and an object weighing 10 pounds or less whose longest dimension is 6 feet or
    less. The spell leaves an Invisible mark on that object and invisibly inscribes the object’s name
    on the sapphire. Each time you cast this spell, you must use a different sapphire. Thereafter, you
    can take a Magic action to speak the object’s name and crush the sapphire. The object instantly appears
    in your hand regardless of physical or planar distances, and the spell ends. If another creature is
    holding or carrying the object, crushing the sapphire doesn’t transport it, but instead you learn
    who that creature is and where that creature is currently located.'
  sha256: 7adabe9109b33697e13d47880a113214ebc92e019f68599cca362e0a4f5df56a
  status: needs_semantic_conversion
---

# Instant Summons

<!-- SRD page 143 -->

Level 6 Conjuration (Wizard)\
Casting Time: 1 minute or Ritual\
Range: Touch\
Components: V, S, M (a sapphire worth 1,000+ GP)\
Duration: Until dispelled\
You touch the sapphire used in the casting and an\
object weighing 10 pounds or less whose longest\
dimension is 6 feet or less. The spell leaves an Invis-\
ible mark on that object and invisibly inscribes the\
object’s name on the sapphire. Each time you cast\
this spell, you must use a different sapphire.\
Thereafter, you can take a Magic action to speak\
the object’s name and crush the sapphire. The ob-\
ject instantly appears in your hand regardless of\
physical or planar distances, and the spell ends.\
If another creature is holding or carrying the ob-\
ject, crushing the sapphire doesn’t transport it, but\
instead you learn who that creature is and where\
that creature is currently located.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
