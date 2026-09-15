---
splat_schema: 1
splat_id: 2f38c561-bd58-5c2f-b824-080a87441002
splat_kind: spell
splat_key: dnd2024:spells/charm_monster
splat_name: Charm Monster
splat_origin:
  key: dnd2024:reference/spell/charm_monster
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114, 115
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: enchantment
  classes:
  - bard
  - druid
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_conversion:
  status: partial
  category: spell
  source_sha256: 760430c5d10b7ab485ee8e6f2b828f52ea327d9016c2185fb416b783c9557474
  source_path: builtin/srd-5.2.1/spell/charm_monster.md
  pages:
  - 114
  - 115
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Charm Monster Level 4 Enchantment (Bard, Druid, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 30 feet Components: V, S Duration: 1 hour One creature you can see within range makes a Wisdom
    saving throw. It does so with Advantage if you or your allies are fighting it. On a failed save, the
    target has the Charmed condition until the spell ends or until you or your allies damage it. The Charmed
    creature is Friendly to you. When the spell ends, the target knows it was Charmed by you. Using a
    Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 4.'
  sha256: c130f1601713c58b50081ca1ca0efe1c44e0b11c453264ec7e757c0d43657669
  status: needs_semantic_conversion
---

# Charm Monster

<!-- SRD page 114 -->

Level 4 Enchantment (Bard, Druid, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 1 hour\

<!-- SRD page 115 -->

One creature you can see within range makes a\
Wisdom saving throw. It does so with Advantage\
if you or your allies are fighting it. On a failed save,\
the target has the Charmed condition until the\
spell ends or until you or your allies damage it. The\
Charmed creature is Friendly to you. When the spell\
ends, the target knows it was Charmed by you.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/114|page 114]], [[../../srd-5.2.1/pages/115|page 115]].
