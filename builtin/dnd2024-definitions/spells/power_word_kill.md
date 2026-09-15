---
splat_schema: 1
splat_id: 1642deb2-80be-57eb-b4e0-0867bc724528
splat_kind: spell
splat_key: dnd2024:spells/power_word_kill
splat_name: Power Word Kill
splat_origin:
  key: dnd2024:reference/spell/power_word_kill
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
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
splat_rule:
  mechanic: dnd2024:spells/power_word_kill
  implementation: dnd2024:spell_power_word_kill
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    target:
      kind: creature
      visible: true
      range_feet: 60
    branches:
    - when:
        stat: current_hit_points
        comparison: lte
        value: 100
      effect: die
    - when:
        stat: current_hit_points
        comparison: gt
        value: 100
      damage:
        dice: 12d12
        type: psychic
splat_contracts:
- id: dnd2024:spell_power_word_kill
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: participant
      required: true
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: 971da8fa8ea4f8cf289874cb95e289dd31ba7be1c37cb10f49d8a57e3687d7c0
  source_path: builtin/srd-5.2.1/spell/power_word_kill.md
  pages:
  - 154
  engine_gaps: &id001
  - dnd2024:spell_power_word_kill
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 97db6cd300b456e50b70caca3c0f3a08b10cce6d51569d5484c1afe4cc69c39d
splat_clauses:
- key: source_rules
  text: '# Power Word Kill Level 9 Enchantment (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 60 feet Component: V Duration: Instantaneous You compel one creature you can see within range
    to die. If the target has 100 Hit Points or fewer, it dies. Otherwise, it takes 12d12 Psychic damage.'
  sha256: 160da8e36b1af1aab4e2c2e22feaef6e6ae4f741aa80f45e208b7436a21ece3e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Power Word Kill

<!-- SRD page 154 -->

Level 9 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 60 feet\
Component: V\
Duration: Instantaneous\
You compel one creature you can see within range\
to die. If the target has 100 Hit Points or fewer, it\
dies. Otherwise, it takes 12d12 Psychic damage.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]].
