---
splat_schema: 1
splat_id: c1fe9cc5-f0f5-582c-afee-9066245175eb
splat_kind: spell
splat_key: dnd2024:spells/time_stop
splat_name: Time Stop
splat_origin:
  key: dnd2024:reference/spell/time_stop
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: transmutation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_conversion:
  status: partial
  category: spell
  source_sha256: 830520630a1b2e32539222aa38217784b56d709617e279985f2060aa70b0f4fb
  source_path: builtin/srd-5.2.1/spell/time_stop.md
  pages:
  - 169
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Time Stop Level 9 Transmutation (Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V Duration: Instantaneous You briefly stop the flow of time for everyone but yourself. No time passes
    for other creatures, while you take 1d4 + 1 turns in a row, during which you can use actions and move
    as normal. This spell ends if one of the actions you use during this period, or any effects that you
    create during it, affects a creature other than you or an object being worn or carried by someone
    other than you. In addition, the spell ends if you move to a place more than 1,000 feet from the location
    where you cast it.'
  sha256: 069593c83d7325c9da8b30a2902931828239bcc00bd959c7a3c47308ac3fc09d
  status: needs_semantic_conversion
---

# Time Stop

<!-- SRD page 169 -->

Level 9 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Instantaneous\
You briefly stop the flow of time for everyone but\
yourself. No time passes for other creatures, while\
you take 1d4 + 1 turns in a row, during which you\
can use actions and move as normal.\
This spell ends if one of the actions you use during\
this period, or any effects that you create during it,\
affects a creature other than you or an object being\
worn or carried by someone other than you. In addi-\
tion, the spell ends if you move to a place more than\
1,000 feet from the location where you cast it.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]].
