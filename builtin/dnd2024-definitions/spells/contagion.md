---
splat_schema: 1
splat_id: 19812bef-7e04-560e-ba1d-256201a0789d
splat_kind: spell
splat_key: dnd2024:spells/contagion
splat_name: Contagion
splat_origin:
  key: dnd2024:reference/spell/contagion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 119
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: necromancy
  classes:
  - cleric
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: 7 days
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_conversion:
  status: partial
  category: spell
  source_sha256: e25d4cf1e220de3d44fdda7332768ac2de505a62cd229846aa8863b812d1a8d2
  source_path: builtin/srd-5.2.1/spell/contagion.md
  pages:
  - 119
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Contagion Level 5 Necromancy (Cleric, Druid) Casting Time: Action Range: Touch Component: V,
    S Duration: 7 days Your touch inflicts a magical contagion. The target must succeed on a Constitution
    saving throw or take 11d8 Necrotic damage and have the Poisoned condition. Also, choose one ability
    when you cast the spell. While Poisoned, the target has Disadvantage on saving throws made with the
    chosen ability. The target must repeat the saving throw at the end of each of its turns until it gets
    three successes or failures. If the target succeeds on three of these saves, the spell ends on the
    target. If the target fails three of the saves, the spell lasts for 7 days on it. Whenever the Poisoned
    target receives an effect that would end the Poisoned condition, the target must succeed on a Constitution
    saving throw, or the Poisoned condition doesn’t end on it.'
  sha256: bfda05a1e9fbe6702cd88430fb81c116cdbb155d48682c0e0c7f0cd0f488c9c6
  status: needs_semantic_conversion
---

# Contagion

<!-- SRD page 119 -->

Level 5 Necromancy (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: 7 days\
Your touch inflicts a magical contagion. The target\
must succeed on a Constitution saving throw or\
take 11d8 Necrotic damage and have the Poisoned\
condition. Also, choose one ability when you cast\
the spell. While Poisoned, the target has Disadvan-\
tage on saving throws made with the chosen ability.\
The target must repeat the saving throw at the\
end of each of its turns until it gets three successes\
or failures. If the target succeeds on three of these\
saves, the spell ends on the target. If the target fails\
three of the saves, the spell lasts for 7 days on it.\
Whenever the Poisoned target receives an effect\
that would end the Poisoned condition, the target\
must succeed on a Constitution saving throw, or the\
Poisoned condition doesn’t end on it.\

Original layout: [[../../srd-5.2.1/pages/119|page 119]].
