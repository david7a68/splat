---
splat_schema: 1
splat_id: 5340380a-57af-5094-9f38-02fe5c88b00b
splat_kind: spell
splat_key: dnd2024:spells/silence
splat_name: Silence
splat_origin:
  key: dnd2024:reference/spell/silence
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - bard
  - cleric
  - ranger
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 120
splat_conversion:
  status: partial
  category: spell
  source_sha256: c6f702b46f18f4fdd8ed93a7926a8c18db1fc02666b64f12c74d52207caa68b4
  source_path: builtin/srd-5.2.1/spell/silence.md
  pages:
  - 162
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Silence Level 2 Illusion (Bard, Cleric, Ranger) Casting Time: Action or Ritual Range: 120 feet
    Components: V, S Duration: Concentration, up to 10 minutes For the duration, no sound can be created
    within or pass through a 20-foot-radius Sphere centered on a point you choose within range. Any creature
    or object entirely inside the Sphere has Immunity to Thunder damage, and creatures have the Deafened
    condition while entirely inside it. Casting a spell that includes a Verbal component is impossible
    there.'
  sha256: bf01103eac4bf9caedb4ff3ebb74a88eb098a93b52a965cbc6eda144116f7c40
  status: needs_semantic_conversion
---

# Silence

<!-- SRD page 162 -->

Level 2 Illusion (Bard, Cleric, Ranger)\
Casting Time: Action or Ritual\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
For the duration, no sound can be created within\
or pass through a 20-foot-radius Sphere centered\
on a point you choose within range. Any creature\
or object entirely inside the Sphere has Immunity\
to Thunder damage, and creatures have the Deaf-\
ened condition while entirely inside it. Casting a\
spell that includes a Verbal component is impossible\
there.\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].
