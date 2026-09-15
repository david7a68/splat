---
splat_schema: 1
splat_id: 64232bd1-5e1c-54a7-9975-06561b611acc
splat_kind: spell
splat_key: dnd2024:spells/power_word_stun
splat_name: Power Word Stun
splat_origin:
  key: dnd2024:reference/spell/power_word_stun
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: enchantment
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_conversion:
  status: partial
  category: spell
  source_sha256: dc714ce9646a96ebfa272623bb36c9156177b89d319fbc29090af85b8accb0eb
  source_path: builtin/srd-5.2.1/spell/power_word_stun.md
  pages:
  - 154
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Power Word Stun Level 8 Enchantment (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 60 feet Components: V Duration: Instantaneous You overwhelm the mind of one creature you can
    see within range. If the target has 150 Hit Points or fewer, it has the Stunned condition. Otherwise,
    its Speed is 0 until the start of your next turn. The Stunned target makes a Constitution saving throw
    at the end of each of its turns, ending the condition on itself on a success.'
  sha256: d9cf04094db65dd70668b28eaad7ea057393ba2b5a66f68b7bfd60c6f7270fbe
  status: needs_semantic_conversion
---

# Power Word Stun

<!-- SRD page 154 -->

Level 8 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous\
You overwhelm the mind of one creature you can\
see within range. If the target has 150 Hit Points or\
fewer, it has the Stunned condition. Otherwise, its\
Speed is 0 until the start of your next turn.\
The Stunned target makes a Constitution saving\
throw at the end of each of its turns, ending the con-\
dition on itself on a success.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]].
