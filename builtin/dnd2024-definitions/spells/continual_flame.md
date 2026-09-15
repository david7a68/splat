---
splat_schema: 1
splat_id: 9c0b220a-a3f4-5c12-b884-dc929fe32d70
splat_kind: spell
splat_key: dnd2024:spells/continual_flame
splat_name: Continual Flame
splat_origin:
  key: dnd2024:reference/spell/continual_flame
splat_sources:
- title: SRD 5.2.1
  locator: pp. 119
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - cleric
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: ruby dust worth 50+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 5aed7e8acec1a665e81803324d2809a5b6ec440e57ce0aa339f1c344e07f6cac
  source_path: builtin/srd-5.2.1/spell/continual_flame.md
  pages:
  - 119
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Continual Flame Level 2 Evocation (Cleric, Druid, Wizard) Casting Time: Action Range: Touch
    Components: V, S, M (ruby dust worth 50+ GP, which the spell consumes) Duration: Until dispelled A
    flame springs from an object that you touch. The effect casts Bright Light in a 20-foot radius and
    Dim Light for an additional 20 feet. It looks like a regular flame, but it creates no heat and consumes
    no fuel. The flame can be covered or hidden but not smothered or quenched.'
  sha256: 83bdd8afb38eae4cbd590c204368d34c5fa2c0dd33f23f53c7e64bbf512b828b
  status: needs_semantic_conversion
---

# Continual Flame

<!-- SRD page 119 -->

Level 2 Evocation (Cleric, Druid, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (ruby dust worth 50+ GP, which\
the spell consumes)\
Duration: Until dispelled\
A flame springs from an object that you touch. The\
effect casts Bright Light in a 20-foot radius and Dim\
Light for an additional 20 feet. It looks like a regular\
flame, but it creates no heat and consumes no fuel.\
The flame can be covered or hidden but not smoth-\
ered or quenched.\

Original layout: [[../../srd-5.2.1/pages/119|page 119]].
