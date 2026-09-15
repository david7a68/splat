---
splat_schema: 1
splat_id: c6f8f1d2-2402-5e8f-b38a-f42e141b139f
splat_kind: spell
splat_key: dnd2024:spells/nondetection
splat_name: Nondetection
splat_origin:
  key: dnd2024:reference/spell/nondetection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - bard
  - ranger
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
    description: a pinch of diamond dust worth 25+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 562a62b765681e72990811507ba5c84630adba9d7e20051749d94e353c52158d
  source_path: builtin/srd-5.2.1/spell/nondetection.md
  pages:
  - 151
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Nondetection Level 3 Abjuration (Bard, Ranger, Wizard) Casting Time: Action Range: Touch Components:
    V, S, M (a pinch of diamond dust worth 25+ GP, which the spell consumes) Duration: 8 hours For the
    duration, you hide a target that you touch from Divination spells. The target can be a willing creature,
    or it can be a place or an object no larger than 10 feet in any dimension. The target can’t be targeted
    by any Divination spell or perceived through magical scrying sensors.'
  sha256: 5f8ab511328832d510888a07f1cef2b7c695d4e0540fc4c67a35c8b8c219f5ce
  status: needs_semantic_conversion
---

# Nondetection

<!-- SRD page 151 -->

Level 3 Abjuration (Bard, Ranger, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pinch of diamond dust worth\
25+ GP, which the spell consumes)\
Duration: 8 hours\
For the duration, you hide a target that you touch\
from Divination spells. The target can be a willing\
creature, or it can be a place or an object no larger\
than 10 feet in any dimension. The target can’t\
be targeted by any Divination spell or perceived\
through magical scrying sensors.\

Original layout: [[../../srd-5.2.1/pages/151|page 151]].
