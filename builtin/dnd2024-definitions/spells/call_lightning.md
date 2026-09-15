---
splat_schema: 1
splat_id: 129a5122-8826-5942-890f-0418c4dad6e7
splat_kind: spell
splat_key: dnd2024:spells/call_lightning
splat_name: Call Lightning
splat_origin:
  key: dnd2024:reference/spell/call_lightning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_conversion:
  status: partial
  category: spell
  source_sha256: b29b5e740e30daa7fe051974226d84d11deaefd0aa56bb8c68db0fce4862e11d
  source_path: builtin/srd-5.2.1/spell/call_lightning.md
  pages:
  - 114
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Call Lightning Level 3 Conjuration (Druid) Casting Time: Action Range: 120 feet Components:
    V, S Duration: Concentration, up to 10 minutes A storm cloud appears at a point within range that
    you can see above yourself. It takes the shape of a Cylinder that is 10 feet tall with a 60-foot radius.
    When you cast the spell, choose a point you can see under the cloud. A lightning bolt shoots from
    the cloud to that point. Each creature within 5 feet of that point makes a Dexterity saving throw,
    taking 3d10 Lightning damage on a failed save or half as much damage on a successful one. Until the
    spell ends, you can take a Magic action to call down lightning in that way again, targeting the same
    point or a different one. If you’re outdoors in a storm when you cast this spell, the spell gives
    you control over that storm instead of creating a new one. Under such conditions, the spell’s damage
    increases by 1d10. Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot
    level above 3.'
  sha256: e2aabe0f3aca38dd4cb529f698ca9f30885c0cbcb2e49ffb3a9db316c7e97e82
  status: needs_semantic_conversion
---

# Call Lightning

<!-- SRD page 114 -->

Level 3 Conjuration (Druid)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
A storm cloud appears at a point within range that\
you can see above yourself. It takes the shape of a\
Cylinder that is 10 feet tall with a 60-foot radius.\
When you cast the spell, choose a point you can\
see under the cloud. A lightning bolt shoots from the\
cloud to that point. Each creature within 5 feet of\
that point makes a Dexterity saving throw, taking\
3d10 Lightning damage on a failed save or half as\
much damage on a successful one.\
Until the spell ends, you can take a Magic action to\
call down lightning in that way again, targeting the\
same point or a different one.\
If you’re outdoors in a storm when you cast this\
spell, the spell gives you control over that storm in-\
stead of creating a new one. Under such conditions,\
the spell’s damage increases by 1d10.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/114|page 114]].
